pub trait Individual: Clone {
    // can this return just a numeric traited instance?
    // post calculated fitness.
    
    // specific type to use within the context of mutate
    // or cross, put w/e you want here 
    type Storage;

    fn fitness(&self) -> f64;
    fn ea_name(&self) -> String;
    fn mutate(&self, stro: &mut Self::Storage) -> Self;
    fn crossover(&self, other: &Self,
			  stro: &mut Self::Storage) -> Self;
}
