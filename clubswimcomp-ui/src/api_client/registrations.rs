use super::*;

pub async fn remove_registration_result(registration_id: Uuid) -> Result<()> {
    let response = Request::delete(&format!(
        "{BASE_URL}/registrations/{registration_id}/result"
    ))
    .send()
    .await
    .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(())
}

pub async fn registration_details(registration_id: Uuid) -> Result<model::RegistrationDetails> {
    let response = Request::get(&format!("{BASE_URL}/registrations/{registration_id}"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}

pub async fn add_result(registration_id: Uuid, disqualified: bool, time_millis: u32) -> Result<()> {
    let body = api::EnterResultBody {
        disqualified,
        time_millis,
    };
    let response = Request::post(&format!(
        "{BASE_URL}/registrations/{registration_id}/result"
    ))
    .json(&body)
    .unwrap()
    .send()
    .await
    .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(())
}
