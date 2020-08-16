use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

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
    name: String,
    field: Field,
    topics: HashSet<Topic>,
}

impl Subject {
    pub fn init(name: String, field: Field) -> Self {
        Self{name, field, topics: std::collections::HashSet::with_capacity(3)}
    }
    pub fn take_topic(&mut self, topic: Topic) -> bool {
        self.topics.insert(topic)
    }
}

impl PartialEq for Subject {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
impl Eq for Subject {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    name: String,
    learning_objectives: HashSet<String>,
    pre_req_to: HashSet<Topic>,
    supported_by: HashSet<Topic>
}

impl Topic {
    pub fn init(name: String) -> Self {
        Self {
            name,
            learning_objectives: HashSet::new(),
            pre_req_to: HashSet::new(),
            supported_by: HashSet::new(),
        }
    }
}

impl PartialEq for Topic {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
impl Eq for Topic {}
impl Hash for Topic {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
