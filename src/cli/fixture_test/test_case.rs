use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::processor::ProcessingOutput;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCase {
    pub name: String,
    pub description: Option<String>,

    pub input_csv: String,

    pub output: ProcessingOutput,
}

pub async fn read_test_case_from_file(file_name: &str) -> TestCase {
    let mut test_case_file = File::open(format!("test_data/{}.toml", file_name))
        .await.unwrap();
    let mut test_case_str = String::new();
    test_case_file.read_to_string(&mut test_case_str).await.unwrap();

    toml::from_str::<TestCase>(&test_case_str)
        .unwrap()
}