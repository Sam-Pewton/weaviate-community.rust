use std::error::Error;

pub async fn decode_response_to_json(
    response: reqwest::Response
) -> Result<serde_json::Value, Box<dyn Error>> {
    let json = response.json::<serde_json::Value>().await?;
    Ok(json)
}
