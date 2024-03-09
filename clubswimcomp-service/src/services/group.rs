use std::collections::HashMap;

use anyhow::{Context, Result};
use clubswimcomp_types::model;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    db, infra,
    services::{score::ScoreService, ParticipantService},
};

use super::ServiceRepositoryError;

pub struct GroupService {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
    group_repo: db::groups::Repository,
    typst_compiler: infra::typst_compiler::TypstCompiler,
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
        typst_compiler: infra::typst_compiler::TypstCompiler,
    ) -> Self {
        Self {
            participant_repo,
            registration_repo,
            competition_repo,
            group_repo,
            typst_compiler,
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
        let score_service = ScoreService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
            self.group_repo.clone(),
            self.typst_compiler.clone(),
        );

        tracing::debug!("Ensuring group actually exists");
        let group = self
            .group_repo
            .group_by_id(group_id)
            .await
            .context("Failed to fetch group from repository")?
            .map(model::Group::from)
            .ok_or(GroupResultError::GroupDoesNotExist)?;

        tracing::debug!("Loading participants in the group");
        let mut participants = self
            .participant_repo
            .list_participants_in_group(group.id)
            .await
            .context("Failed to fetch participants for group from repository")?
            .into_iter()
            .map(|p| (p.id, model::Participant::from(p)))
            .collect::<HashMap<_, _>>();

        tracing::debug!("Fetching participant scores");
        let participant_scores = score_service
            .participants_fina_points()
            .await
            .context("Failed to fetch participants FINA points")?
            .into_iter()
            .filter(|r| participants.contains_key(&r.0))
            .collect::<HashMap<_, _>>();

        if participant_scores.len() == participants.len() {
            return Err(GroupResultError::RepositoryError(anyhow::anyhow!(
                "Participants scores and participants do not match up in length"
            )));
        }

        let mut scores = Vec::with_capacity(participants.len());
        for (participant_id, (results_missing, own_fina_points)) in participant_scores.iter() {
            let participants_with_more_points = participant_scores
                .values()
                .filter(|ps| ps.1 > *own_fina_points)
                .count();

            // 0 with more means 1st rank
            let rank = participants_with_more_points as u32 + 1;

            scores.push(model::GroupScore {
                participant: participants.remove(participant_id).unwrap(),
                fina_points: *own_fina_points,
                rank,
            });
        }

        Ok(model::GroupDetails { group, scores })
    }
}
