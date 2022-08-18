#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "xxxxx")
        .body("a list with shit words")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", res);

    Ok(())
}
