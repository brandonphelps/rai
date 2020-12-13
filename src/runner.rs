extern crate beanstalkd;

use beanstalkd::Beanstalkd;

mod distro;
mod nn;
mod neat;


fn main() -> () {
    println!("Hello world");
    let mut beanstalkd = Beanstalkd::connect("192.168.1.77", 11300).unwrap();

    beanstalkd.watch("rasteroids").unwrap();
    while true { 

	let p = beanstalkd.reserve().unwrap();
	beanstalkd.delete(p.0).unwrap();

	let job_str = p.1.as_str();
	let mut result: distro::JobInfo  = serde_json::from_str(&job_str).unwrap();
	println!("{:#?}", result);

	// todo: either needs a way to touch the job or the timeout must be large. 
	// or maybe run the job touch in another thread? 
	distro::EaFuncMap::do_func(&result.name, &mut result.individual);

	println!("Fitness of individual: {}", result.individual.fitness());

	let result = distro::JobResults { job_id: result.job_id,
					  fitness: result.individual.fitness() };

	let result_str = serde_json::to_string(&result).unwrap();
	match beanstalkd.put(&result_str, 1, 0, 120) {
	    Ok(_t) => println!("Posted results"),
	    Err(_) => println!("Failed to post results"),
	};
    }
}
