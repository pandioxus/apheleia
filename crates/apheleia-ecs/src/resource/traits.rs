use std::{any::Any, fmt::Debug};

pub trait Resource: Any + Debug + 'static {}
