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
        pagination::{
            extract_pagination,
            Pagination,
        },
        question::{Question, QuestionId},
    },
};

pub async fn add_question(store: Store, new_question: NewQuestion,) -> 
Result<impl Reply, Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

#[instrument]
pub async fn get_questions(params: HashMap<String, String>, store: Store,) -> 
Result<impl Reply, Rejection> {
    info!("querying questions");
    let mut pagination = Pagination::default();
    
    if !params.is_empty() {
        info!(pagination = true);
        pagination = extract_pagination(params)?;
    }

    let res: Vec<Question> = match store.get_questions(pagination.limit,
        pagination.offset).await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };
    
    Ok(warp::reply::json(&res))
}

pub async fn update_question(id: i32, store: Store, question: Question,) ->
Result<impl Reply, Rejection> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(id: i32, store: Store) -> 
Result<impl Reply, Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }

    Ok(warp::reply::with_status(format!("Question {} deleted", id), StatusCode::OK))
}
