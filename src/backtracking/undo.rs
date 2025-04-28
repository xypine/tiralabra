use crate::{tile::Tile, utils::space::Location2D};

pub struct BacktrackingByUndo {
    /// Keeps history of tile modifications that we can undo and then ban
    pub update_log: Vec<(Location2D, Tile)>,
}
