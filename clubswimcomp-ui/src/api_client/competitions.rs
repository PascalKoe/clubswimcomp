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
    target_time: u32,
) -> Result<Uuid> {
    let request = api::AddCompetitionRequest {
        distance,
        gender,
        stroke,
        target_time,
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

pub async fn delete_competition(competition_id: Uuid, force_delete: bool) -> Result<()> {
    let response = Request::delete(&format!("{BASE_URL}/competitions/{}", competition_id))
        .query([("force_delete", force_delete.to_string())])
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(())
}

pub async fn competition_scoreboard(competition_id: Uuid) -> Result<model::CompetitionScoreboard> {
    let response = Request::get(&format!(
        "{BASE_URL}/competitions/{competition_id}/scoreboard"
    ))
    .send()
    .await
    .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}
