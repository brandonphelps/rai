use beanstalkc::Beanstalkc;
use core::time::Duration;

use std::str;

mod utils;
mod distro;
mod neat;
mod nn;
mod individual;
mod asteroids_individual;
mod leven;
mod bana_individ;

use crate::distro::{JobInfo, JobResults};
use crate::individual::{Individual};
use serde_json;

use serde::{Deserialize};


use crate::bana_individ::BananaIndivid;
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

fn resolve_name(job_body: &[u8]) -> String {
    let tmp_p: TempReserver =  match serde_json::from_slice(&job_body) {
	Ok(r) => { r },
	Err(t) => { panic!("Failed") },
    };
    tmp_p.name
}

// welp this is lame, can't do this cause individual as an internal param that rust can't infert? lame
// fn resolve_result(job_body: &[u8]) -> Box<dyn Individual> {
//     let tmp_p: TempReserver =  match serde_json::from_slice(&job_body) {
// 	Ok(r) => { r },
// 	Err(t) => { panic!("Failed") },
//     };

//     println!("Obtained name for: {}", tmp_p.name);
//     if tmp_p.name == "rasteroids" { 
// 	let k: AsteroidsPlayer = serde_json::from_value(tmp_p.individual).unwrap();
// 	Box::new(k)
//     } else if tmp_p.name == "BANANAAS" {
// 	let k: BananaIndivid = serde_json::from_value(tmp_p.individual).unwrap();
// 	Box::new(k)
//     }
//     else {
// 	panic!("FAILED");
//     }
// }


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

	let job_type = resolve_name(&job_str);
	let mut fitness = 0.0; 
	let mut job_id  = 0;

	if job_type == String::from("rasteroids") {
	    // todo: if name is rasters then JobInfo must be of AsteroidsPlayer
	    // i.e must do dynamic dispatch on the JobInfo type parameter. 
            let result: JobInfo<AsteroidsPlayer> = match serde_json::from_slice(&job_str) {
		Ok(r) => r,
		Err(t) => {
                    println!("Got an err on str: {}", str::from_utf8(&job_str).unwrap());
                    panic!("{}", t);
		}

            };
            current_job.delete().expect("Failed remove job");
	    fitness = result.individual.fitness();
	    job_id = result.job_id;

	} else if job_type == String::from("BANANAAS") {
	    // todo: if name is rasters then JobInfo must be of AsteroidsPlayer
	    // i.e must do dynamic dispatch on the JobInfo type parameter. 
            let result: JobInfo<BananaIndivid> = match serde_json::from_slice(&job_str) {
		Ok(r) => r,
		Err(t) => {
                    println!("Got an err on str: {}", str::from_utf8(&job_str).unwrap());
                    panic!("{}", t);
		}
            };
            current_job.delete().expect("Failed remove job");
	    fitness = result.individual.fitness();
	    job_id = result.job_id;
	} else
	{
	    panic!("Unknown job type: {}", job_type);
	}

        // todo: either needs a way to touch the job or the timeout must be large.
        // or maybe run the job touch in another thread?
        // distro::EaFuncMap::do_func(&result.name, &result.individual);


        println!("Fitness of individual: {}", fitness);

        let result = JobResults {
            job_id: job_id,
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
