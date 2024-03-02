use super::*;

pub async fn remove_registration_result(registration_id: Uuid) -> Result<()> {
    let response = Request::delete(&format!("{BASE_URL}/results/{registration_id}"))
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return Err(response.text().await.unwrap());
    }

    Ok(())
}
