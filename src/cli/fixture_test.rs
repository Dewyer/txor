use tokio::fs::File;
use tokio::io::AsyncReadExt;
use super::{process_file, write_processing_output};

use rstest::{rstest};
use crate::cli::setup_logs::setup_logs;

#[rstest]
#[case("simple deposit", "0_deposit")]
#[tokio::test]
async fn fixture_test(#[case] _name: &str, #[case] file_name: &str) {
    setup_logs(true).unwrap();

    let input_file = File::open(format!("test_data/{}_input.csv", file_name))
        .await.unwrap();

    let output = process_file(
        input_file
    ).await;

    let mut buff_wr = Vec::new();
    write_processing_output::write_processing_output(output, &mut buff_wr).await.unwrap();

    let output_csv = std::str::from_utf8(&buff_wr).unwrap();

    let mut correct_output_file = File::open(format!("test_data/{}_output.csv", file_name))
        .await.unwrap();
    let mut correct_output_csv = String::new();
    correct_output_file.read_to_string(&mut correct_output_csv).await.unwrap();

    assert_eq!(output_csv.trim(), correct_output_csv.trim());
}