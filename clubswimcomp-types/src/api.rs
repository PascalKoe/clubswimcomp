use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddParticipantBody {
    pub first_name: String,
    pub last_name: String,
    pub gender: model::Gender,
    pub birthday: NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddParticipantResponse {
    pub participant_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoveParticipantParameters {
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RegisterForCompetitionBody {
    pub competition_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RegisterForCompetitionResponse {
    pub registration_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddCompetitionRequest {
    pub gender: model::Gender,
    pub stroke: model::Stroke,
    pub distance: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddCompetitionResponse {
    pub competition_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DeleteCompetitionParams {
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EnterResultBody {
    pub registration_id: Uuid,
    pub disqualified: bool,
    pub time_millis: u32,
}
