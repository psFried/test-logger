#[macro_use]
extern crate test_logger;

#[macro_use]
extern crate log;

fn get_some_value() -> u32 {
    debug!("This out put will only be printed if the test fails AND the log level is set to debug");
    error!("This output will be be printed by default if the test fails");
    99
}

test!(failing_test_has_log_output_printed_if_log_level_is_high_enough, {
    assert_eq!(100, get_some_value());
});
