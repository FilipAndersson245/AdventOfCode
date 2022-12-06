pub mod prelude {
    //! Helper prelude with useful imports.
    pub use crate::{bench, input, run};
    pub use anyhow::{anyhow, bail, Context, Result};
    pub use indexmap::IndexMap;
    pub use itertools;
    pub use itertools::Itertools;
}
pub use rayon;

/// Prepare an input processor.
#[macro_export]
macro_rules! input {
    ($path:literal) => {{
        let path = concat!("inputs/", $path);
        let read_path = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", $path);
        std::fs::read_to_string(read_path)
    }};
}

#[macro_export]
macro_rules! run {
    ($name:literal, $func:expr, $expected:literal) => {
        let res = { $func };
        assert_eq!(res, $expected);
        println!("{}: \x1b[93m{}\x1b[0m", $name, res);
        bench!($func);
        println!();
    };
    ($name:literal, $func:expr) => {
        let res = { $func };
        println!("{}: {}", $name, res);
        bench!($func);
        println!();
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! bench {
    ($func:expr) => {
        #[allow(unused_must_use)]
        {
            let iterations = 500;
            let mut measurments: Vec<std::time::Duration> = Vec::with_capacity(iterations);
            for _ in 0..iterations / 10 {
                std::hint::black_box({ $func });
            }

            let start = std::time::Instant::now();
            for _ in 0..iterations {
                #[allow(unused_must_use)]
                std::hint::black_box({ $func });
            }
            let d = std::time::Instant::now().duration_since(start);
            let d = d / iterations as u32;

            println!(" took {d:?}");
        }
    };

    ($name:literal, $func:expr) => {
        println!("{}:", $name);
        bench!($func);
    };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! bench {
    ($func:expr) => {};
    ($name:literal, $func:expr) => {};
}
