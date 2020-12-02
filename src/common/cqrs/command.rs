use std::fmt;

use crate::common::cqrs;

pub trait Command<A>: fmt::Debug
where
    A: cqrs::Aggregate,
{
    fn apply(&self, aggregate: A);
}

pub trait CommandHandler<A, C>
where
    A: cqrs::Aggregate,
    C: Command<A>,
{
    fn apply(&mut self, command: C);
}
