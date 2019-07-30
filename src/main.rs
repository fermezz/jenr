extern crate env_logger;
extern crate jenkins_api;
extern crate log;
#[macro_use] extern crate serde;
#[macro_use] extern crate failure;

use jenkins_api::client::Path;
use jenkins_api::JenkinsBuilder;
use log::info;
use std::env;
use structopt::StructOpt;

mod objects;
use objects::{BuildOfJob, RootIterator, HealthReportOfJob};

#[derive(StructOpt, Debug)]
#[structopt(name = "jenr", about = "Jenkins from the CLI!")]
enum Jenr {
    #[structopt(name = "status")]
    Status {
        /// Possible values: "job".
        object: String,
        /// The name of the object.
        name: String,
        /// Fetch last completed build.
        #[structopt(short = "c", long = "completed")]
        completed: bool,
    },

    #[structopt(name = "list")]
    List {
        /// Possible values: "jobs", "computers".
        object: String,
    },

    #[structopt(name = "health")]
    Health {
        /// Possible values: "job".
        object: String,
        /// The name of the object.
        name: String,
    },
}

fn main() -> Result<(), failure::Error> {
    // Initialising logger.
    env_logger::init();

    let key = "JENKINS_HOME";
    let jenkins_home = match env::var(key) {
        Ok(val) => val,
        Err(e)  => bail!("Couldn't interpret {}: {}.", key, e),
    };

    info!("Building Jenkins client.");
    let jenkins = JenkinsBuilder::new(&jenkins_home).build()?;

    match Jenr::from_args() {
        Jenr::Status { object, name, completed } => {
            let object_to_fetch: Path = match &object as &str {
                "job" => Path::Job {
                    name: &name,
                    configuration: None,
                },
                _ => Path::Job {
                    name: &name,
                    configuration: None,
                },
            };

            let build_kind: String;

            if completed {
                build_kind = "lastCompletedBuild".to_string()
            } else {
                build_kind = "lastBuild".to_string()
            };

            let object_tree = BuildOfJob::get_tree_for_build(&build_kind).build();

            info!("Fetching the {} named {}.", &object, &name);
            let object: BuildOfJob = jenkins.get_object_as(object_to_fetch, object_tree)?;

            object.display_status().expect("There was an error printing the status of the object.");
        }

        Jenr::List { object } => {
            let (object_to_fetch, object_name) = match &object as &str {
                "computers" => (Path::Computers, "computer"),
                "jobs" => (Path::Home, "jobs"),
                _ => (Path::Home, "jobs"),
            };

            info!("Fetching the root object named {}.", &object_name);
            let tree = RootIterator::get_default_tree(&object_name).build();
            let root_iterator: RootIterator = jenkins.get_object_as(object_to_fetch, tree)?;

            root_iterator.list_items().expect("There was an error printing the list.");
        }

        Jenr::Health { object, name } => {
            let object_to_fetch = match &object as &str {
                "job" => Path::Job {
                    name: &name,
                    configuration: None,
                },
                _ => Path::Job {
                    name: &name,
                    configuration: None,
                },
            };

            info!("Fetching the health report of job named {}.", &name);
            let tree = HealthReportOfJob::get_default_tree().build();
            let health_report: HealthReportOfJob = jenkins.get_object_as(object_to_fetch, tree)?;

            health_report.display_health_report().expect("There was an error displaying the health report.");
        }
    }

    Ok(())
}
