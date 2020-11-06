

struct Instruction {

}


pub struct Program {
    // points to the next instruction to be evaluated. 
    program_counter: u64,
    program_instruction: Vec<Instruction>,
        
}

impl Program {

    // pub fn new() -> Program {
    //     return Program{program_counter: 0, program_instruction: Vec::new()};
    // }

    // pub fn step(&mut self, dt: f64) {

    //     self.eval(self.program_instruction[self.program_counter as usize]);

    //     self.program_counter += 1;
    // }

    
    // fn eval(&mut self, op: Instruction) -> () {
    //     println!("Evaluate!");
    // }
}
