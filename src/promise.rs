
use std::task::Poll;

use crate::scheduler::{Scheduler};

// todo: should this be generic'ed on the Output
// and store the output here?
pub struct EAFuture {
    pub result: f64,
    job_id: u32,
}

impl EAFuture {
    pub fn new(job_id: u32) -> Self {
        Self {
            result: 0.0,
            job_id: job_id,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.job_id
    }

    // todo: should self be constant?
    pub fn poll<T, S>(&mut self, sched: &mut S) -> Poll<f64>
    where
        S: Scheduler<T>,
    {
        match sched.get_result(&self) {
            Some(t) => Poll::Ready(t),
            None => Poll::Pending,
        }
    }
}

