use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::Datelike;
use clubswimcomp_types::model;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{db, infra, services::ParticipantService};

use super::{CompetitionService, ServiceRepositoryError};

pub struct ScoreService {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
    group_repo: db::groups::Repository,
    typst_compiler: infra::typst_compiler::TypstCompiler,
}

#[derive(Debug, Error)]
pub enum ParticipantFinaPointsError {
    #[error("The participant does not exist")]
    ParticipantDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum CompetitionScoreboardError {
    #[error("The competition does not exist")]
    CompetitionDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ParticipantScoreboardError {
    #[error("The participant does not exist")]
    ParticipantDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ParticipantCertificateError {
    #[error("The participant does not exist")]
    ParticipantDoesNotExist,

    #[error("The participant does not exist")]
    PdfGenerationFailed(anyhow::Error),

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum GroupScoreboardError {
    #[error("The group does not exist")]
    GroupDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

impl From<super::competition::CompetitionDetailsError> for CompetitionScoreboardError {
    fn from(err: super::competition::CompetitionDetailsError) -> Self {
        use super::competition::CompetitionDetailsError::*;
        match err {
            CompetitionDoesNotExist => Self::CompetitionDoesNotExist,
            RepositoryError(e) => Self::RepositoryError(e),
        }
    }
}

impl From<super::participant::ParticipantDetailsError> for ParticipantScoreboardError {
    fn from(err: super::participant::ParticipantDetailsError) -> Self {
        use super::participant::ParticipantDetailsError::*;
        match err {
            ParticipantDoesNotExist => Self::ParticipantDoesNotExist,
            RepositoryError(e) => Self::RepositoryError(e),
        }
    }
}

impl From<ParticipantScoreboardError> for ParticipantCertificateError {
    fn from(err: ParticipantScoreboardError) -> Self {
        use ParticipantScoreboardError::*;
        match err {
            ParticipantDoesNotExist => Self::ParticipantDoesNotExist,
            RepositoryError(e) => Self::RepositoryError(e),
        }
    }
}

impl ScoreService {
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

    fn competition_service(&self) -> CompetitionService {
        CompetitionService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
        )
    }

    fn participant_service(&self) -> ParticipantService {
        ParticipantService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
            self.group_repo.clone(),
        )
    }

    /// Scorboard for a competition.
    ///
    /// Generates the scoreboard for the given competition. The scoreboard
    /// contains all registrations for the competition, partitions them in
    /// missing results, disqualified and the ranked qualified scores.
    ///
    /// # Parameters:
    /// - `competition_id` - the id of the competition
    #[instrument(skip(self))]
    pub async fn competition_scoreboard(
        &self,
        competition_id: Uuid,
    ) -> Result<model::CompetitionScoreboard, CompetitionScoreboardError> {
        tracing::debug!("Loading competition details from competition service");
        let competition_details = self
            .competition_service()
            .competition_details(competition_id)
            .await
            .unwrap();

        tracing::debug!("Partitioning registrations into ones with and without result");
        let registrations = competition_details.registrations;
        let (with_result, missing_results): (Vec<_>, Vec<_>) =
            registrations.into_iter().partition(|r| r.result.is_some());

        tracing::debug!("Partitioning results into ones with and without disqualification");
        let (disqualifications, qualified): (Vec<_>, Vec<_>) =
            with_result.into_iter().partition(|r| {
                let result = r.result.as_ref().unwrap();
                result.disqualified
            });

        tracing::debug!("Ranking the qualified results");
        let mut scores = Vec::with_capacity(qualified.len());
        for registration in qualified.clone().into_iter() {
            let own_time_millis = registration.result.as_ref().unwrap().time_millis;
            let faster_registrations = qualified
                .iter()
                .filter(|r| r.result.as_ref().unwrap().time_millis < own_time_millis)
                .count();

            // If there is nobody faster than you (faster_registration == 0), then
            // you are the first in the ranking.
            let rank = faster_registrations as u32 + 1;
            let result = registration.result.unwrap();
            let competition_score = model::CompetitionScore {
                participant: registration.participant,
                rank,
                time: result.time_millis,
                fina_points: result.fina_points,
            };

            scores.push(competition_score);
        }

        Ok(model::CompetitionScoreboard {
            competition: competition_details.competition,
            scores,
            disqualifications,
            missing_results,
        })
    }

    #[instrument(skip(self))]
    pub async fn group_scoreboard(
        &self,
        group_id: Uuid,
    ) -> Result<model::GroupScoreboard, GroupScoreboardError> {
        tracing::debug!("Fetching group from repository");
        let group = self
            .group_repo
            .group_by_id(group_id)
            .await
            .context("Failed to fetch group from repository")?
            .map(model::Group::from)
            .ok_or(GroupScoreboardError::GroupDoesNotExist)?;

        tracing::debug!("Fetching participants in group from repository");
        let participants = self
            .participant_repo
            .list_participants_in_group(group_id)
            .await
            .context("Failed to fetch participants in group from repository")?;

        let mut participant_details = Vec::with_capacity(participants.len());
        for participant in participants {
            tracing::debug!("Fetching participant details from participant service");
            let pd = self
                .participant_service()
                .participant_details(participant.id)
                .await
                .context("Failed to fetch participant details even through participant exists")?;
            participant_details.push(pd);
        }

        // Precompute the FINA points for all participants
        let participant_points = participant_details
            .iter()
            .map(|pd| pd.fina_points())
            .collect::<Vec<_>>();

        // Calculate the rank for each of the participants
        let mut scores = Vec::with_capacity(participant_details.len());
        let mut missing_results = Vec::new();
        for pd in participant_details.clone().into_iter() {
            // Search for all missing results and add them to the list of missing
            // results.
            if pd.results_missing() {
                let missing = pd
                    .registrations
                    .iter()
                    .filter(|r| r.result.is_none())
                    .map(|r| model::RegistrationDetails {
                        id: r.id,
                        competition: r.competition.clone(),
                        participant: pd.participant.clone(),
                        result: r.result.clone(),
                    });
                missing_results.extend(missing);
            }

            // Calulation of the rank within the group
            let own_fina_points = pd.fina_points();
            let participants_with_higher = participant_points
                .iter()
                .filter(|points| **points > own_fina_points)
                .count();

            // 0 with higher points means 1st rank
            let rank = participants_with_higher as u32 + 1;

            scores.push(model::GroupScore {
                participant: pd.participant,
                fina_points: own_fina_points,
                rank,
            });
        }

        Ok(model::GroupScoreboard {
            group,
            scores,
            missing_results,
        })
    }

    pub async fn participant_scoreboard(
        &self,
        participant_id: Uuid,
    ) -> Result<model::ParticipantScoreboard, ParticipantScoreboardError> {
        tracing::debug!("Fetching the participant details from participant service");
        let participant_details = self
            .participant_service()
            .participant_details(participant_id)
            .await?;

        tracing::debug!("Partitioning registrations into ones with and without results");
        let (with_results, missing_results): (Vec<_>, Vec<_>) = participant_details
            .registrations
            .into_iter()
            .partition(|r| r.result.is_some());

        tracing::debug!("Partitioning registrations with results into qualified and disqualified");
        let (disqualifications, qualified): (Vec<_>, Vec<_>) = with_results
            .into_iter()
            .partition(|r| r.result.as_ref().unwrap().disqualified);

        let mut competition_scores = Vec::with_capacity(qualified.len());
        for registration in qualified {
            tracing::debug!(registration_id = ?registration.id, "Loading competition scoreboard for registration");
            let competition_scoreboard = self
                .competition_scoreboard(registration.competition.id)
                .await.context("Failed to load scoreboard for competition even though competition is referenced in registration")?;

            let participants_score = competition_scoreboard
                .scores
                .into_iter()
                .find(|s| s.participant.id == participant_id)
                .ok_or(anyhow::anyhow!(
                    "Scoreboard has no result even though registration has result"
                ))?;

            competition_scores.push(model::ParticipantCompetitionScore {
                competition: registration.competition,
                time: participants_score.time,
                fina_points: participants_score.fina_points,
                rank: participants_score.rank,
            });
        }

        tracing::debug!("Fetching group scoreboard from score service");
        let group_scoreboard = self
            .group_scoreboard(participant_details.group.id)
            .await
            .context("Failed to fetch group scoreboard from score service")?;
        let group_score = group_scoreboard.scores.into_iter().find(|gs| gs.participant.id == participant_id).context("Group scoreboard does not contain score for participant even though participant is in group")?;

        Ok(model::ParticipantScoreboard {
            participant: participant_details.participant,
            competition_scores,
            group_score: model::ParticipantGroupScore {
                group: group_scoreboard.group,
                fina_points: group_score.fina_points,
                rank: group_score.rank,
            },
            disqualifications,
            missing_results,
        })
    }

    #[instrument(skip(self))]
    pub async fn participant_certificate(
        &self,
        participant_id: Uuid,
    ) -> Result<Vec<u8>, ParticipantCertificateError> {
        let participant_scoreboard = self.participant_scoreboard(participant_id).await?;

        let results = participant_scoreboard
            .competition_scores
            .into_iter()
            .map(|cs| infra::certificate::CompetitionResult {
                distance: cs.competition.distance,
                stroke: cs.competition.stroke.into(),
                millis: cs.time,
                rank: cs.rank,
            })
            .collect();

        let certificate = infra::certificate::Certificate {
            first_name: participant_scoreboard.participant.first_name,
            last_name: participant_scoreboard.participant.last_name,
            birthyear: participant_scoreboard.participant.birthday.year() as _,
            group_points: participant_scoreboard.group_score.fina_points,
            group_rank: participant_scoreboard.group_score.rank,
            results,
        };

        let pdf = infra::certificate::Certificates(vec![certificate])
            .generate_pdf(&self.typst_compiler)
            .await
            .map_err(ParticipantCertificateError::PdfGenerationFailed)?;

        Ok(pdf)
    }

    #[instrument(skip(self))]
    pub async fn participants_fina_points(
        &self,
    ) -> Result<HashMap<Uuid, (bool, u32)>, ServiceRepositoryError> {
        tracing::debug!("Fetching all participants from repository");
        let participants = self
            .participant_repo
            .list_participants()
            .await
            .context("Failed to fetch all participants from repository")?
            .into_iter()
            .map(|p| (p.id, model::Participant::from(p)))
            .collect::<HashMap<_, _>>();

        let mut particpant_scores = HashMap::with_capacity(participants.len());
        for participant in participants.values() {
            tracing::debug!(participant_id = ?participant.id, "Fetching details of participant");
            let pd = self
                .participant_service()
                .participant_details(participant.id)
                .await
                .context("Could not fetch participant details even though participant exists")?;

            particpant_scores.insert(participant.id, (pd.results_missing(), pd.fina_points()));
        }

        Ok(particpant_scores)
    }
}
