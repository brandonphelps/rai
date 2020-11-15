pub trait Individual {
    // can this return just a numeric traited instance?
    // post calculated fitness.
    fn fitness(&self) -> f64;
    fn update_fitness(&mut self) -> ();
    fn print(&self) -> ();
    fn mutate(&mut self) -> ();
    // fn crossover(&self, other: Box<dyn Individual>) -> Box<dyn Individual>;
}

pub trait Crossover<Rhs = Self> {
    type Output;

    fn crossover(&self, rhs: &Rhs) -> Self::Output;
}
