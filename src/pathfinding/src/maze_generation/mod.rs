mod recursive_division;

use std::collections::BTreeSet;

pub use recursive_division::recursive_division;

use crate::Coord;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MazeGenerationStep {
    pub walls: BTreeSet<Coord>,
}
impl MazeGenerationStep {
    pub fn new(walls: BTreeSet<Coord>) -> Self {
        Self { walls }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MazeGenerationResult {
    pub steps: Vec<MazeGenerationStep>,
    pub walls: BTreeSet<Coord>,
}
impl MazeGenerationResult {
    pub fn new(steps: Vec<MazeGenerationStep>, walls: BTreeSet<Coord>) -> Self {
        Self { steps, walls }
    }
}
