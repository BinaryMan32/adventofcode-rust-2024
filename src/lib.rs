use std::{collections::HashSet, str::Lines};

pub struct Named<T> {
    pub wrapped: T,
    pub name: &'static str,
}

impl<T> Named<T> {
    pub fn create(t: T, name: &'static str) -> Self {
        Self { wrapped: t, name }
    }
}

#[macro_export]
macro_rules! named {
    ($x:ident) => {
        &Named::create($x, std::stringify!($x))
    };
}

pub struct Runner {
    module_name: &'static str,
    operations: HashSet<String>,
}

type Operation = fn(Lines) -> String;

impl Runner {
    pub fn create(module_name: &'static str) -> Self {
        Self {
            module_name,
            operations: std::env::args().skip(1).collect(),
        }
    }

    pub fn run(&self, op: &Named<Operation>, input: &str) {
        let enabled: bool = self.operations.is_empty() || self.operations.contains(op.name);
        let result: String = if enabled {
            (op.wrapped)(input.lines())
        } else {
            String::from("(DISABLED)")
        };
        println!("{} {}:\n{}", self.module_name, op.name, result);
    }
}

#[macro_export]
macro_rules! create_runner {
    () => {
        &Runner::create(module_path!())
    };
}

#[macro_export]
macro_rules! verify {
    ( $op:ident, $input:ident, $expected:expr ) => {{
        let result = $op($input.lines());
        assert_eq!(
            result,
            $expected,
            "{} {}",
            module_path!(),
            std::stringify!($op)
        );
    }};
}
