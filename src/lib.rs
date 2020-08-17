use std::collections::{HashMap, HashSet};

pub type EntryId = u32;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Register {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub type SubjectId = EntryId;
pub type SubjectCollection = HashMap<EntryId, Subject>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub id: SubjectId,
    pub field: Field,
    pub topics: TopicCollection,
}

impl Subject {
    pub fn init(id: SubjectId, name: String, field: Field) -> Self {
        Self{id, name, field, topics: std::collections::HashMap::with_capacity(3)}
    }
}
pub type TopicId = EntryId;
pub type TopicCollection = HashMap<EntryId, Topic>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    id: TopicId,
    name: String,
    learning_objectives: LearningObjCollection,
    pre_req_to: HashSet<TopicId>,
    supported_by: HashSet<TopicId>,
}

impl Topic {
    pub fn init(id: TopicId, name: String) -> Self {
        Self {
            id,
            name,
            learning_objectives: HashMap::new(),
            pre_req_to: HashSet::new(),
            supported_by: HashSet::new(),
        }
    }
}

pub type LearningObjCollection = HashMap<EntryId, Topic>;
#[derive(Debug, Serialize, Deserialize)]
pub enum Field {
    ComputerScience
}

pub type LearningObjId = EntryId;
#[derive(Debug, Serialize, Deserialize)]
pub struct LearningObj {
    id: LearningObjId,
    name: String,
    instructions: String
}

impl LearningObj {
    pub fn init(id: LearningObjId, name: String, instructions: String) -> Self {
        Self {
            id,
            name,
            instructions
        }
    }
}
