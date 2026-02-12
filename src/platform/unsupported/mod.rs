use crate::{Listener, Process, Protocol};
use std::collections::HashSet;

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    Err("This OS isn't supported yet".into())
}

pub(crate) fn get_process_by_port(_port: u16, _protocol: Protocol) -> crate::Result<Process> {
    Err("This OS isn't supported yet".into())
}
