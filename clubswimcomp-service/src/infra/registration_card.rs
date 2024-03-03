use anyhow::Context;
use clubswimcomp_types::model;
use serde::{Deserialize, Serialize};

use crate::infra;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistrationCards {
    pub event_name: String,
    pub organization: String,

    pub cards: Vec<RegistrationCard>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistrationCard {
    pub first_name: String,
    pub last_name: String,
    pub distance: u32,
    pub stroke: Stroke,
    pub gender: Gender,
    pub participant_number: String,
    pub qr_code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stroke {
    Butterfly,
    Back,
    Breast,
    Freestyle,
}

impl From<model::Stroke> for Stroke {
    fn from(stroke: model::Stroke) -> Self {
        match stroke {
            model::Stroke::Butterfly => Self::Butterfly,
            model::Stroke::Back => Self::Back,
            model::Stroke::Breast => Self::Breast,
            model::Stroke::Freestyle => Self::Freestyle,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

impl From<model::Gender> for Gender {
    fn from(gender: model::Gender) -> Self {
        match gender {
            model::Gender::Female => Self::Female,
            model::Gender::Male => Self::Male,
        }
    }
}

impl RegistrationCards {
    pub async fn generate_pdf(&self) -> anyhow::Result<Vec<u8>> {
        let input_data = serde_json::to_string(self)
            .context("Invalid registration card, serialization failed")?;

        let template = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/registration_card.typst"
        ));
        let inputs = [("registration_cards".to_string(), input_data)]
            .into_iter()
            .collect();
        infra::typst_compiler::compile(template, infra::typst_compiler::TypstOutput::Pdf, &inputs)
            .await
            .context("Failed to compile typst registration cards")
    }
}
