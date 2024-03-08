use std::collections::HashMap;

use anyhow::{Context, Result};
use clubswimcomp_types::model;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{db, services::ParticipantService};

use super::ServiceRepositoryError;

pub struct GroupService {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
    group_repo: db::groups::Repository,
}

#[derive(Debug, Error)]
pub enum GroupResultError {
    #[error("The group does not exist")]
    GroupDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

impl GroupService {
    pub fn new(
        participant_repo: db::participants::Repository,
        registration_repo: db::registrations::Repository,
        competition_repo: db::competitions::Repository,
        group_repo: db::groups::Repository,
    ) -> Self {
        Self {
            participant_repo,
            registration_repo,
            competition_repo,
            group_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn list_groups(&self) -> Result<Vec<model::Group>, ServiceRepositoryError> {
        Ok(self
            .group_repo
            .all_groups()
            .await
            .context("Failed to fetch groups from repository")?
            .into_iter()
            .map(model::Group::from)
            .collect())
    }

    #[instrument(skip(self))]
    pub async fn add_group(&self, group_name: String) -> Result<Uuid, ServiceRepositoryError> {
        Ok(self
            .group_repo
            .create_group(group_name)
            .await
            .context("Failed to create group in repository")?)
    }

    #[instrument(skip(self))]
    pub async fn group_details(
        &self,
        group_id: Uuid,
    ) -> Result<model::GroupDetails, GroupResultError> {
        tracing::debug!("Ensuring group actually exists");
        let group = self
            .group_repo
            .group_by_id(group_id)
            .await
            .context("Failed to fetch group from repository")?
            .map(model::Group::from)
            .ok_or(GroupResultError::GroupDoesNotExist)?;

        tracing::debug!("Loading participants in the group");
        let db_participants = self
            .participant_repo
            .list_participants_in_group(group.id)
            .await
            .context("Failed to fetch participants for group from repository")?;

        tracing::debug!("Fetching participant details");
        let participant_service = ParticipantService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
            self.group_repo.clone(),
        );

        let mut participant_details = Vec::with_capacity(db_participants.len());
        let mut participant_points: HashMap<Uuid, u32> = HashMap::new();
        let mut registration_results_missing = Vec::new();

        for db_participant in db_participants.into_iter() {
            let pd = participant_service
                .participant_details(db_participant.id)
                .await
                .context("Could not fetch participant even though a reference exists")?;

            let (with_result, without_result): (Vec<_>, Vec<_>) = pd
                .registrations
                .clone()
                .into_iter()
                .partition(|r| r.result.is_some());

            let without_result =
                without_result
                    .into_iter()
                    .map(|registration| model::RegistrationDetails {
                        id: registration.id,
                        participant: pd.participant.clone(),
                        competition: registration.competition,
                        result: registration.result,
                    });
            registration_results_missing.extend(without_result);

            let total_points = with_result
                .iter()
                .map(|r| r.result.as_ref().unwrap())
                .map(|r| if !r.disqualified { r.fina_points } else { 0 })
                .sum();

            participant_points.insert(pd.participant.id, total_points);
            participant_details.push(pd);
        }

        let mut scores = Vec::with_capacity(participant_details.len());
        for participant in participant_details.into_iter() {
            let own_points = *participant_points
                .get(&participant.participant.id)
                .expect("total points to be calculated for every participant");

            let participants_with_higher_score = participant_points
                .iter()
                .filter(|p| *p.1 > own_points)
                .count();
            let rank = participants_with_higher_score as u32 + 1;

            scores.push(model::GroupScore {
                participant: participant.participant,
                fina_points: own_points,
                rank,
            });
        }

        Ok(model::GroupDetails {
            group,
            registration_results_missing,
            scores,
        })
    }
}
