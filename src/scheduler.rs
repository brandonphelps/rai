/// Contains structs and methods for "enqueing" arbitrary fitness functions
/// and waiting for results.


use std::time::Duration;
use beanstalkc::Beanstalkc;

use crate::distro::{EaFuncMap, JobInfo, JobResults};
use crate::nn::Network;

trait IndiFit<Individual> {
    fn set_fitness(&mut self, fitness: f64) -> ();
    fn fitness(&self) -> f64;
    fn name() -> String;
}

// todo: remove T and remain Scheduler to beanstalk Scheduler
trait SchedulerT<'a, Individual> {
    /// blocking call to wait around untill all the jobs are finished. 
    fn wait(&mut self);
    /// Performs the act of enqueing or setting up w/e is needed for the individual
    /// to be evaluated. 
    fn schedule_job(&mut self, indi: &'a mut Individual);
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
    pub fn schedule_job(
        &mut self,
        individual: &'a mut Network,
        fitness_func_name: &String,
    ) -> () {
        self.job_queue.use_tube(&fitness_func_name).expect("Failed to use tube");
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

pub struct LocalScheduler;

impl<'a, IndivType> SchedulerT<'a, IndivType> for LocalScheduler
where
    IndivType: IndiFit<IndivType>
{
    fn schedule_job(&mut self, individual: &'a mut IndivType) {
	individual.set_fitness(individual.fitness());
    }

    fn wait(&mut self) -> () {
	// do nothing. 
    }
}

// impl<'a, Indi, FitnessFunc> SchedulerT<'a, Indi, FitnessFunc> for LocalScheduler
// where
//     Indi: IndiFit<Indi>,
//     FitnessFunc: FitnessFunctor<Indi>,
// {
//     fn schedule_job(&mut self, individual: &'a mut Indi) {
// 	// do the actual job?
// 	individual.set_fitness(FitnessFunc::eval(individual.get_individual()));
//     }

//     fn wait(&mut self) -> () {
// 	// does nothing.
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    // is it possible to pass in the scheduler? 
    fn do_stuff<Individual>(params: u8)
    where
	Individual: IndiFit<Individual> + Default,
    {
	let mut scheduler = LocalScheduler { };

	let mut holder = Vec::<Individual>::new();

	for i in 0..params {
	    holder.push(Individual::default());
	}

	for i in holder.iter_mut() { 
	    scheduler.schedule_job(&mut i);
	}
	scheduler.wait();
    }


    #[test]
    fn playground() {
	struct MathMax {
	    w1: f64,
	    x1: f64,
	}

	struct MathMaxIndi {
	    helper: MathMax,
	    fitness: f64,
	}

	impl IndiFit<MathMaxIndi> for MathMaxIndi {
	    fn fitness(&self) -> f64 {
		return self.helper.w1 * self.helper.x1;
	    }

	    fn name() -> String {
		String::from("MathMax")
	    }

	    fn set_fitness(&mut self, fit: f64) -> () {
		self.fitness = fit;
	    }
	}

	impl Default for MathMaxIndi {
	    fn default() -> Self {
		Self {
		    helper: MathMax { w1: 4.0, x1: 0.0 },
		    fitness: 0.0
		}
	    }
	}

	let mut schedul = LocalScheduler { };

	let mut p = MathMaxIndi { helper: MathMax { w1: 0.3,
						    x1: 0.0 },
				  fitness: 0.0};
	schedul.schedule_job(&mut p);

	schedul.wait();

	do_stuff::<MathMaxIndi>(100);
	
    }
}
