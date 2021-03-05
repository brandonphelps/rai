
use rand::Rng;
use std::time::Duration;
use beanstalkc::Beanstalkc;

use serde::{Deserialize, Serialize};

// use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Serialize, Deserialize)]
struct JobInfo<T>
where
    T: Individual
{
    name: String,
    individual: T,
    job_id: u128,
}

trait Individual : Clone {
    fn fitness(&self) -> f64;
    fn ea_name(&self) -> String;
}

// todo: should this be generic'ed on the Output
// and store the output here? 
struct EAFuture { 
    result: f64,
    job_id: u32,
}

impl EAFuture { 
    fn new(job_id: u32) -> Self {
	Self { result: 0.0, job_id: job_id }
    }

    pub fn get_id(&self) -> u32 {
	self.job_id
    }

    pub fn poll<T: Individual>(&mut self, sched: &mut LocalScheduler<T>) -> Poll<f64> {
	match sched.get_result(&self) {
	    Some(t) => { Poll::Ready(t) },
	    None => { Poll::Pending },
	}
    }
}

/// a scheduler main goal is provide single thread asycn like behaviour.
/// this occurs in a similar fasion to futures, however allows for maintaing state
/// when a schedule_job event occurs a handle (EAFturue) is passed to the user that
/// will (after enough update) calls be able to retrieve its value via the poll method
/// care must be taken for that th eEAfuture polls the scheduler it came from. 
trait Scheduler<T> {
    fn schedule_job(&mut self, job_info: T) -> EAFuture;
    fn update(&mut self) -> ();
    fn wait(&mut self) -> ();
}

struct LocalScheduler<T> where T: Individual
{
    input: Vec<T>,
    output: Vec<Option<f64>>,
}

impl<T> LocalScheduler<T> where T: Individual {
    pub fn new() -> Self {
	Self {
	    input: vec![],
	    output: vec![],
	}
    }

    // should f be mut ? 
    pub fn get_result(&self, f: &EAFuture) -> Option<f64> {
	if f.get_id() as usize >= self.output.len() {
	    None
	} else { 
	    self.output[f.get_id() as usize]
	}
    }
}

impl<T> Scheduler<T> for LocalScheduler<T> where T: Individual {

    fn schedule_job(&mut self, job_info: T) -> EAFuture {
	let mut f = EAFuture::new(self.input.len() as u32);
	self.output.push(Some(job_info.fitness()));
	self.input.push(job_info);
	return f;
    }

    fn update(&mut self) {
	// no need for this. 
    }

    /// @brief does blocking until all associated futures are completed. 
    fn wait(&mut self) {
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

struct BeanstalkScheduler<T>
where
    T: Individual
{
    current_jobs: Vec<u128>,
    job_queue: Beanstalkc,
    next_job_id: u128,
    output_values: Vec<T>,
}

impl<T> BeanstalkScheduler<T> where T: Individual {
    pub fn new(host: &str, port: u16) -> Self {
	let mut p = Beanstalkc::new().host(host).port(port).connect().expect("Connection failed");
	p.watch("results").expect("Failed to watch results queue");

	Self {
	    current_jobs: vec![],
	    job_queue: p,
	    next_job_id: 0,
	    output_values: vec![],
	}
    }
}

impl<T> Scheduler<T> for BeanstalkScheduler<T>
where
    T: Individual + Serialize
{
    fn schedule_job(&mut self, job_info: T) -> EAFuture {
	self.job_queue.use_tube(&job_info.ea_name()).expect("Failed to use tube");
	let job_id = self.next_job_id + 1 as u128;
	self.next_job_id += 1;

	let job = JobInfo {
	    name: job_info.ea_name().clone(),
	    individual: job_info.clone(),
	    job_id: job_id,
	};

	let job_str = serde_json::to_string(&job).unwrap();

	match self.job_queue.put(
	    job_str.as_bytes(),
	    1,
	    Duration::from_secs(0),
	    Duration::from_secs(120),
	) {
	    Ok(_t) => { self.current_jobs.push((job_id, job_info.clone())) },
	    Err(_) => {
		println!("Failed to schedule Job");
	    }
	};

	EAFuture::new(job_id as u32)
    }

    fn update(&mut self) -> () {

    }

    fn wait(&mut self) {
	while self.current_jobs.len() > {

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

	    fn ea_name(&self) -> String {
		String::from("String")
	    }
	}

	let mut sched = LocalScheduler::<String>::new();
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
