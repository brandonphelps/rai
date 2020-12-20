extern crate beanstalkd;

use beanstalkd::Beanstalkd;
use beanstalkc::Beanstalkc;
use core::time::Duration;

mod distro;
mod nn;
mod neat;


fn main() -> () {
    let mut beanstalkd = Beanstalkc::new().host("192.168.1.77").port(11300).connect().unwrap();

    beanstalkd.watch("rasteroids").unwrap();
    while true { 

	let mut current_job = beanstalkd.reserve().unwrap();

	let job_str = current_job.body();
	let mut result: distro::JobInfo  = serde_json::from_slice(&job_str).unwrap();

	// todo: either needs a way to touch the job or the timeout must be large. 
	// or maybe run the job touch in another thread? 
	distro::EaFuncMap::do_func(&result.name, &mut result.individual);

	current_job.delete();

	println!("Fitness of individual: {}", result.individual.fitness());

	let result = distro::JobResults { job_id: result.job_id,
					  fitness: result.individual.fitness() };

	let result_str = serde_json::to_string(&result).unwrap();
	match beanstalkd.put(result_str.as_bytes(), 1, Duration::from_secs(0), Duration::from_secs(120)) {
	    Ok(_t) => println!("Posted results"),
	    Err(_) => println!("Failed to post results"),
	};
	
    }
}