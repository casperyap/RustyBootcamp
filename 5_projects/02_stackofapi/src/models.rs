use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    // TODO: add a public `title` field of type String
    // TODO: add a public `description` field of type String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {    
    // TODO: add a public `question_uuid` field of type String
    // TODO: add a public `title` field of type String
    // TODO: add a public `description` field of type String
    // TODO: add a public `created_at` field of type String

    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,    
}

// TODO: create a struct called `QuestionId`
//       derive the following traits: Serialize, Deserialize
//       add a public `question_uuid` field of type String
#[derive(Serialize, Deserialize)]
pub struct QuestionId{
    pub question_uuid: String,
}

// ----------

// TODO: create a struct called `Answer`
//       derive the following traits: Serialize, Deserialize
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String
#[derive(Serialize, Deserialize)]
pub struct Answer{
    pub question_uuid: String,
    pub content: String,
}

// TODO: create a struct called `AnswerDetail`
//       derive the following traits: Serialize, Deserialize, Debug, Clone, PartialEq
//       add a public `answer_uuid` field of type String
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String
//       add a public `created_at` field of type String
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail{
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

// TODO: create a struct called `AnswerId`
//       derive the following traits: Serialize, Deserialize
//       add a public `answer_uuid` field of type String
#[derive(Serialize, Deserialize)]
pub struct AnswerId{
    pub answer_uuid: String,
}