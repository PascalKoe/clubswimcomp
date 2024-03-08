#![allow(dead_code)]

use chrono::NaiveDate;
use clubswimcomp_types::{api, model};
use gloo_net::http::Request;
use uuid::Uuid;

mod competitions;
mod groups;
mod participants;
mod registrations;

pub use competitions::*;
pub use groups::*;
pub use participants::*;
pub use registrations::*;

const BASE_URL: &str = "http://localhost:3000";
type Result<T> = core::result::Result<T, String>;
