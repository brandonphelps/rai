
/// Contains structs and methods for "enqueing" arbitrary fitness functions
/// and waiting for results.
use std::time::Duration;
use std::task::Poll;

use serde::{Deserialize, Serialize};

use beanstalkc::Beanstalkc;

use crate::nn::Network;
use crate::individual::Individual;

use crate::promise::EAFuture;

use crate::distro::{JobInfo, JobResults};


/// a scheduler main goal is provide single thread asycn like behaviour.
/// this occurs in a similar fasion to futures, however allows for maintaing state
/// when a schedule_job event occurs a handle (EAFturue) is passed to the user that
/// will (after enough update) calls be able to retrieve its value via the poll method
/// care must be taken for that th eEAfuture polls the scheduler it came from.
pub trait Scheduler<T> {
    fn schedule_job(&mut self, job_info: T) -> EAFuture;
    fn get_result(&self, f: &EAFuture) -> Option<f64>;
    fn update(&mut self) -> ();
    fn wait(&mut self) -> ();
    fn clear(&mut self) -> ();
}

pub struct LocalScheduler<T>
where
    T: Individual,
{
    // todo: remove this since we don't need to keep track of the input.
    input: Vec::<T>,
    output: Vec<Option<f64>>,
}

impl<T> LocalScheduler<T>
where
    T: Individual,
{
    pub fn new() -> Self {
        Self {
	    input: vec![],
            output: vec![],
        }
    }
}

impl<T> Scheduler<T> for LocalScheduler<T>
where
    T: Individual,
{
    fn schedule_job(&mut self, job_info: T) -> EAFuture {
        let f = EAFuture::new(self.output.len() as u32);
        self.output.push(Some(job_info.fitness()));
        return f;
    }

    fn update(&mut self) {
        // no need for this.
    }

    // should f be mut ?
    fn get_result(&self, f: &EAFuture) -> Option<f64> {
        if f.get_id() as usize >= self.output.len() {
            None
        } else {
            self.output[f.get_id() as usize]
        }
    }

    fn clear(&mut self) {

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

pub struct BeanstalkScheduler<T>
where
    T: Individual,
{
    // todo: remove.
    input: Vec<T>,
    // todo: replace u128 with JobId(u128)
    current_jobs: Vec<u128>,
    job_queue: Beanstalkc,
    next_job_id: u128,
    output_values: Vec<Option<f64>>,
}

impl<T> BeanstalkScheduler<T>
where
    T: Individual,
{
    pub fn new(host: &str, port: u16) -> Self {
        let mut p = Beanstalkc::new()
            .host(host)
            .port(port)
            .connect()
            .expect("Connection failed");
        p.watch("results").expect("Failed to watch results queue");

        Self {
            input: vec![],
            current_jobs: vec![],
            job_queue: p,
            next_job_id: 0,
            output_values: vec![],
        }
    }

    
}

impl<T> Scheduler<T> for BeanstalkScheduler<T>
where
    T: Individual + Serialize,
{
    fn schedule_job(&mut self, job_info: T) -> EAFuture {
        self.job_queue
            .use_tube(&job_info.ea_name())
            .expect("Failed to use tube");

        let job_id = self.next_job_id + 1 as u128;
	println!("Scheduling job: {}", job_id);
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
            Ok(_t) =>{
		self.current_jobs.push(job_id);
		self.output_values.push(None);
	    },
            Err(_) => {
                println!("Failed to schedule Job");
            }
        };

        EAFuture::new(job_id as u32)
    }

    fn update(&mut self) -> () {}

    fn get_result(&self, f: &EAFuture) -> Option<f64> {
	println!("Job ID: {}, total value: {}", f.get_id(), self.output_values.len());
        if f.get_id() as usize >= self.output_values.len() {
	    None
	} else {
	    self.output_values[f.get_id() as usize]
	}
    }

    fn clear(&mut self) {
	self.next_job_id = 0;
	self.output_values.clear();
	self.current_jobs.clear();
    }

    fn wait(&mut self) {

	let mut need_check_for_updates = true;

	while need_check_for_updates {
	    need_check_for_updates = false;

	    let current_job = self.job_queue.reserve_with_timeout(Duration::from_secs(2));
            match current_job {
		Ok(mut job_info) => {
		    job_info.delete().expect("Failed to delete job from queue");

		    let unpacked_result: JobResults =
			serde_json::from_slice(&job_info.body()).unwrap();

		    for (index, job_r) in self.current_jobs.iter().enumerate() {
			if unpacked_result.job_id == *job_r {
			    println!("Obtained fitness from worker: {}", unpacked_result.fitness);
			    self.output_values[index] = Some(unpacked_result.fitness);
			}
		    }
		},
		Err(_) => { }
	    }


	    for i in self.output_values.iter() {
		if i.is_none() {
		    need_check_for_updates = true;
		}
	    }
	}
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
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

	    fn crossover<S>(&self, other: &Self, storage: &mut S) -> Self {
		self.clone()
	    }

	    fn mutate<S>(&self, stor: &mut S) -> Self {
		self.clone()
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
            }
            Poll::Pending => {
                println!("Still waiting");
                assert!(false);
            }
        }
        match j.poll(&mut sched) {
            Poll::Ready(value) => {
                println!("Got a value: {}", value);
                assert!(true);
            }
            Poll::Pending => {
                println!("Still waiting");
                assert!(false);
            }
        }
        assert!(false);
    }
}
