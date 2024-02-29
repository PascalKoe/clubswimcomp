use clubswimcomp_types::model;
use gloo_net::http::{Request, RequestBuilder};
use uuid::Uuid;

const BASE_URL: &str = "http://localhost:3000";

type Result<T> = core::result::Result<T, String>;

pub async fn participants_overview() -> Result<Vec<model::Participant>> {
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

pub async fn participants_details(participant_id: Uuid) -> Result<model::ParticipantDetails> {
    let response = Request::get(&format!("{BASE_URL}/participants/{participant_id}"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}

pub async fn competition_overview() -> Result<Vec<model::Competition>> {
    let response = Request::get(&format!("{BASE_URL}/competitions"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}

pub async fn competition_details(competition_id: Uuid) -> Result<model::CompetitionDetails> {
    let response = Request::get(&format!("{BASE_URL}/competitions/{competition_id}"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}
