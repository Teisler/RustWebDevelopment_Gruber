use std::collections::HashMap;
use warp::{http::StatusCode, Rejection, Reply};

use crate::{
    store::Store,
    types::{
        answer::{Answer, AnswerId, NewAnswer},
        question::QuestionId,
    },
};

pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let answer = Answer {
        id: AnswerId(1),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("questionId").unwrap().parse::<i32>().expect("Invalid Question_id")),
    };

    store.answers.write().insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
