use super::GCloudFactory;

pub mod table;

pub use table::Table;

pub type CrudResult<T> = Result<T, Box<dyn std::error::Error>>;


pub struct BigQueryClient<'a> {
    gcloud_factory: &'a GCloudFactory,
    project_id: String,
}

impl<'a> BigQueryClient<'a> {

    pub fn new(gcloud_factory: &'a GCloudFactory, project_id: &str) -> BigQueryClient<'a> {
       
        BigQueryClient{
            gcloud_factory,
            project_id: project_id.to_owned(),
        }
    }

    pub fn table(&self, dataset_id: &str, name: &str) -> Table {
        Table::new(self.gcloud_factory, self.project_id.as_str(), dataset_id, name)
    }

}