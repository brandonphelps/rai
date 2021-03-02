
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


	struct JobResult {
	    id: u8,
	    job_state: JobState,
	    input: String,
	    // length of string
	    val: u8
	}

	struct 
	
	struct Sched {
	    jobs: Vec::<JobResult>,
	}

	impl Sched {
	    pub fn new() -> Self {
		Self { jobs: vec![] }
	    }

	    pub fn schedule_job(&mut self, job_info: String) -> u8 {
		let job_id = self.jobs.len() as u8;
		self.jobs.push(JobResult { id : job_id,
					   input: job_info,
					   val: 0,
					   job_state: JobState::InProgress() });
		return job_id;
	    }

	    pub fn get_job_state(&self, job_id: u8) -> Option<&JobState> {
		for i in self.jobs.iter() {
		    if job_id == i.id {
			return Some(&i.job_state);
		    }
		}
		return None;
	    }

	    pub fn update(&mut self) {
		println!("Update");
		let mut rng = rand::thread_rng();
		for i in self.jobs.iter_mut() {
		    if rng.gen::<f64>() < 0.5 {
			println!("Job moved to done");
			i.val = i.input.len() as u8;
			i.job_state = JobState::Done();
		    } else {
			println!("Job state no change");
		    }
		}
	    }

	    pub fn get_result(&self, job_id: u8) -> Option<u8> {
		for i in self.jobs.iter() {
		    if job_id == i.id {
			return Some(i.val);
		    }
		}
		return None
	    }

	    pub fn wait(&mut self) {
		// call update
		let mut do_we_need_to_update = false;
		while !do_we_need_to_update {
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
			self.update();
		    }
		}
	    }
	}

	// struct JobResult<'a> {
	//     job_state: JobState,
	//     result: u8,
	//     sched_p: &'a Sched
	// }

	// impl<'a> JobResult<'a> {
	//     pub fn new(sched: &'a Sched) -> Self {
	// 	Self { job_state: JobState::InProgress(),
	// 	       result: 0,
	// 	       sched_p: sched
	// 	}
	//     }

	//     pub fn set_result(&mut self, res: u8) -> () {
	// 	self.result = res;
	// 	self.job_state = JobState::Done();
	//     }

	//     pub fn get_result(&self) -> u8 {
	// 	self.result
	//     }
	// }

	// impl<'a> Promise for JobResult<'a> {
	//     type Output = JobResult<'a>;

	//     fn is_done(&self) -> Option<&JobResult<'a> > {
	// 	println!("Job state: {:#?}", self.job_state);
	// 	match self.job_state {
	// 	    JobState::InProgress() => {
	// 		self.sched_p.do_me(&self);
	// 		None
	// 	    },
	// 	    JobState::Done() => {
	// 		Some(self)
	// 	    }
	// 	}
	//     }
	// }

	let mut rng = rand::thread_rng();
	let mut sched = Sched::new();

	// let mut p = JobResult::new(&sched);

	let job_one = sched.schedule_job(String::from("hello"));

	assert!(sched.get_job_state(job_one).is_some());
	assert!(sched.get_job_state(job_one).is_some());

	sched.wait();

	assert_eq!(sched.get_result(job_one).unwrap(), 5);

	assert!(false);

	// assert!(p.is_done().is_none());
	// p.set_result(3);
	// assert!(p.is_done().is_some());
    }
}
