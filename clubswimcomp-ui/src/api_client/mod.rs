#![allow(dead_code)]

use chrono::NaiveDate;
use clubswimcomp_types::{api, model};
use gloo_net::http::Request;
use uuid::Uuid;

mod competitions;
mod participants;

pub use competitions::*;
pub use participants::*;

const BASE_URL: &str = "http://localhost:3000";
type Result<T> = core::result::Result<T, String>;