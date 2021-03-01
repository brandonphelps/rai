
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

	struct Sched {
	    
	}

	struct JobResult<'a> {
	    job_state: JobState,
	    result: u8,
	    sched_p: &'a Sched
	}

	impl<'a> JobResult<'a> {
	    pub fn new(sched: &'a Sched) -> Self {
		Self { job_state: JobState::InProgress(),
		       result: 0,
		       sched_p: sched
		}
	    }

	    pub fn set_result(&mut self, res: u8) -> () {
		self.result = res;
		self.job_state = JobState::Done();
	    }

	    pub fn get_result(&self) -> u8 {
		self.result
	    }
	}

	impl<'a> Promise for JobResult<'a> {
	    type Output = JobResult<'a>;

	    fn is_done(&self) -> Option<&JobResult<'a> > {
		println!("Job state: {:#?}", self.job_state);
		match self.job_state {
		    JobState::InProgress() => {
			None
		    },
		    JobState::Done() => {
			Some(self)
		    }
		}
	    }
	}

	let sched = Sched { };

	let mut p = JobResult::new(&sched);
	let mut rng = rand::thread_rng();
	assert!(p.is_done().is_none());
	p.set_result(3);
	assert!(p.is_done().is_some());
    }
}
