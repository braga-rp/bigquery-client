use serde::{Serialize};
use super::CrudResult;
use serde_json;
use crate::gcloud::{GCloud, GCloudFactory, client::Endpoint};

pub struct Table {
    gcloud_client: GCloud,
    project_id: String,
    dataset_id: String,
    name: String,
}


impl Endpoint for Table {}

impl<'a> Table {

    pub fn insert_many(&self, entities: &Vec<&impl Serialize>) -> CrudResult<()> {
        let header = self.gcloud_client.header_value();
        let resource = format!("bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}/insertAll",
                                     project_id=self.project_id, dataset_id=self.dataset_id, table_id=self.name);
        let endpoint = self.endpoint(resource.as_str());

        println!("{}", header);
        println!("{}", endpoint);
        println!("{:?}", serde_json::to_string(entities));

        Ok(())
    }

    pub fn insert(&self, entity: &impl Serialize) -> CrudResult<()> {
        let entities = &vec![entity];
        self.insert_many(entities)
    }

    pub fn new(gcloud_factory: &'a GCloudFactory, project_id: &str, dataset_id: &str, name: &str) -> Table {
        let gcloud_client = gcloud_factory();
        Table {
            project_id: project_id.to_owned(),
            dataset_id: dataset_id.to_owned(),
            name: name.to_owned(),
            gcloud_client
        }
    }
}