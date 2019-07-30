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
struct Build {
    duration: u32,
    number: u32,
    result: Option<String>,
    url: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildOfJob {
    display_name: Option<String>,
    #[serde(alias = "lastBuild")]
    #[serde(alias = "lastCompletedBuild")]
    build: Build,
}

impl BuildOfJob {
    pub fn display_status(self) -> Result<(), failure::Error> {
        let duration = Duration::from_millis(self.build.duration as u64);

        let display_name = self.display_name.unwrap_or("None".to_string());
        let result = self.build.result.unwrap_or("UNFINISHED".to_string());
        let url = self.build.url.unwrap_or("None".to_string());

        println!(
            "----------\n  Build  \n----------\n\nJOB: {}\nNUMBER: {}\nRESULT: {}\nDURATION: {}s\nURL: {}",
            display_name,
            self.build.number,
            result,
            duration.as_secs(),
            url,
        );

        Ok(())
    }

    pub fn get_tree_for_build(build_kind: &str) -> TreeBuilder {
        TreeBuilder::new().with_field("displayName").with_field(
            TreeBuilder::object(build_kind)
                .with_subfield("duration")
                .with_subfield("number")
                .with_subfield("result")
                .with_subfield("url"),
        )
    }
}

#[derive(Deserialize)]
struct DescribedObject {
    description: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthReportOfJob {
    name: String,
    health_report: Vec<DescribedObject>,
}

impl HealthReportOfJob {
    pub fn display_health_report(self) -> Result<(), failure::Error> {
        println!("-----------  Health Check  ------------");
        println!("\nJob Name: {}", self.name);
        for report in self.health_report {
            let description = report.description.unwrap_or("None".to_string());
            println!("\n{}", description);
        }
        Ok(())
    }

    pub fn get_default_tree() -> TreeBuilder {
        TreeBuilder::new()
            .with_field("name")
            .with_field(TreeBuilder::object("healthReport").with_subfield("description"))
    }
}
