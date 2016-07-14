Test Logger
===========

Simple helper for Rust to initialize env_logger before unit and integration tests.

Running `cargo test` does not initialize a logging backend, so those of us who use the `log` crate do not get to see log output when a unit or integration test fails. This crate just provides a simple macro that provides a way to initialize an `env_logger` during unit and integration tests while still making tests somewhat ergonomic to write.

## `test!`

Many applications use logging to aid in debugging a running application, but unfortunately there's no simple way to initialize the logger prior to running unit and integration tests. The `test!` macro is just a simple wrapper to take care of initializing an `env_logger` prior to running tests. This ensures that log statements are printed to stdout during the tests. The macro ensures that the logger is initialized exactly once.

use the `test!` macro:

```rust
test!(logs_are_printed_to_stdout_when_this_fails, {
    assert!(subject.do_stuff_that_fails());
});
// The above macro expands to:
#[test]
fn logs_are_printed_to_stdout_when_this_fails() {
    test_logger::ensure_env_logger_initialized();
    assert!(subject.do_stuff_that_fails());
}

test!(should_panic, simple_way_to_annotate_a_test_that_is_supposed_to_panic, {
    panic!("I'm supposed to panic.");
});
//expands to:
#[test]
#[should_panic]
fn simple_way_to_annotate_a_test_that_is_supposed_to_panic() {
    test_logger::ensure_env_logger_initialized();
    assert!(subject.do_stuff_that_fails());
}

// alternate format to support any arbitrary attributes on the test function
test!(#[allow(dead_code)], this_test_function_will_have_the_dead_code_lint_disabled, {
    ... 
    
```

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
