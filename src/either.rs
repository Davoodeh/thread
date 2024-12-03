//! Budget either crate.

#![allow(unused_imports, dead_code)]

pub(crate) mod prelude {
    pub(crate) use super::Either::{self, Left, Right};
}

#[derive(Debug, Clone)]
pub(crate) enum Either<Left, Right> {
    Left(Left),
    Right(Right),
}

impl<Left, Right> Either<Left, Right> {
    pub fn left(&self) -> Option<&Left> {
        match self {
            Either::Left(v) => Some(v),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<&Right> {
        match self {
            Either::Right(v) => Some(v),
            _ => None,
        }
    }
}
