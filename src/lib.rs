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
            test_logger::init_env_logger();
            $test
        }
    };
    ($name:ident, $test:block) => {
        test!(, $name, $test);
    };
}

pub fn init_env_logger() {
    LOGGER_INIT.call_once(|| env_logger::init().unwrap());
}



