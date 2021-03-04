
use rand::Rng;

// use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

trait Individual : Clone {
    fn fitness(&self) -> f64;
}

trait IndividualFuture<T> where T: Individual {
    type Output;
    fn poll(&mut self, sched: &mut IndividualScheduler<T>) -> Poll<Self::Output>;
}

// todo: should this be generic'ed on the Output
// and store the output here? 
struct EAFuture { 
    result: f64,
    job_id: u32,
}

impl EAFuture { 
    fn new() -> Self {
	Self { result: 0.0, job_id: 0 }
    }
    
    pub fn set_id(&mut self, id: u32) {
	self.job_id = id;
    }

    pub fn get_id(&self) -> u32 {
	self.job_id
    }
}

struct IndividualScheduler<T> where T: Individual
{
    input: Vec<T>,
    output: Vec<Option<f64>>,
}

impl<T> IndividualScheduler<T> where T: Individual {

    pub fn new() -> Self {
	Self {
	    input: vec![],
	    output: vec![],
	}
    }

    pub fn schedule_job(&mut self, job_info: T) -> EAFuture {
	let mut f = EAFuture::new();
	f.set_id(self.input.len() as u32);
	self.output.push(None);
	self.input.push(job_info);
	return f;
    }

    // should f be mut ? 
    pub fn get_result(&self, f: &EAFuture) -> Option<f64> {
	if f.get_id() as usize >= self.output.len() {
	    None
	} else { 
	    self.output[f.get_id() as usize]
	}
    }

    pub fn update(&mut self) {
	let mut rng = rand::thread_rng();
	for (index, i) in self.input.iter().enumerate() {
	    if rng.gen::<f64>() < 0.3 {
		self.output[index] = Some(i.fitness());
	    }
	}
    }

    /// @brief does blocking until all associated futures are completed. 
    pub fn wait(&mut self) {
	let mut do_we_need_to_update = true;
	while do_we_need_to_update {
	    do_we_need_to_update = false;
	    for i in self.output.iter() {
		if i.is_none() {
		    do_we_need_to_update = true;
		    break;
		}
	    }

	    if do_we_need_to_update {
		self.update();
	    }
	}
    }
}

impl<T> IndividualFuture<T> for EAFuture
where
    T: Individual
{
    type Output = f64;

    fn poll(&mut self, sched: &mut IndividualScheduler<T>) -> Poll<Self::Output> {
	match sched.get_result(&self) {
	    Some(t) => {
		Poll::Ready(t)
	    },
	    None => { Poll::Pending },
	}
    }
}

enum JobState {
    InProgress(),
    Done(),
}

#[derive(Debug)]
struct JobFuture {
    id: usize,
    blah: f64
}

// specifically not public
struct JobResult<Input, Output> {
    id: usize,
    job_state: JobState,
    input: Input,
    // length of string
    val: Output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn playground() {
	impl Individual for String {
	    fn fitness(&self) -> f64 {
		self.len() as f64
	    }
	}

	let mut sched = IndividualScheduler::<String>::new();
	let mut p = sched.schedule_job(String::from("hello world"));
	let mut j = sched.schedule_job(String::from("hello wakakwaka"));

	sched.wait();

	// all associated futures must be completed since we did a wait. 
	match p.poll(&mut sched) {
	    Poll::Ready(value) => {
		println!("Got a value: {}", value);
		assert!(true);
	    },
	    Poll::Pending => {
		println!("Still waiting");
		assert!(false);
	    }

	}	    
	match j.poll(&mut sched) {
	    Poll::Ready(value) => {
		println!("Got a value: {}", value);
		assert!(true);
	    },
	    Poll::Pending => {
		println!("Still waiting");
		assert!(false);
	    }
	}
	assert!(false);
    }
}
