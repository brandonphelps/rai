extern crate beanstalkd;

use beanstalkd::Beanstalkd;

mod distro;
mod nn;
mod neat;

use serde::{Deserialize, Serialize};
use serde_json::Result;


fn main() -> () {
    println!("Hello world");
    let mut beanstalkd = Beanstalkd::connect("192.168.1.77", 11300).unwrap();

    beanstalkd.watch("rasteroids");
    let p = beanstalkd.reserve().unwrap();
    beanstalkd.delete(p.0);

    let job_str = p.1.as_str();
    let result: distro::JobInfo  = serde_json::from_str(&job_str).unwrap();
    println!("{:#?}", result);
}
