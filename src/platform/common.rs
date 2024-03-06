use std::collections::HashSet;

use crate::platform::get_all_listeners;
use crate::Listener;

pub fn get_all() -> Result<HashSet<Listener>, String> {
    get_all_listeners()
}
