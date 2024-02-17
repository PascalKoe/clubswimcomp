use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Gender {
    Female,
    Male,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
    pub min_age: Option<u32>,
    pub max_age: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CompetitionDetails {
    #[serde(flatten)]
    pub competition: Competition,
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
