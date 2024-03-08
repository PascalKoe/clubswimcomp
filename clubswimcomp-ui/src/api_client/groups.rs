use super::*;

pub async fn list_groups() -> Result<Vec<model::Group>> {
    let response = Request::get(&format!("{BASE_URL}/groups"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}

pub async fn add_group(name: String) -> Result<Uuid> {
    let body = api::AddGroupRequest { name };
    let response = Request::post(&format!("{BASE_URL}/groups"))
        .json(&body)
        .unwrap()
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    let response: api::AddGroupResponse = response.json().await.unwrap();
    Ok(response.group_id)
}

pub async fn group_details(group_id: Uuid) -> Result<model::GroupDetails> {
    let response = Request::get(&format!("{BASE_URL}/groups/{group_id}"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(response.json().await.unwrap())
}
