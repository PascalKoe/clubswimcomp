use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Gender {
    Female,
    Male,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Stroke {
    Butterfly,
    Back,
    Breast,
    Freestyle,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Participant {
    pub id: Uuid,
    pub short_code: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub birthday: NaiveDate,
    pub age: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ParticipantDetails {
    #[serde(flatten)]
    pub participant: Participant,
    pub registrations: Vec<ParticipantRegistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Competition {
    pub id: Uuid,
    pub gender: Gender,
    pub distance: u32,
    pub stroke: Stroke,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CompetitionDetails {
    #[serde(flatten)]
    pub competition: Competition,
    pub results_pending: bool,
    pub registrations: Vec<CompetitionRegistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CompetitionRegistration {
    pub id: Uuid,
    pub participant: Participant,
    pub result: Option<RegistrationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ParticipantRegistration {
    pub id: Uuid,
    pub competition: Competition,
    pub result: Option<RegistrationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RegistrationDetails {
    pub id: Uuid,
    pub participant: Participant,
    pub competition: Competition,
    pub result: Option<RegistrationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RegistrationResult {
    pub disqualified: bool,
    pub time_millis: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CompetitionScoreboard {
    #[serde(flatten)]
    pub competition: Competition,
    pub scores: Vec<CompetitionScore>,
    pub disqualifications: Vec<CompetitionRegistration>,
    pub missing_results: Vec<CompetitionRegistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CompetitionScore {
    #[serde(flatten)]
    pub participant: Participant,
    pub result: RegistrationResult,
    pub rank: u32,
}
