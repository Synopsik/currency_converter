use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ExchangeResponse {
    date: String,
    #[serde(flatten)]
    rates: HashMap<String, HashMap<String, f64>>,
}

pub async fn get_exchange_rate(
    from: &str,
    to: &str
) -> Result<
    f64,
    Box<dyn std::error::Error>,
> {
    let from = from.to_lowercase();
    let to = to.to_lowercase();

    let primary_url = format!(
        "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies/{}.json",
        from
    );

    let fallback_url = format!(
        "https://currency-api.pages.dev/v1/currencies/{}.json",
        from
    );

    let client = reqwest::Client::new();

    let response = match client.get(&primary_url).send().await {
        Ok(resp) => resp,
        Err(_) => client.get(&fallback_url).send().await?,
    };

    let data: ExchangeResponse = response.json().await?;

    data.rates
        .get(&from)
        .and_then(|rates| rates.get(&to).copied())
        .ok_or_else(|| format!("Rate not found for {}/{}", from, to).into())
}