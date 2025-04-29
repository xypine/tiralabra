use crate::{rules::RuleSet, utils::space::Direction};

mod helpers;
pub mod overlapping_bitmap;
// pub mod overlapping_text;

pub trait TileExtractor<
    const NEIGHBOURS_PER_TILE: usize,
    TDirection: Direction<NEIGHBOURS_PER_TILE>,
>
{
    fn get_rules(&self) -> &RuleSet<NEIGHBOURS_PER_TILE, TDirection>;
}
