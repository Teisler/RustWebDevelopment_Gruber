use std::collections::HashMap;

use warp::{
    http::StatusCode,
    Rejection,
    Reply,
};

use tracing::{
    event,
    instrument,
    Level,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    store::Store,
    types::{
        pagination::{
            extract_pagination,
            Pagination,
        },
        question::{
            Question,
            NewQuestion,
        },
    },
};

#[instrument]
pub async fn get_questions(params: HashMap<String, String>, store: Store,) -> 
Result<impl Reply, Rejection> {
    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();
    
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    match store.get_questions(pagination.limit, pagination.offset).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(id: i32, store: Store, question: Question,) ->
Result<impl Reply, Rejection> {
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(id: i32, store: Store) -> 
Result<impl Reply, Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK)
        ),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

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
    #[serde(rename = "replaceLen")]
    replace_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

pub async fn add_question(store: Store, new_question: NewQuestion,) -> 
Result<impl Reply, Rejection> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "xxxxx")
        .body("a list with shit words")
        .send()
        .await
        .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;

    if !res.status().is_success() {
        return if res.status().is_client_error() {
            let err = transform_error(res).await;
            Err(warp::reject::custom(handle_errors::Error::ClientError(err)))
        } else {
            let err = transform_error(res).await;
            Err(warp::reject::custom(handle_errors::Error::ServerError(err)))
        }
    }

    let res = res.json::<BadWordsResponse>()
        .await
        .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;

    let content = res.censored_content;

    let question = NewQuestion {
        title: new_question.title,
        content,
        tags: new_question.tags
    };

    match store.add_question(question).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
