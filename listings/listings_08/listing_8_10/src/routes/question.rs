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

    match res.error_for_status() {
        Ok(res) => {
            let res = res.text()
                .await
                .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;

            println!("{}", res);

            match store.add_question(new_question).await {
                Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        },
        Err(err) => Err(warp::reject::custom(handle_errors::Error::ExternalAPIError(err))),
    }
}
