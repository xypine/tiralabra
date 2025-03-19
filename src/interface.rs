use std::collections::HashMap;

pub trait TileInterface<State, TCoords> {
    fn possible_states(&self) -> impl Iterator<Item = State>;
    fn possible_states_ref<'a>(&'a self) -> impl Iterator<Item = &'a State>
    where
        State: 'a;
}

pub trait Location<const DIMENSIONS: usize> {}
pub trait Direction<const COUNT: usize> {}

pub trait GridInterface<
    const DIMENSIONS: usize,
    TState,
    TPosition,
    TDirection: Direction<{ DIMENSIONS * 2 }>,
    T: TileInterface<TState, TPosition>,
>
{
    fn image(&self) -> HashMap<TPosition, T>;
    fn get_tile(&self, location: TPosition) -> Option<T>;
    fn get_neighbours(&self, location: TPosition) -> [(TDirection, Option<T>); DIMENSIONS * 2];
}
