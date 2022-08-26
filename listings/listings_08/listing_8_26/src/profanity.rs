use serde::{Deserialize, Serialize};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{
    RetryTransientMiddleware,
    policies::ExponentialBackoff
};
use reqwest_retry = {RetryTransientMiddleware, policies::ExponentialBackoff};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
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

pub async fn check_profanity(content: String) ->
Result<String, handle_errors::Error> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_characters=*")
        .header("apikey", "API_KEY")
        .body(content)
        .send()
        .await
        .map_err(|e| handle_errors::Error::MiddlewareRequestAPIError(e))?;

    if !res.status().is_success() {
        return if res.status().is_client_error() {
            let err = transform_error(res).await;
            Err(handle_errors::Error::ClientError(err))
        } else {
            let err = transform_error(res).await;
            Err(handle_errors::Error::MiddlewareReqwestAPIError(err))
        }
    }

    match res.json::<BadWordsResponse>()
        .await {
        Ok(res) => Ok(res.censored_content),
        Err(e) => Err(handle_errors::Error::ReqwestAPIError(e)),
    }
}

async fn transform_error(res: reqwest::Response) ->
handle_errors::APILayerError {
    handle_errors::APILayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}