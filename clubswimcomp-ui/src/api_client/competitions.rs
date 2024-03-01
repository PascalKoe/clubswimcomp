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
