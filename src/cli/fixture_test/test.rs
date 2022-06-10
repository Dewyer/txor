use super::super::process_reader;

use crate::cli::fixture_test::test_case::read_test_case_from_file;
use crate::cli::setup_logs::setup_logs;
use rstest::rstest;
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
#[case("5_no_tx")]
#[case("6_precision")]
#[case("7_dis_no_access")]
#[case("8_dis_started")]
#[case("10_res_no_double")]
#[case("12_chb_simple")]
#[case("13_chb_no_double")]
#[case("14_chb_locked_acc")]
#[case("15_dis_withdrawn")]
#[case("16_chb_withdrawn")]
#[case("17_dep_overflow_check")]
#[tokio::test]
async fn fixture_test(#[case] file_name: &str) {
    initialize();

    let test_case = read_test_case_from_file(file_name).await;
    let input_bytes = Box::leak(Box::new(test_case.input_csv.clone())).as_bytes();
    let mut output = process_reader(input_bytes).await;

    output
        .clients
        .sort_by(|aa, bb| aa.get_id().cmp(&bb.get_id()));
    output.transactions_in_dispute.sort();

    assert_eq!(
        output, test_case.output,
        "test case: {}, expected to match output",
        test_case.name
    );
}
