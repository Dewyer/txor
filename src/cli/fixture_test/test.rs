use tokio::fs::File;
use tokio::io::AsyncReadExt;
use super::super::{process_reader, write_processing_output};

use rstest::{rstest};
use crate::cli::fixture_test::test_case::read_test_case_from_file;
use crate::cli::setup_logs::setup_logs;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        setup_logs(true).unwrap();
    });
}

#[rstest]
#[case("0_deposit")]
#[case("1_withdrawal")]
#[case("2_duplicate_txid")]
#[case("3_client_persistance")]
#[case("4_withdrawal_insufficient")]
#[tokio::test]
async fn fixture_test(#[case] file_name: &str) {
    initialize();

    let test_case = read_test_case_from_file(file_name).await;
    // log::info!("Running {} - test", test_case.name);
    let input_bytes = Box::leak(Box::new(test_case.input_csv.clone())).as_bytes();
    let output = process_reader(input_bytes)
        .await;

    assert_eq!(output, test_case.output, "test case: {}, expected to match output", test_case.name);
}