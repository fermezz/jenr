extern crate env_logger;
extern crate failure;
extern crate jenkins_api;
extern crate log;
#[macro_use]
extern crate serde;

use jenkins_api::client::{Path, TreeBuilder};
use jenkins_api::{JenkinsBuilder};
use log::info;
use std::env;
use std::time::Duration;
use structopt::StructOpt;


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

#[derive(Deserialize)]
struct RootIterator {
    #[serde(alias = "computer")]
    #[serde(alias = "jobs")]
    named_objects: Vec<NamedObject>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LastBuild {
    duration: u32,
    number: u32,
    result: Option<String>,
    url: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LastBuildOfJob {
    display_name: Option<String>,
    last_build: LastBuild,
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NamedObject {
    display_name: Option<String>,
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

            let object_tree = TreeBuilder::new()
                           .with_field("displayName")
                           .with_field(
                               TreeBuilder::object("lastBuild")
                                   .with_subfield("duration")
                                   .with_subfield("number")
                                   .with_subfield("result")
                                   .with_subfield("url"),
                           )
                           .build();

            info!("Fetching the {} named {}.", &object, &name);
            let object: LastBuildOfJob = jenkins.get_object_as(
                object_to_fetch,
                object_tree,
            )?;

            let duration = Duration::from_millis(object.last_build.duration as u64);

            println!(
                "----------\nLast build\n----------\n\nJOB: {}\nNUMBER: {}\nRESULT: {}\nDURATION: {}s\nURL: {}",
                object.display_name.unwrap(),
                object.last_build.number,
                object.last_build.result.unwrap(),
                duration.as_secs(),
                object.last_build.url.unwrap(),
            );

        },

        Jenr::List { object } => {
            let (object_to_fetch, object_name) = match &object as &str {
                "computers" => (Path::Computers, "computer"),
                "jobs"      => (Path::Home, "jobs"),
                _           => (Path::Home, "jobs"),
            };

            let tree = TreeBuilder::new()
                           .with_field(TreeBuilder::object(&object_name).with_subfield("displayName"))
                           .build();

            let iterator: RootIterator = jenkins.get_object_as(
                object_to_fetch,
                tree,
            )?;

            for named_object in iterator.named_objects {
                println!("{}", named_object.display_name.unwrap())
            }
        },
    }

    Ok(())
}
