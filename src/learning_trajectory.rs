use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::hash::Hasher;

/// Captures the pathway through a curriculum. A university intro to CS
/// course might have Sequence, Repetition, and Conditionals as LT's.
pub struct LearningTrajectory {
    title: String,
    description: String,
    consensus_goals: Vec<ConsensusGoal>,
    learning_goals: Vec<LearningGoal>,
    edges: Vec<ConsensusEdge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsensusEdge {
    pub id: usize,
    pub label: String,
    pub left: usize,
    pub right: usize,
    pub weight: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConsensusGoal {
    pub id: usize,
    pub plugged: bool,
    pub st8mnt: String,
    pub weight: f32,
}

pub type CGGraph = (Vec<ConsensusGoal>, Vec<ConsensusEdge>);

/// "Any explicit statement or implicitendorsement of what students can or
/// should be able to do in relation tocomputational thinking" K. M. Rich
/// The author of this paper began with collecting a set of these.
pub struct LearningGoal {
    statement: String,
    /// Extent to which this goal is justified by student engagement
    student_support: f32,
    /// Extent to which this goal is justified through academic theory.
    academic_support: f32,
    consensus_goal: Option<ConsensusGoal>,
    concept: ConceptCluster,
}

pub enum ConceptCluster {
    ConditionalLogic,
    FlowControl,
    IterRecParThinking,
}
