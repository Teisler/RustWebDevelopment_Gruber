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

use crate::profanity::check_profanity;

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
    let title = tokio::spawn(check_profanity(question.title));
    let content = tokio::spawn(check_profanity(question.content));

    let (title, content) = (title.await.unwrap(), content.await.unwrap());

    if title.is_err() {
        return Err(warp::reject::custom(title.unwrap_err()));
    }

    if content.is_err() {
        return Err(warp::reject::custom(content.unwrap_err()));
    }

    let question = Question {
        id: question.id,
        title: title.unwrap(),
        content: content.unwrap(),
        tags: question.tags,
    };

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
    let title = match check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    match store.add_question(question).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
