use chrono::{NaiveDate, Utc};
use clubswimcomp_types::model::*;

use crate::db;

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
            group_id: p.group_id,
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

impl From<db::competitions::Competition> for Competition {
    fn from(c: db::competitions::Competition) -> Self {
        Self {
            id: c.id,
            gender: c.gender.into(),
            distance: c.distance as _,
            stroke: c.stroke.into(),
            target_time: c.target_time,
        }
    }
}

impl From<db::registrations::RegistrationResult> for RegistrationResult {
    fn from(r: db::registrations::RegistrationResult) -> Self {
        Self {
            disqualified: r.disqualified,
            time_millis: r.time_millis,
            fina_points: r.fina_points as _,
        }
    }
}

impl From<db::groups::Group> for Group {
    fn from(g: db::groups::Group) -> Self {
        Self {
            id: g.id,
            name: g.name,
        }
    }
}
