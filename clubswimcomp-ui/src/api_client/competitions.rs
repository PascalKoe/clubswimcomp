use super::*;

pub async fn list_competitions() -> Result<Vec<model::Competition>> {
    let response = Request::get(&format!("{BASE_URL}/competitions"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}

pub async fn add_competition(
    distance: u32,
    gender: model::Gender,
    stroke: model::Stroke,
) -> Result<Uuid> {
    let request = api::AddCompetitionRequest {
        distance,
        gender,
        stroke,
    };

    let response = Request::post(&format!("{BASE_URL}/competitions"))
        .json(&request)
        .unwrap()
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    let response: api::AddCompetitionResponse = response.json().await.unwrap();

    Ok(response.competition_id)
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
