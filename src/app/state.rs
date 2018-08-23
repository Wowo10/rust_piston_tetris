pub enum State{
    Free,
    Taken,
    Active
}

impl Copy for State {

}

impl Clone for State{
    fn clone(&self) -> State{
        *self
    }
}