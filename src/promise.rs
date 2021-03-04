
use rand::Rng;

// use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

trait Individual : Clone {
    fn fitness(&self) -> f64;
}

trait IndividualFuture<T> where T: Individual {
    type Output;
    fn poll_s(&mut self, sched: &mut IndividualScheduler<T>) -> Poll<Self::Output>;
}


trait Future {
    type Output;

    fn poll(&mut self, cx: &Context) -> Poll<Self::Output>;
    fn poll_n(&mut self) -> Poll<Self::Output>;
    fn poll_s<S: Individual>(&mut self, sched: &mut IndividualScheduler<S>)-> Poll<Self::Output>;
}

#[derive(Default)]
struct MyFuture {
    id: u32,
    count: u32,
}

impl MyFuture { 
  pub fn new(id: u32) -> Self { 
    Self { id: id, count: 0 }
  }
}


struct EAFuture<T>  where T: Individual { 
    // can input be removed? 
    input: T,
    result: f64,
    job_id: u32,
}

impl<T> EAFuture<T> where T: Individual {
    fn new(input: T) -> Self {
	Self { input: input, result: 0.0, job_id: 0 }
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

    pub fn schedule_job(&mut self, job_info: T) -> EAFuture<T> {
	let mut f = EAFuture::new(job_info.clone());
	f.set_id(self.input.len() as u32);
	self.output.push(None);
	self.input.push(job_info);
	return f;
    }
   
    pub fn do_handling(&self, f: &mut EAFuture<T>) {
	f.result = self.output.len() as f64;
    }

    // should f be mut ? 
    pub fn get_result(&self, f: &EAFuture<T>) -> Option<f64> {
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
}

impl<T> IndividualFuture<T> for EAFuture<T>
where
    T: Individual
{
    type Output = f64;

    fn poll_s(&mut self, sched: &mut IndividualScheduler<T>) -> Poll<Self::Output> {
	match sched.get_result(&self) {
	    Some(t) => {
		Poll::Ready(t)
	    },
	    None => { Poll::Pending },
	}
    }
}

// scheduler.
fn run_many<F>(mut f: Vec::<F>) -> Vec::<F::Output>
where
    F: Future,
{
    // create beanstalkd connection.
    // let mut scheduler = Beanstalkd::new();
    // for i in f.iter_mut() {
    //     scheduler.schedule_job(&mut i);
    // }

    let mut results = Vec::<F::Output>::new();
    while results.len() < f.len() { 
        for i in f.iter_mut() {
            match i.poll_n() { 
		Poll::Ready(val) => { 
                    results.push(val);
		}, 
		Poll::Pending => {
                    println!("Waiting around");
		}
            }
	}
    }
    return results;
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

	for i in 0..10 { 
	    match p.poll_s(&mut sched) {
		Poll::Ready(value) => {
		    println!("Got a value: {}", value);
		},
		Poll::Pending => {
		    println!("Still waiting");
		}

	    }

	    match j.poll_s(&mut sched) {
		Poll::Ready(value) => {
		    println!("Got a value: {}", value);
		},
		Poll::Pending => {
		    println!("Still waiting");
		}
	    }
	    sched.update();
	}
	assert!(false);

    }
}
