pub mod renderable;

use std::any::Any;
use std::fmt;

pub trait Component: fmt::Debug + Any {
    fn name(&self) -> &'static str;
}
