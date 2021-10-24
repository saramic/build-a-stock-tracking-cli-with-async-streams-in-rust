use serde::Serialize;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Serialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
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
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }
    fn init(&mut self) -> Self {
        let question = Question::new(
            QuestionId::from_str("1").expect("invalid"),
            "How?".to_string(),
            "Please help!".to_string(),
            Some(vec!["general".to_string()]),
        );
        self.add_question(&question)
    }

    fn add_question(&mut self, question: &Question) -> Self {
        self.questions.insert(question.clone().id, question.clone());

        Self {
            questions: self.questions.clone(),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let store = &mut Store::new();
    store.init();
    println!("{:?}", store)
}
