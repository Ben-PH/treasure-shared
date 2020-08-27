/// Captures the pathway through a curriculum. A university intro to CS
/// course might have Sequence, Repetition, and Conditionals as LT's.
pub struct LearningTrajectory {

    title: String,
    description: String,
    stopping_points: Vec<ConsensusGoal>,
}
pub struct ConsensusGoal {
    delivery: Delivery,
    comments: String,
    supporting_eg_LG: String,
    LearningGoals: Vec<LearningGoal>,
}

/// "Any explicit statement or implicitendorsement of what students can or
/// should be able to do in relation tocomputational thinking" K. M. Rich
/// The author of this paper began with collecting a set of these.
pub struct LearningGoal {
    statement: String,
    /// Extent to which this goal is justified by student engagement
    student_support: f32,
    /// Extent to which this goal is justified through academic theory.
    academic_support: f32,
    concept_tag: Concept,
}

pub enum Concept {
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
