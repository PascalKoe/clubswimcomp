use anyhow::Context;
use clubswimcomp_types::model;
use serde::{Deserialize, Serialize};

use crate::infra;

use super::typst_compiler::{self, TypstCompiler};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certificates(pub Vec<Certificate>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certificate {
    pub first_name: String,
    pub last_name: String,
    pub birthyear: u32,

    pub group_points: u32,
    pub group_rank: u32,

    pub results: Vec<CompetitionResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompetitionResult {
    pub distance: u32,
    pub stroke: Stroke,
    pub millis: u32,
    pub rank: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
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

impl Certificates {
    pub async fn generate_pdf(mut self, typst_compiler: &TypstCompiler) -> anyhow::Result<Vec<u8>> {
        self.0
            .iter_mut()
            .for_each(|cert| cert.results.sort_by_key(|r| r.stroke));

        let input_data = serde_json::to_string(&self.0)
            .context("Invalid certificate data, serialization failed")?;

        let template = "certificate.typst";
        let inputs = [("certificates".to_string(), input_data)]
            .into_iter()
            .collect();
        typst_compiler
            .compile(template, infra::typst_compiler::TypstOutput::Pdf, &inputs)
            .await
            .context("Failed to compile typst registration cards")
    }
}
