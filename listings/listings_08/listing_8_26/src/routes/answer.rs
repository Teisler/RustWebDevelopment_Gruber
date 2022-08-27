use std::collections::HashMap;
use warp::{http::StatusCode, Rejection, Reply};

use crate::{
    store::Store,
    types::answer::NewAnswer,
    profanity::check_profanity,
};

pub async fn add_answer(store: Store, params: HashMap<String, String>,) ->
Result<impl Reply, Rejection> {
    let content = match check_profanity(params.get("content").unwrap().to_string()).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let answer = NewAnswer {
        content,
        question_id: params.get("questionId").unwrap().parse().unwrap(),
    };

    match store.add_answer(answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
