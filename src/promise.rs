
use std::future::Future;


pub trait Promise {
    type Output;

    // returns a Filled out Option if done else None
    fn is_done(&self) -> Option<&Self::Output>;
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn playground() {
	#[derive(Debug)]
	enum JobState {
	    InProgress(),
	    Done(),
	}


	struct JobResult<Output> {
	    id: usize,
	    job_state: JobState,
	    input: String,
	    // length of string
	    val: Output
	}

	struct JobFuture {
	    id: usize,
	}
	
	impl JobFuture {
	    pub fn poll(&self, scheduler: &Sched) -> Option<f64> {
		scheduler.get_result(self.id)
	    }
	}

	struct Sched {
	    jobs: Vec::<JobResult<f64>>,
	}

	impl Sched { 

	    pub fn new() -> Self {
		Self { jobs: vec![] }
	    }

	    pub fn schedule_job(&mut self, job_info: String) -> JobFuture {
		JobFuture { id: self.schedule_job_f(job_info) }
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

	    pub fn wait(&mut self) {
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

	
	let mut rng = rand::thread_rng();
	let mut sched = Sched::new();

	// let mut p = JobResult::new(&sched);

	let job_one = sched.schedule_job(String::from("hello"));
	let job_two = sched.schedule_job(String::from("Wakakakakaka"));

	assert!(job_one.poll(&sched).is_none());

	sched.wait();
	// all futures must be completed. 

	//assert_eq!(sched.get_result(job_one).unwrap(), 5);
	assert_eq!(job_one.poll(&sched).unwrap(), 5.0);
	assert_eq!(job_two.poll(&sched).unwrap(), 12.0);

	assert!(false);

    }
}
