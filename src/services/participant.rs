use anyhow::{Context, Result};
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use crate::{db, model};

pub struct ParticipantService {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
}

impl ParticipantService {
    pub fn new(
        participant_repo: db::participants::Repository,
        registration_repo: db::registrations::Repository,
        competition_repo: db::competitions::Repository,
    ) -> Self {
        Self {
            participant_repo,
            registration_repo,
            competition_repo,
        }
    }

    pub async fn list_participants(&self) -> Result<Vec<model::Participant>> {
        let participants = self
            .participant_repo
            .list_participants()
            .await
            .context("Failed to load participants from repository")?
            .into_iter()
            .map(model::Participant::from)
            .collect();

        Ok(participants)
    }

    pub async fn add_participant(
        &self,
        first_name: &str,
        last_name: &str,
        gender: model::Gender,
        birthday: NaiveDate,
    ) -> Result<model::Participant> {
        let participant_id = self
            .participant_repo
            .create_participant(first_name, last_name, gender.into(), birthday)
            .await
            .context("Failed to add participant to repository")?;

        let participant = self
            .participant_repo
            .participant_by_id(participant_id)
            .await
            .context("Failed to load participant from repository")?
            .map(model::Participant::from)
            .context("Participant does not exist in database should have been created just now")?;

        Ok(participant)
    }

    pub async fn participant_details(
        &self,
        participant_id: Uuid,
    ) -> Result<Option<model::ParticipantDetails>> {
        let participant = self
            .participant_repo
            .participant_by_id(participant_id)
            .await
            .context("Failed to load participant from repository")?
            .map(model::Participant::from);

        let Some(participant) = participant else {
            tracing::info!("Repository did not find the requested participant");
            return Ok(None);
        };

        let registrations = self
            .registration_repo
            .list_for_participant(participant_id)
            .await
            .context("Failed to fetch list of registrations for participant from repository")?
            .into_iter()
            .map(|r| async move {
                self.load_registration(r.id, r.competition_id)
                    .await?
                    .context("Details on registration are missing")
            });

        let registrations = futures::future::join_all(registrations)
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()
            .context("Failed to fetch registration details for participant")?;

        Ok(Some(model::ParticipantDetails {
            participant,
            registrations,
        }))
    }

    async fn load_registration(
        &self,
        registration_id: Uuid,
        competition_id: Uuid,
    ) -> Result<Option<model::ParticipantRegistration>> {
        let competition = self
            .load_competition(competition_id)
            .await
            .context("Failed to load competition for registration")?;

        let Some(competition) = competition else {
            tracing::info!(
                "Tried to load competition for registration but the competition does not exists"
            );
            return Ok(None);
        };

        let result = self
            .load_registration_results(registration_id)
            .await
            .context("Failed to load registration result from repository")?;

        Ok(Some(model::ParticipantRegistration {
            id: registration_id,
            competition,
            result,
        }))
    }

    async fn load_competition(&self, competition_id: Uuid) -> Result<Option<model::Competition>> {
        let competition = self
            .competition_repo
            .competition_by_id(competition_id)
            .await
            .context("Failed to load competition form repository")?
            .map(model::Competition::from);

        Ok(competition)
    }

    async fn load_registration_results(
        &self,
        registration_id: Uuid,
    ) -> Result<Option<model::RegistrationResult>> {
        let registration_result = self
            .registration_repo
            .result_for_registration(registration_id)
            .await
            .context("Failed to load registration result from repository")?
            .map(model::RegistrationResult::from);

        Ok(registration_result)
    }
}

impl From<db::participants::Participant> for model::Participant {
    fn from(p: db::participants::Participant) -> Self {
        Self {
            id: p.id,
            short_code: format!("{:04}", p.short_id),
            first_name: p.first_name,
            last_name: p.last_name,
            gender: p.gender.into(),
            birthday: p.birthday,
            age: age_from_birthday(p.birthday),
        }
    }
}

impl From<db::Gender> for model::Gender {
    fn from(g: db::Gender) -> Self {
        match g {
            db::Gender::Female => Self::Female,
            db::Gender::Male => Self::Male,
        }
    }
}

impl From<model::Gender> for db::Gender {
    fn from(g: model::Gender) -> Self {
        match g {
            model::Gender::Female => Self::Female,
            model::Gender::Male => Self::Male,
        }
    }
}

impl From<db::Stroke> for model::Stroke {
    fn from(s: db::Stroke) -> Self {
        match s {
            db::Stroke::Butterfly => Self::Butterfly,
            db::Stroke::Back => Self::Back,
            db::Stroke::Breast => Self::Breast,
            db::Stroke::Freestyle => Self::Freestyle,
        }
    }
}

impl From<db::competitions::Competition> for model::Competition {
    fn from(c: db::competitions::Competition) -> Self {
        Self {
            id: c.id,
            gender: c.gender.into(),
            distance: c.distance as _,
            stroke: c.stroke.into(),
            min_age: c.min_age.map(|age| age as _),
            max_age: c.max_age.map(|age| age as _),
        }
    }
}

impl From<db::registrations::RegistrationResult> for model::RegistrationResult {
    fn from(r: db::registrations::RegistrationResult) -> Self {
        Self {
            disqualified: r.disqualified,
            time_millis: r.time_millis,
        }
    }
}

/// Calculate the age based on the birthday.
///
/// In case the birthday lies in the future, an age of 0 will be returned.
fn age_from_birthday(birthday: NaiveDate) -> u32 {
    Utc::now()
        .naive_local()
        .date()
        .years_since(birthday)
        .unwrap_or_default()
}
