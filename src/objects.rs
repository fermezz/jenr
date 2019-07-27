use jenkins_api::client::TreeBuilder;
use std::time::Duration;


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NamedObject {
    display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct RootIterator {
    #[serde(alias = "computer")]
    #[serde(alias = "jobs")]
    named_objects: Vec<NamedObject>,
}


impl RootIterator {
    pub fn list_items(self) {
        for named_object in self.named_objects {
            println!("{}", named_object.display_name.unwrap())
        }
    }

    pub fn get_default_tree(object_name: &str) -> TreeBuilder {
        TreeBuilder::new().with_field(TreeBuilder::object(object_name).with_subfield("displayName"))
    }
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
pub struct LastBuildOfJob {
    display_name: Option<String>,
    last_build: LastBuild,
}

impl LastBuildOfJob {

    pub fn display_status(self) {
        let duration = Duration::from_millis(self.last_build.duration as u64);

        println!(
            "----------\nLast build\n----------\n\nJOB: {}\nNUMBER: {}\nRESULT: {}\nDURATION: {}s\nURL: {}",
            self.display_name.unwrap(),
            self.last_build.number,
            self.last_build.result.unwrap(),
            duration.as_secs(),
            self.last_build.url.unwrap(),
        );
    }

    pub fn get_default_tree() -> TreeBuilder {
        TreeBuilder::new()
           .with_field("displayName")
           .with_field(
               TreeBuilder::object("lastBuild")
                   .with_subfield("duration")
                   .with_subfield("number")
                   .with_subfield("result")
                   .with_subfield("url"),
           )
    }

}
