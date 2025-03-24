#[cfg(test)]
mod e2e_tests;

use crate::{
    grid::ConstantSizeGrid2D,
    interface::{
        GridInterface, TileInterface, WaveFunctionCollapse, WaveFunctionCollapseInterruption,
    },
    space::Location2D,
};

impl<const W: usize, const H: usize> WaveFunctionCollapse<Location2D> for ConstantSizeGrid2D<W, H> {
    fn find_lowest_entropy(&mut self) -> Option<Location2D> {
        self.get_lowest_entropy_position()
    }

    fn collapse(
        &mut self,
        position: Location2D,
    ) -> Result<(), crate::interface::WaveFunctionCollapseInterruption<Location2D>> {
        self.with_tile(position, |tile| tile.collapse())
            .flatten()
            .ok_or(WaveFunctionCollapseInterruption::Contradiction(position))?;

        // TODO: Propagate

        Ok(())
    }

    fn tick(&mut self) -> crate::interface::TickResult<Location2D> {
        let lowest_entropy = self
            .find_lowest_entropy()
            .ok_or(WaveFunctionCollapseInterruption::Finished::<Location2D>)?;

        self.collapse(lowest_entropy)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn find_lowest_entropy_sanity() {
        const W: usize = 2;
        const H: usize = 2;
        let mut grid: ConstantSizeGrid2D<W, H> = ConstantSizeGrid2D::new(HashSet::from([1, 2, 3]));

        let lowest_entropy_location = Location2D { x: 0, y: 1 };
        assert_eq!(
            grid.get_tile(lowest_entropy_location)
                .unwrap()
                .possible_states()
                .count(),
            3
        );

        grid.with_tile(lowest_entropy_location, |t| {
            t.set_possible_states(HashSet::from([1, 2]))
        });

        assert_eq!(
            grid.get_tile(lowest_entropy_location)
                .unwrap()
                .possible_states()
                .count(),
            2
        );

        let implementation = grid.find_lowest_entropy().unwrap();
        assert_eq!(lowest_entropy_location, implementation);
    }
}
