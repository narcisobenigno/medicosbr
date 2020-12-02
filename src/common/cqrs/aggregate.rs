use std::fmt;

pub trait Aggregate: fmt::Debug {
    fn name() -> String;
}
