
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};


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


pub struct Sched {
    jobs: Vec::<JobResult<String, f64>>,
}

pub trait Promise {
    type Output;

    // returns a Filled out Option if done else None
    fn poll(&self, sched: &mut Sched) -> Poll<Self::Output>;
}

/// @brief a non async job process handler interface
/// is intended to for loading up jobs then waiting for them to complete
pub trait Scheduler<InputType, PromiseP : Promise>  {
    
    /// @brief indicates to the schedule that processing should occur. 
    fn schedule_job(&mut self, input: &InputType) -> PromiseP;

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


	impl Future for JobFuture {
	    type Output = f64;

	    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		Poll::Ready(self.blah)
	    }
		
	}

	impl Promise for JobFuture {
	    type Output = f64;
	    fn poll(&self, scheduler: &mut Sched) -> Poll<f64> {
		match scheduler.get_result(self.id) {
		    Some(value) => {
			Poll::Ready(value)
		    },
		    None => {
			scheduler.update();
			Poll::Pending
		    }
		}
	    }
	}

	impl Sched  {
	    pub fn new() -> Self {
		Self { jobs: vec![] }
	    }

	    fn schedule_job_f(&mut self, job_info: String) -> usize {
		let job_id = self.jobs.len();
		self.jobs.push(JobResult { id : job_id,
					   input: job_info,
					   val: 0.0,
					   job_state: JobState::InProgress()
		});
		return job_id;
	    }

	    pub fn update(&mut self) {
		let mut rng = rand::thread_rng();
		for i in self.jobs.iter_mut() {
		    if rng.gen::<f64>() < 0.5 {
			println!("Job moved to done");
			i.val = i.input.len() as f64;
			i.job_state = JobState::Done();
		    } else {
			println!("Job state no change");
		    }
		}
	    }
	    
	    pub fn get_result(&self, job_id: usize) -> Option<f64> {
		for i in self.jobs.iter() {
		    if job_id == i.id {
			match i.job_state { 
			    JobState::InProgress() => { 
				return None
			    },
			    JobState::Done() => {
				return Some(i.val);
			    }
			}
		    }
		}
		return None
	    }
	}

	impl Scheduler<String, JobFuture> for Sched { 
	    fn schedule_job(&mut self, job_info: &String) -> JobFuture {
		JobFuture { id: self.schedule_job_f(job_info.clone()), blah: 0.3 }
	    }

	    fn wait(&mut self) {
		// call update
		let mut do_we_need_to_update = true;
		while do_we_need_to_update {
		    do_we_need_to_update = false;
		    for i in self.jobs.iter() {
			match i.job_state {
			    JobState::InProgress() => {
				do_we_need_to_update = true;
				break;
			    },
			    _ => {},
			}
		    }


		    if do_we_need_to_update {
			println!("update call");
			self.update();
		    }
		}
	    }
	}

	
	let mut sched = Sched::new();

	// let mut p = JobResult::new(&sched);

	let job_one = sched.schedule_job(&String::from("hello"));
	let job_two = sched.schedule_job(&String::from("Wakakakakaka"));

	assert!(job_one.poll(&mut sched).is_pending());

	sched.wait();
	// all futures must be completed. 

	//assert_eq!(sched.get_result(job_one).unwrap(), 5);
	match job_one.poll(&mut sched) {
	    Poll::Ready(res) => {
		assert_eq!(res, 5.0);
	    },
	    _ => assert!(false)
	}


	assert!(false);

    }
}
