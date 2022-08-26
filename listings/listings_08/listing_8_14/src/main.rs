use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "xxxxx")
        .body("a list with shit words")
        .send()
        .await?;

    let status_code = res.status();
    let message = res.text().await?;

    let response = json!({
        "StatusCode": status_code.as_str(),
        "Message": message
    });

    println!("{:#?}", response);

    Ok(())
}
