use casper_execution_engine::core::{engine_state::Error as EngineStateError, execution};
use casper_types::ApiError;

pub(crate) fn assert_expected_error(actual_error: EngineStateError, error_code: u16, reason: &str) {
    let actual = format!("{actual_error:?}");
    let expected = format!(
        "{:?}",
        EngineStateError::Exec(execution::Error::Revert(ApiError::User(error_code)))
    );

    assert_eq!(
        actual, expected,
        "Error should match {error_code} with reason: {reason}"
    )
}
