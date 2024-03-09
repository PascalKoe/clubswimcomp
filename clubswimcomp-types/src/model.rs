use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Gender {
    Female,
    Male,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, PartialOrd, Ord)]
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
    pub group_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ParticipantDetails {
    #[serde(flatten)]
    pub participant: Participant,
    pub group: Group,
    pub registrations: Vec<ParticipantRegistration>,
}

impl ParticipantDetails {
    /// The total number of FINA points, the participant has achieved.
    ///
    /// Registrations, that do not have a result yet, are ignored. Results that
    /// are classified as disqualified are counted as 0 FINA points.
    pub fn fina_points(&self) -> u32 {
        self.registrations
            .iter()
            .filter(|r| r.result.is_some())
            .map(|r| {
                let result = r.result.as_ref().unwrap();
                !result.disqualified as u32 * result.fina_points
            })
            .sum()
    }

    /// Checks of any registration does not have a result yet.
    pub fn results_missing(&self) -> bool {
        self.registrations.iter().any(|r| r.result.is_none())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Competition {
    pub id: Uuid,
    pub gender: Gender,
    pub distance: u32,
    pub stroke: Stroke,
    pub target_time: u32,
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
    pub time_millis: u32,
    pub fina_points: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GroupDetails {
    #[serde(flatten)]
    pub group: Group,
    pub scores: Vec<GroupScore>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CompetitionScore {
    #[serde(flatten)]
    pub participant: Participant,
    pub time: u32,
    pub fina_points: u32,
    pub rank: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CompetitionScoreboard {
    pub competition: Competition,
    pub scores: Vec<CompetitionScore>,
    pub disqualifications: Vec<CompetitionRegistration>,
    pub missing_results: Vec<CompetitionRegistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ParticipantGroupScore {
    pub group: Group,
    pub fina_points: u32,
    pub rank: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ParticipantCompetitionScore {
    #[serde(flatten)]
    pub competition: Competition,
    pub time: u32,
    pub fina_points: u32,
    pub rank: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ParticipantScoreboard {
    #[serde(flatten)]
    pub participant: Participant,
    pub group_score: ParticipantGroupScore,
    pub competition_scores: Vec<ParticipantCompetitionScore>,
    pub disqualifications: Vec<ParticipantRegistration>,
    pub missing_results: Vec<ParticipantRegistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GroupScore {
    pub participant: Participant,
    pub fina_points: u32,
    pub rank: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GroupScoreboard {
    pub group: Group,
    pub scores: Vec<GroupScore>,
    pub missing_results: Vec<RegistrationDetails>,
}
