use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub id: EntryId,
    pub field: Field,
    pub topics: HashMap<u32, Topic>,
}

impl Subject {
    pub fn init(id: EntryId, name: String, field: Field) -> Self {
        Self{id, name, field, topics: std::collections::HashMap::with_capacity(3)}
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    id: EntryId,
    name: String,
    learning_objectives: HashMap<u32, String>,
    pre_req_to: HashMap<u32, Topic>,
    supported_by: HashMap<u32, Topic>
}

impl Topic {
    pub fn init(id: EntryId, name: String) -> Self {
        Self {
            id,
            name,
            learning_objectives: HashMap::new(),
            pre_req_to: HashMap::new(),
            supported_by: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Field {
    ComputerScience
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LearningObj {
    name: String,
    instructions: String
}

impl LearningObj {
    pub fn init(name: String, instructions: String) -> Self {
        Self {
            name,
            instructions
        }
    }
}
