use beanstalkc::Beanstalkc;
/// Contains structs and methods for "enqueing" arbitrary fitness functions
/// and waiting for results.
use std::time::Duration;

use crate::distro::{JobInfo, JobResults};
use crate::nn::Network;

trait IndiFit<Individual> {
    fn set_fitness(&mut self, fitness: f64) -> ();
    fn fitness(&self) -> f64;
    fn name() -> String;
}

// todo: remove T and remain Scheduler to beanstalk Scheduler
pub trait SchedulerT<Individual> {
    /// blocking call to wait around untill all the jobs are finished.
    fn wait(&mut self);
    /// Performs the act of enqueing or setting up w/e is needed for the individual
    /// to be evaluated.
    fn schedule_job(&mut self, indi: &Individual) -> u32;
    fn get_results(&self) -> f64;
}

// todo: allow for different schedule types / connectors etc.
// one main option should be have "remote workers" vs local running.
pub struct Scheduler<'a> {
    current_jobs: Vec<(u128, &'a mut Network)>,
    job_queue: Beanstalkc,
    next_job_id: u128,
}

impl<'a> Scheduler<'a> {
    // todo: allow for local where beanstalk is not used.
    pub fn new(host: &str, port: u16) -> Scheduler {
        let mut p = Beanstalkc::new()
            .host(host)
            .port(port)
            .connect()
            .expect("Connection failed");
        p.watch("results").expect("Failed to watch results queue");
        return Scheduler {
            current_jobs: vec![],
            job_queue: p,
            next_job_id: 0,
        };
    }

    /// @param: fitness_func_name name of fitness function to run.
    pub fn schedule_job(&mut self, individual: &'a mut Network, fitness_func_name: &String) -> () {
        self.job_queue
            .use_tube(&fitness_func_name)
            .expect("Failed to use tube");
        let job_id = self.next_job_id + 1 as u128;
        self.next_job_id += 1;

        let job = JobInfo {
            name: fitness_func_name.clone(),
            individual: individual.clone(),
            job_id: job_id,
        };

        let job_str = serde_json::to_string(&job).unwrap();
        match self.job_queue.put(
            job_str.as_bytes(),
            1,
            Duration::from_secs(0),
            Duration::from_secs(120),
        ) {
            Ok(_t) => self.current_jobs.push((job_id, individual)),
            Err(_) => {
                println!("Failed to schedule job")
            }
        };
    }

    pub fn wait(&mut self) -> () {
        // hold off or do w/e till scheduled items are finished.
        while self.current_jobs.len() > 0 {
            let current_job = self.job_queue.reserve_with_timeout(Duration::from_secs(2));
            match current_job {
                Ok(mut job_info) => {
                    job_info.delete().expect("Failed to delete job from queue");
                    // self.job_queue.delete(current_job.id());
                    // todo: pair fitness with the scheduled fitness items.
                    let mut i = 0;
                    let unpacked_result: JobResults =
                        serde_json::from_slice(&job_info.body()).unwrap();
                    for (index, job_r) in self.current_jobs.iter().enumerate() {
                        if unpacked_result.job_id == job_r.0 {
                            i = index;
                        }
                    }
                    let mut queued_job = self.current_jobs.remove(i);
                    queued_job.1.fitness = unpacked_result.fitness;
                }

                Err(_) => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground() {}
}
