use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
  pub id: String,
  pub name: String,
  pub run: String,
  pub depends_on: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline {
  pub name: String,
  pub stages: Vec<Stage>,
}

impl Pipeline {
  pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Pipeline, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let p = serde_yaml::from_reader(reader)?;
    Ok(p)
  }
}
