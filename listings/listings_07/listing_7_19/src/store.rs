use sqlx::{
    Error, 
    postgres::{
        PgPoolOptions,
        PgPool,
        PgRow
    },
    Row,
};

use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId, NewQuestion},
};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await {
                Ok(pool) => pool,
                Err(e) => panic!("Couldn't establish DB connection!"),
            };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(&self, limit: Option<u32>, offset: u32) ->
    Result<Vec<Question>, sqlx::Error> {
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_all(&self.connection)
        .await {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
    
    pub async fn add_question(&self, new_question: NewQuestion) -> 
    Result<Question, sqlx::Error> {
        match sqlx::query("INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3) RETURNING id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await {
            Ok(question) => Ok(question),
            Err(e) => Err(e),
        }
    }
    
    pub async fn update_question(&self, question: Question, question_id: i32) -> 
    Result<Question, sqlx::Error> {
        match sqlx::query("UPDATE questions SET title = $1, content = $2, tags = $3 where id = $4 RETURNING id, title, contents, tags")
            .bind(question.title)
            .bind(question.content)
            .bind(question.tags)
            .bind(question.question_id)
        .map(|row: PgRow| Question {
            id: QustionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await {
            Ok(question) => Ok(question),
            Err(e) => Err(e),
        }
    }
}
