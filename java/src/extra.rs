use std::collections::HashMap;
use super::*;

pub struct Extra {
    /// local variable that are occupied.
    locals: HashMap<String, ClassType>,
}

impl Extra {
    pub fn new() -> Extra {
        Extra { locals: HashMap::new() }
    }

    pub fn with_locals(locals: HashMap<String, ClassType>) -> Extra {
        Extra { locals: locals }
    }

    // check if the given local is registered
    pub fn absolute_import(&self, name: &str, class_type: &ClassType) -> bool {
        if let Some(value) = self.locals.get(name) {
            class_type.to_raw() != *value
        } else {
            false
        }
    }
}

impl Default for Extra {
    fn default() -> Extra {
        Extra::new()
    }
}
