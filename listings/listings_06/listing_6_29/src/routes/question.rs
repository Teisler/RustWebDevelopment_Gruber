use handle_errors::Error;
use std::collections::HashMap;
use warp::{http::StatusCode, Rejection, Reply};
use tracing::{
    info,
    instrument,
};

use crate::{
    store::Store,
    types::{
        pagination::extract_pagination,
        question::{Question, QuestionId},
    },
};

pub async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    info!("querying questions");
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        info!(pagination = true);
        let res: Vec<Question> = store.questions.read().values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        info!(pagination = false);
        let res: Vec<Question> = store.questions.read().values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.questions.write().get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
