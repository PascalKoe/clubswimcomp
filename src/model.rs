use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Gender {
    Female,
    Male,
}

impl From<db::Gender> for Gender {
    fn from(g: db::Gender) -> Self {
        match g {
            db::Gender::Female => Self::Female,
            db::Gender::Male => Self::Male,
        }
    }
}

impl From<Gender> for db::Gender {
    fn from(g: Gender) -> Self {
        match g {
            Gender::Female => Self::Female,
            Gender::Male => Self::Male,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Stroke {
    Butterfly,
    Back,
    Breast,
    Freestyle,
}

impl From<db::Stroke> for Stroke {
    fn from(s: db::Stroke) -> Self {
        match s {
            db::Stroke::Butterfly => Self::Butterfly,
            db::Stroke::Back => Self::Back,
            db::Stroke::Breast => Self::Breast,
            db::Stroke::Freestyle => Self::Freestyle,
        }
    }
}

impl From<Stroke> for db::Stroke {
    fn from(s: Stroke) -> Self {
        match s {
            Stroke::Butterfly => Self::Butterfly,
            Stroke::Back => Self::Back,
            Stroke::Breast => Self::Breast,
            Stroke::Freestyle => Self::Freestyle,
        }
    }
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

impl From<db::participants::Participant> for Participant {
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

impl From<db::competitions::Competition> for Competition {
    fn from(c: db::competitions::Competition) -> Self {
        Self {
            id: c.id,
            gender: c.gender.into(),
            distance: c.distance as _,
            stroke: c.stroke.into(),
        }
    }
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
pub struct RegistrationResult {
    pub disqualified: bool,
    pub time_millis: i64,
}

impl From<db::registrations::RegistrationResult> for RegistrationResult {
    fn from(r: db::registrations::RegistrationResult) -> Self {
        Self {
            disqualified: r.disqualified,
            time_millis: r.time_millis,
        }
    }
}

/*
// List Participants
GET     /participants
// Add Participant
POST    /participants

// Show Participant Details
GET     /participants/<PARTICIPANT-ID>
// Delete Participant
DELETE  /participants/<PARTICIPANT-ID>






// List available competitions
GET     /participants/<PARTICIPANT-ID>/available-competitions
// Add Registration To Participant
POST    /participants/<PARTICIPANT-ID>/registrations
// Delete Registration From Participant
DELETE  /participants/<PARTICIPANT-ID>/registrations/<REGISTRATION-ID>
// Add Result To Registration
POST    /participants/<PARTICIPANT-ID>/registrations/<REGISTRATION-ID>/result
// Delete Result Of Registration
DELETE  /participants/<PARTICIPANT-ID>/registrations/<REGISTRATION-ID>/result

// Participant Certificate
GET     /participants/<PARTICIPANT-ID>/certificate
// Participant Competition Cards
GET     /participants/<PARTICIPANT-ID>/competition-cards







GET     /trials
POST    /competitions


GET     /competitions/<COMPETITION-ID>

GET     /competitions/results
GET     /competitions/<COMPETITION-ID>/results


 */
