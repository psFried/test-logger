#[macro_use]
extern crate test_logger;

#[macro_use]
extern crate log;

fn get_some_value() -> u32 {
    debug!("This out put will only be printed if the test fails AND the log level is set to debug");

    error!("This output will be be printed by default if the test fails");
    99
}

test!(passing_test_should_never_have_its_output_printed, {
    assert_eq!(99, get_some_value());
});

test!(should_panic, test_may_specify_that_it_should_panic, {
    panic!("This test is supposed to fail")
});

test!(#[should_panic], test_may_specify_that_it_should_panic_by_using_the_full_attribute, {
    panic!("This test also is supposed to fail")
});



