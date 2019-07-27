extern crate env_logger;
extern crate failure;
extern crate jenkins_api;
extern crate log;
#[macro_use]
extern crate serde;

use jenkins_api::client::Path;
use jenkins_api::JenkinsBuilder;
use log::info;
use std::env;
use structopt::StructOpt;

mod objects;
use objects::{LastBuildOfJob, RootIterator};


#[derive(StructOpt, Debug)]
#[structopt(name = "jenr", about = "Jenkins from the CLI!")]
enum Jenr {
    #[structopt(name = "status")]
    Status {
        /// Possible values: "job".
        object: String,
        /// The name of the object.
        name: String,
    },

    #[structopt(name = "list")]
    List {
        /// Possible values: "jobs", "computers".
        object: String,
    },
}




fn main() -> Result<(), failure::Error> {
    env_logger::init();

    info!("Building Jenkins client.");
    let jenkins = JenkinsBuilder::new(&env::var("JENKINS_HOME").unwrap()).build()?;

    match Jenr::from_args() {
        Jenr::Status { object, name } => {
            let object_to_fetch: Path = match &object as &str {
                "job" => Path::Job { name: &name, configuration: None },
                _     => Path::Job { name: &name, configuration: None },
            };
            let object_tree = LastBuildOfJob::get_default_tree().build();

            info!("Fetching the {} named {}.", &object, &name);
            let object: LastBuildOfJob = jenkins.get_object_as(object_to_fetch, object_tree)?;

            object.display_status();
        },

        Jenr::List { object } => {
            let (object_to_fetch, object_name) = match &object as &str {
                "computers" => (Path::Computers, "computer"),
                "jobs"      => (Path::Home, "jobs"),
                _           => (Path::Home, "jobs"),
            };

            info!("Fetching the root object named {}.", &object_name);
            let tree = RootIterator::get_default_tree(&object_name).build();
            let root_iterator: RootIterator = jenkins.get_object_as(
                object_to_fetch,
                tree,
            )?;

            root_iterator.list_items();

        },
    }

    Ok(())
}
