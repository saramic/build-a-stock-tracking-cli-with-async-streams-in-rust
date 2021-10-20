use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::{http::StatusCode, reject::Reject, Filter, Rejection, Reply};

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug, Serialize)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions(id_param: String) -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str(&id_param).expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    match question.id.0.is_empty() {
        true => Err(warp::reject::custom(InvalidId)),
        false => Ok(warp::reply::json(&question)),
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        // ONE version
        // .and(warp::path("questions"))
        // .and(warp::path::param())
        // ANOTHER version
        .and(warp::path!("questions" / String))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items;

    println!(
        "listening on http://127.0.0.1:1337\nbut only serving http://127.0.0.1:1337/questions"
    );

    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await
}
