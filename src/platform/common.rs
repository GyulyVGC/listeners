use crate::platform::get_all_listeners;
use crate::Listener;
use std::collections::HashSet;

pub fn get_all() -> Result<HashSet<Listener>, String> {
    get_all_listeners()
}
