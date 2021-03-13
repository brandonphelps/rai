use beanstalkc::Beanstalkc;
use core::time::Duration;

use std::str;

mod utils;
mod distro;
mod neat;
mod nn;
mod individual;
mod asteroids_individual;

use crate::distro::{JobInfo, JobResults};
use crate::individual::{Individual};
use serde_json;

use serde::{Deserialize};

use crate::asteroids_individual::{AsteroidsPlayer};
use crate::nn::Network;

use toml::Value;
use std::fs;

#[derive(Deserialize)]
struct TempReserver  {
    name: String,
    individual: serde_json::Value,
    job_id: u128,
}


fn resolve_result(job_body: &[u8]) -> impl Individual {
    let tmp_p: TempReserver =  match serde_json::from_slice(&job_body) {
	Ok(r) => { r },
	Err(t) => { panic!("Failed") },
    };

    AsteroidsPlayer { brain: Network::new(8, 3, true) }
}

fn main() -> () {
    let config_file = fs::read_to_string("config.toml").expect("Failed to read configuration from: 'config.toml'");
    let configuration = config_file.parse::<Value>().expect("Failed to parse configuration file");

    let beanstalk_host = configuration["runner"]["host"].as_str().unwrap();
    let beanstalk_port = match configuration["runner"]["port"] {
	Value::Integer(t) => { t as u16 },
	_ => { panic!("Invalid value type for port") }
    };

    let mut beanstalkd = Beanstalkc::new()
        .host(beanstalk_host)
        .port(beanstalk_port)
        .connect()
        .expect("Failed to connect to beanstalkd server");

    beanstalkd.watch("rasteroids").unwrap();
    beanstalkd.use_tube("results").expect("Failed to watch results tube");

    loop {
        let mut current_job = beanstalkd.reserve().unwrap();

        let job_str = current_job.body();

	// todo: if name is rasters then JobInfo must be of AsteroidsPlayer
	// i.e must do dynamic dispatch on the JobInfo type parameter. 
        let result: JobInfo<AsteroidsPlayer> = match serde_json::from_slice(&job_str) {
            Ok(r) => r,
            Err(t) => {
                println!("Got an err on str: {}", str::from_utf8(&job_str).unwrap());
                panic!("{}", t);
            }
        };

        // todo: either needs a way to touch the job or the timeout must be large.
        // or maybe run the job touch in another thread?
        // distro::EaFuncMap::do_func(&result.name, &result.individual);

        current_job.delete().expect("Failed remove job");
	let fitness = result.individual.fitness();

        println!("Fitness of individual: {}", fitness);

        let result = JobResults {
            job_id: result.job_id,
            fitness: fitness,
        };

        let result_str = serde_json::to_string(&result).unwrap();
        match beanstalkd.put(
            result_str.as_bytes(),
            1,
            Duration::from_secs(0),
            Duration::from_secs(120),
        ) {
            Ok(_t) => { },
            Err(_) => println!("Failed to post results"),
        };
    }
}
