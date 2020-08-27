/// Captures the pathway through a curriculum. A university intro to CS
/// course might have Sequence, Repetition, and Conditionals as LT's.
pub struct LearningTrajectory<'a> {

    title: String,
    description: String,
    consensus_goals: Vec<ConsensusGoal>,
    learning_goals: Vec<LearningGoal>,
    edges: Vec<Edge<'a>>,
}

pub struct Edge<'a> {
    left: &'a dyn Node,
    right: &'a dyn Node,
    weight: f32,
}

pub trait Node {}

impl Node for ConsensusGoal {}
pub struct ConsensusGoal {
    delivery: Delivery,
    comments: String,
    supporting_eg_LG: String,
    LearningGoals: Vec<LearningGoal>,
}



impl Node for LearningGoal {}
/// "Any explicit statement or implicitendorsement of what students can or
/// should be able to do in relation tocomputational thinking" K. M. Rich
/// The author of this paper began with collecting a set of these.
pub struct LearningGoal {
    statement: String,
    /// Extent to which this goal is justified by student engagement
    student_support: f32,
    /// Extent to which this goal is justified through academic theory.
    academic_support: f32,
    concept: ConceptCluster,
}

pub enum ConceptCluster {
    ConditionalLogic,
    FlowControl,
    IterRecParThinking,
}


pub enum Delivery {
    /// Benefit is only realized when done with a computer. E.g. writing code.
    Plugged,
    /// Can be done without technology. E.g. Describing soln.s to algo. puzzles.
    Unplugged,
}
