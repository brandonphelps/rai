
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

	if self.output.len() as u32 >= f.get_id() {
	    return None;
	}

	return self.output[f.get_id() as usize];
    }

    pub fn update(&mut self) {
	let mut rng = rand::thread_rng();
	for (index, i) in self.input.iter().enumerate() {
	    if rng.gen::<f64>() < 0.5 {
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
	    Some(t) => { Poll::Ready(t) },
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


pub trait Promise {
    type Output;

    // returns a Filled out Option if done else None
    fn poll(&self, sched: &mut Sched) -> Poll<Self::Output>;
}

/// @brief a non async job process handler interface
/// is intended to for loading up jobs then waiting for them to complete
pub trait Scheduler<F: Promise>  {

    /// @brief indicates to the schedule that processing should occur. 
    fn schedule_job(&mut self, input: F) -> F::Output;

    /// @brief ensures that all currently scheduled jobs are completed
    /// blocking
    fn wait(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn playground() {
	// impl Future for JobFuture {
	//     type Output = f64;

	//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
	// 	Poll::Ready(self.blah)
	//     }
		
	// }

	// impl Promise for JobFuture {
	//     type Output = f64;
	//     fn poll(&self, scheduler: &mut Sched) -> Poll<f64> {
	// 	match scheduler.get_result(self.id) {
	// 	    Some(value) => {
	// 		Poll::Ready(value)
	// 	    },
	// 	    None => {
	// 		scheduler.update();
	// 		Poll::Pending
	// 	    }
	// 	}
	//     }
	// }

	// impl Scheduler<JobFuture> for Sched { 
	//     fn schedule_job(&mut self, job_info: JobFuture) -> JobFuture::Output {
	// 	JobFuture { id: self.schedule_job_f(job_info.clone()), blah: 0.3 }
	//     }

	//     fn wait(&mut self) {
	// 	// call update
	// 	let mut do_we_need_to_update = true;
	// 	while do_we_need_to_update {
	// 	    do_we_need_to_update = false;
	// 	    for i in self.jobs.iter() {
	// 		match i.job_state {
	// 		    JobState::InProgress() => {
	// 			do_we_need_to_update = true;
	// 			break;
	// 		    },
	// 		    _ => {},
	// 		}
	// 	    }

	// 	    if do_we_need_to_update {
	// 		self.update();
	// 	    }
	// 	}
	//     }
	// }

	
	// let mut sched = Sched::new();

	// // let mut p = JobResult::new(&sched);

	// let job_one = sched.schedule_job(&String::from("hello"));
	// let job_two = sched.schedule_job(&String::from("Wakakakakaka"));

	// assert!(job_one.poll(&mut sched).is_pending());

	// sched.wait();
	// // all futures must be completed. 

	// //assert_eq!(sched.get_result(job_one).unwrap(), 5);
	// match job_one.poll(&mut sched) {
	//     Poll::Ready(res) => {
	// 	assert_eq!(res, 5.0);
	//     },
	//     _ => assert!(false)
	// }
	
	
	// let my_future = MyFuture::new(1);
	// let other = MyFuture::new(2);

	// println!("Output: {:#?}", run_many(vec![my_future, other]));

	impl Individual for String {
	    fn fitness(&self) -> f64 {
		self.len() as f64
	    }
	}

	let mut sched = IndividualScheduler::<String>::new();
	let mut p = sched.schedule_job(String::from("hello world"));

	match p.poll_s(&mut sched) {
	    Poll::Ready(value) => {
		println!("Got a value");
	    },
	    Poll::Pending => {
		println!("Still waiting");
	    }
	}

	for i in 0..10 { 
	    sched.update();

	    match p.poll_s(&mut sched) {
		Poll::Ready(value) => {
		    println!("Got a value");
		},
		Poll::Pending => {
		    println!("Still waiting");
		}
	    }
	}
	assert!(false);

    }
}
