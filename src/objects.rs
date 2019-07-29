use jenkins_api::client::TreeBuilder;
use std::time::Duration;
extern crate failure;

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
    pub fn list_items(self) -> Result<(), failure::Error> {
        for named_object in self.named_objects {
            let display_name = named_object.display_name.unwrap_or("None".to_string());
            println!("{}", display_name);
        }
        Ok(())
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
    pub fn display_status(self) -> Result<(), failure::Error> {
        let duration = Duration::from_millis(self.last_build.duration as u64);

        let display_name = self.display_name.unwrap_or("None".to_string());
        let result = self.last_build.result.unwrap_or("UNFINISHED".to_string());
        let url = self.last_build.url.unwrap_or("None".to_string());

        println!(
            "----------\nLast build\n----------\n\nJOB: {}\nNUMBER: {}\nRESULT: {}\nDURATION: {}s\nURL: {}",
            display_name,
            self.last_build.number,
            result,
            duration.as_secs(),
            url,
        );

        Ok(())
    }

    pub fn get_default_tree() -> TreeBuilder {
        TreeBuilder::new().with_field("displayName").with_field(
            TreeBuilder::object("lastBuild")
                .with_subfield("duration")
                .with_subfield("number")
                .with_subfield("result")
                .with_subfield("url"),
        )
    }
}
