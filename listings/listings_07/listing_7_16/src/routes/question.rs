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
        question::{Question, QuestionId},
    },
};

#[instrument]
pub async fn get_questions(params: HashMap<String, String>, store: Store, ) ->
Result<impl Reply, Rejection> {
    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();
    
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    let res: Vec<Question> = match store.get_questions(
        pagination.limit,
        pagination.offset).await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
        };
    
    Ok(warp::reply::json(&res))
}

pub async fn update_question( id: i32, store: Store, question: Question, ) ->
Result<impl Reply, Rejection> {
    match store.questions.write().get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(id: i32, store: Store) ->
Result<impl Reply, Rejection> {
    match store.questions.write().remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

pub async fn add_question(store: Store, question: Question) ->
Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}
