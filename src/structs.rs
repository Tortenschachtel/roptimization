use serde::Deserialize;

/* An InputFile has a name and a size */
#[derive(Deserialize, Eq, PartialEq, Clone)]
pub struct InputFile {
  pub name: String,
  pub size: u64,
}

/* Data is a list of input files plus the total disk size */
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
  pub files: Vec<InputFile>,
  pub  disk_size: u64,
}

/* A task contains the names of the two files it needs to run */
#[derive(Clone, PartialEq, Eq)]
pub struct Task {
  pub first: String,
  pub second: String,
}

/* The working data contains the original data, a list of tasks that still need to run, how much data has been loaded thus far */
#[derive(Clone)]
pub struct WorkResult {
  pub operations: Vec<String>,
  pub download_size: u64,
}
