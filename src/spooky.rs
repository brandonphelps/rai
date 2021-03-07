

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}
trait PrinTer {
    fn print_er(&self);
}

// Because we would otherwise have to express this as `T: Debug` or 
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T where
// todo: what does this option<T> thing dosih? i kind aget it but i don't get
// why thats do able. 
    Option<T>: Debug {
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}


impl<T> PrinTer for T where
    T: Debug {
    fn print_er(&self) {
	println!("{:#?}", self);
    }
}



let vec = vec![1, 2, 3];

vec.print_in_option();

let p = 10;
p.print_in_option();

p.print_er();

struct H {
    i: u32,
}

impl H {
    pub fn printme(&self) -> {
	println!("{}", self.i);
    }
}



impl Debug for H {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	println!("H: {}", self.i);
	Ok(())
    }
}

let k = H { i: 100};

k.print_in_option();
