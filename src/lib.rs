extern crate env_logger;

use std::sync::{Once, ONCE_INIT};

static LOGGER_INIT: Once = ONCE_INIT;

#[macro_export]
macro_rules! test {
    (should_panic, $name:ident, $test:block) => {
        test!(#[should_panic], $name, $test);
    };
    ($(#[$attr:meta])*, $name:ident, $test:block) => {
        #[test]
        $( #[$attr] )*
        fn $name() {
            test_logger::ensure_env_logger_initialized();
            $test
        }
    };
    ($name:ident, $test:block) => {
        test!(, $name, $test);
    };
}

pub fn ensure_env_logger_initialized() {
    LOGGER_INIT.call_once(|| env_logger::init());
}



