use warp::{http::StatusCode, Rejection, Reply};

use crate::{
    store::Store,
    types::answer::NewAnswer,
    profanity::check_profanity,
};

pub async fn add_answer(store: Store, new_answer: NewAnswer) ->
Result<impl Reply, Rejection> {
    let content = match check_profanity(new_answer.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };

    match store.add_answer(answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
