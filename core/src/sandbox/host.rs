use crate::actor::Actor;
use std::rc::Rc;

use crate::sandbox::execution_context::ExecutionContext;

pub struct Host {
    global_context: ExecutionContext,
    actors: Vec<Box<dyn Actor>>,
}
