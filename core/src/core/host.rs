use crate::actor::Actor;

use crate::core::execution_context::ExecutionContext;

pub struct Host {
    global_context: ExecutionContext,
    actors: Vec<Box<dyn Actor>>,
}
