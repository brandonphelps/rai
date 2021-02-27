use beanstalkc::Beanstalkc;
use core::time::Duration;

use std::str;

mod distro;
mod neat;
mod nn;

fn main() -> () {
    let mut beanstalkd = Beanstalkc::new()
        .host("192.168.1.77")
        .port(11300)
        .connect()
        .expect("Failed to connect to beanstalkd server");

    beanstalkd.watch("rasteroids").unwrap();
    beanstalkd.use_tube("results").expect("Failed to watch results tube");
    loop {
        let mut current_job = beanstalkd.reserve().unwrap();

        let job_str = current_job.body();

        let mut result: distro::JobInfo = match serde_json::from_slice(&job_str) {
            Ok(r) => r,
            Err(t) => {
                println!("Got an err on str: {}", str::from_utf8(&job_str).unwrap());
                panic!(t);
            }
        };

        // todo: either needs a way to touch the job or the timeout must be large.
        // or maybe run the job touch in another thread?
        distro::EaFuncMap::do_func(&result.name, &mut result.individual);

        current_job.delete().expect("Failed remove job");

        println!("Fitness of individual: {}", result.individual.fitness());

        let result = distro::JobResults {
            job_id: result.job_id,
            fitness: result.individual.fitness(),
        };

        let result_str = serde_json::to_string(&result).unwrap();
        match beanstalkd.put(
            result_str.as_bytes(),
            1,
            Duration::from_secs(0),
            Duration::from_secs(120),
        ) {
            Ok(_t) => println!("Posted results"),
            Err(_) => println!("Failed to post results"),
        };
    }
}
