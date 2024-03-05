use std::collections::HashSet;
use crate::Listener;
use crate::platform::hi;

pub fn hi_cross() -> Result<HashSet<Listener>, String> {
    hi()
}
