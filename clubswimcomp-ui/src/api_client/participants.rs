use super::*;

pub async fn list_participants() -> Result<Vec<model::Participant>> {
    let response = Request::get(&format!("{BASE_URL}/participants"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}

pub async fn participant_details(participant_id: Uuid) -> Result<model::ParticipantDetails> {
    let response = Request::get(&format!("{BASE_URL}/participants/{participant_id}"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}

pub async fn add_participant(
    first_name: String,
    last_name: String,
    gender: model::Gender,
    birthday: NaiveDate,
    group_id: Uuid,
) -> Result<Uuid> {
    let request = api::AddParticipantBody {
        first_name,
        last_name,
        gender,
        birthday,
        group_id,
    };

    let response = Request::post(&format!("{BASE_URL}/participants"))
        .json(&request)
        .unwrap()
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    let response: api::AddParticipantResponse = response.json().await.unwrap();

    Ok(response.participant_id)
}

pub async fn remove_participant(participant_id: Uuid, force_delete: bool) -> Result<()> {
    let response = Request::delete(&format!("{BASE_URL}/participants/{}", participant_id))
        .query([("force_delete", force_delete.to_string())])
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(())
}

pub async fn available_competitions_for_registration(
    participant_id: Uuid,
) -> Result<Vec<model::Competition>> {
    let response = Request::get(&format!(
        "{BASE_URL}/participants/{participant_id}/registrations/available-competitions"
    ))
    .send()
    .await
    .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}

pub async fn register_for_competition(participant_id: Uuid, competition_id: Uuid) -> Result<Uuid> {
    let request_body = api::RegisterForCompetitionBody { competition_id };
    let response = Request::post(&format!(
        "{BASE_URL}/participants/{participant_id}/registrations"
    ))
    .json(&request_body)
    .unwrap()
    .send()
    .await
    .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    let response: api::RegisterForCompetitionResponse = response.json().await.unwrap();

    Ok(response.registration_id)
}

pub async fn unregister_from_competition(
    participant_id: Uuid,
    registration_id: Uuid,
) -> Result<()> {
    let response = Request::delete(&format!(
        "{BASE_URL}/participants/{participant_id}/registrations/{registration_id}"
    ))
    .send()
    .await
    .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(())
}
