pub trait Individual: Clone {
    // can this return just a numeric traited instance?
    // post calculated fitness.
    fn fitness(&self) -> f64;
    fn ea_name(&self) -> String;
    fn mutate<Storage>(&self, stro: &mut Storage) -> Self;
    fn crossover<Storage>(&self, other: &Self,
			  stro: &mut Storage) -> Self;
}
