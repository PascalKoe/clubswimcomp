use anyhow::{Context, Result};
use clubswimcomp_types::model;
use qrcode::{render::svg, QrCode};
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{db, infra};

pub struct RegistrationCardService {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
    typst_compiler: infra::typst_compiler::TypstCompiler,
}

#[derive(Debug, Error)]
pub enum EventRegistrationCardsError {
    #[error("Failed to generate the start card PDF")]
    PdfGenerationFailed(anyhow::Error),

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ParticipantRegistrationCardsError {
    #[error("The participant does not exist")]
    ParticipantDoesNotExist,

    #[error("Failed to generate the start card PDF")]
    PdfGenerationFailed(anyhow::Error),

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RegistrationCardsError {
    #[error("The registration does not exist")]
    RegistrationDoesNotExist,

    #[error("Failed to generate the start card PDF")]
    PdfGenerationFailed(anyhow::Error),

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

impl RegistrationCardService {
    pub fn new(
        participant_repo: db::participants::Repository,
        registration_repo: db::registrations::Repository,
        competition_repo: db::competitions::Repository,
        typst_compiler: infra::typst_compiler::TypstCompiler,
    ) -> Self {
        Self {
            participant_repo,
            registration_repo,
            competition_repo,
            typst_compiler,
        }
    }

    #[instrument(skip(self))]
    async fn generate_pdf_for_cards(
        &self,
        cards: Vec<infra::registration_card::RegistrationCard>,
    ) -> anyhow::Result<Vec<u8>> {
        infra::registration_card::RegistrationCards {
            event_name: "TEST EVENT NAME".to_string(),
            organization: "TEST ORGANIZATION".to_string(),
            cards,
        }
        .generate_pdf(&self.typst_compiler)
        .await
        .context("Failed to generate PDF for registration cards")
    }

    #[instrument(skip(self))]
    async fn load_for_registration(
        &self,
        registration_id: Uuid,
    ) -> anyhow::Result<Option<infra::registration_card::RegistrationCard>> {
        tracing::debug!("Fetching registration from repository");
        let registration = self
            .registration_repo
            .registration_by_id(registration_id)
            .await
            .context("Failed to fetch registration from repository")?;
        let Some(registration) = registration else {
            return Ok(None);
        };

        tracing::debug!(participant_id = ?registration.participant_id, "Fetching participant for registration");
        let participant = self
            .participant_repo
            .participant_by_id(registration.participant_id)
            .await
            .context("Failed to load participant for registration from repository")?
            .map(model::Participant::from)
            .context(
                "Participant is referenced in registration but could not be found in repository",
            )?;

        tracing::debug!(competition_id = ?registration.competition_id, "Fetching competition for registration");
        let competition = self
            .competition_repo
            .competition_by_id(registration.competition_id)
            .await
            .context("Failed to load competition for registration from repository")?
            .map(model::Competition::from)
            .context(
                "Competition is referenced in registration but could not be found in repository",
            )?;

        let qr_code = QrCode::new(registration.id.to_string().as_bytes()).unwrap();
        let qr_code = qr_code.render::<svg::Color>().build();

        Ok(Some(infra::registration_card::RegistrationCard {
            first_name: participant.first_name.clone(),
            last_name: participant.last_name.clone(),
            distance: competition.distance,
            stroke: competition.stroke.into(),
            gender: competition.gender.into(),
            participant_number: participant.short_code.clone(),
            qr_code,
        }))
    }

    #[instrument(skip(self))]
    async fn load_for_participant(
        &self,
        participant: model::Participant,
    ) -> anyhow::Result<Vec<infra::registration_card::RegistrationCard>> {
        tracing::debug!("Loading all registrations for participant");
        let db_registrations = self
            .registration_repo
            .registrations_of_participant(participant.id)
            .await
            .context("Failed to load registrations for participant from repository")?;

        tracing::debug!("Loading the registration details");
        let mut registration_cards = Vec::with_capacity(db_registrations.len());
        for db_registration in db_registrations.into_iter() {
            tracing::debug!(
                competition_id = ?db_registration.competition_id,
                "Loading competition for registration"
            );
            let competition = self
                    .competition_repo
                    .competition_by_id(db_registration.competition_id)
                    .await
                    .context("Failed to load competition for registration from repository")?
                    .map(model::Competition::from)
                    .context("Competition is referenced in registration but could not be found in repository")?;

            let qr_code = QrCode::new(db_registration.id.to_string().as_bytes()).unwrap();
            let qr_code = qr_code.render::<svg::Color>().build();

            let registeration_card = infra::registration_card::RegistrationCard {
                first_name: participant.first_name.clone(),
                last_name: participant.last_name.clone(),
                distance: competition.distance,
                stroke: competition.stroke.into(),
                gender: competition.gender.into(),
                participant_number: participant.short_code.clone(),
                qr_code,
            };

            registration_cards.push(registeration_card);
        }

        Ok(registration_cards)
    }

    #[instrument(skip(self))]
    pub async fn all_registration_cards(&self) -> Result<Vec<u8>, EventRegistrationCardsError> {
        tracing::debug!("Fetching participants from repository");
        let mut participants = self
            .participant_repo
            .list_participants()
            .await
            .context("Failed to fetch participants from repository")?
            .into_iter()
            .map(model::Participant::from)
            .collect::<Vec<_>>();

        participants.sort_by_cached_key(|p| format!("{}, {}", p.last_name, p.first_name));

        let mut registration_cards = Vec::new();
        for participant in participants {
            tracing::debug!(participant_id = ?participant.id, "Loading participants card information");
            let mut participant_cards = self
                .load_for_participant(participant)
                .await
                .context("Failed to load participant registration card information")?;

            registration_cards.append(&mut participant_cards);
        }

        tracing::debug!("Generating registration cards PDF");
        self.generate_pdf_for_cards(registration_cards)
            .await
            .context("Failed to generate registration cards for participant")
            .map_err(EventRegistrationCardsError::PdfGenerationFailed)
    }

    #[instrument(skip(self))]
    pub async fn participants_registration_cards(
        &self,
        participant_id: Uuid,
    ) -> Result<Vec<u8>, ParticipantRegistrationCardsError> {
        tracing::debug!("Fetching participant from repository");
        let participant = self
            .participant_repo
            .participant_by_id(participant_id)
            .await
            .context("Failed to fetch participant from repository")?
            .map(model::Participant::from)
            .ok_or(ParticipantRegistrationCardsError::ParticipantDoesNotExist)?;

        tracing::debug!("Loading participants registration card information");
        let participant_cards = self
            .load_for_participant(participant)
            .await
            .context("Failed to load participant registration card information")?;

        tracing::debug!("Generating registration cards PDF");
        self.generate_pdf_for_cards(participant_cards)
            .await
            .context("Failed to generate registration cards for participant")
            .map_err(ParticipantRegistrationCardsError::PdfGenerationFailed)
    }

    #[instrument(skip(self))]
    pub async fn registration_card(
        &self,
        registration_id: Uuid,
    ) -> Result<Vec<u8>, RegistrationCardsError> {
        tracing::debug!("Fetching registration card content from repository");
        let card = self
            .load_for_registration(registration_id)
            .await
            .context("Failed to fetch registration card content from repository")?
            .ok_or(RegistrationCardsError::RegistrationDoesNotExist)?;

        tracing::debug!("Generating registration card PDF");
        self.generate_pdf_for_cards(vec![card])
            .await
            .context("Failed to generate registration card for registration")
            .map_err(RegistrationCardsError::PdfGenerationFailed)
    }
}
