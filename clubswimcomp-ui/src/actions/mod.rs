use chrono::NaiveDate;
use clubswimcomp_types::{api, model};
use leptos::*;
use uuid::Uuid;

use crate::api_client;

struct CreateParticipantAction {
    action: Action<(String, String, model::Gender, NaiveDate), Result<Uuid, String>>,
}

pub fn use_create_participant() -> CreateParticipantAction {
    let action = create_action(|input: &(String, String, model::Gender, NaiveDate)| {
        let last_name = input.0.clone();
        let first_name = input.1.clone();
        let gender = input.2;
        let birthday = input.3;

        async move { api_client::add_participant(first_name, last_name, gender, birthday).await }
    });

    CreateParticipantAction { action }
}

impl CreateParticipantAction {
    pub fn dispatch(
        &self,
        last_name: String,
        first_name: String,
        gender: model::Gender,
        birthday: NaiveDate,
    ) {
        self.action
            .dispatch((last_name, first_name, gender, birthday));
    }

    pub fn is_running(&self) -> bool {
        self.action.pending().get()
    }

    // pub fn
}
