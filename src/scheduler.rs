extern crate beanstalkd;

use beanstalkd::Beanstalkd;

use serde::{Deserialize, Serialize};
use serde_json::Result;

mod nn;
mod neat;

fn schedule_job(beanstalk: &mut Beanstalkd, job_type: String, individual: nn::Network) -> () {
    beanstalk.tube(&job_type);
    let job_str = serde_json::to_string(&individual).unwrap();
    beanstalk.put(&job_str, 1, 0, 120);
}

fn main() -> () {
    let mut beanstalkd = Beanstalkd::connect("192.168.1.77", 11300).unwrap();

    schedule_job(&mut beanstalkd, String::from("rasteroids"), nn::Network::new(16, 3, true));
}
