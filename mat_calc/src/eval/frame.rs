use super::object::*;
use std::collections::HashMap;

pub struct Frame {
    objects: HashMap<String, ObjectPairItem>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            objects: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, object: ObjectPairItem) {
        self.objects.insert(name, object);
    }

    pub fn get(&self, name: &str) -> Option<ObjectPairItem> {
        self.objects.get(name).map(|obj| obj.clone())
    }

    pub fn format_func_args(&self) -> String {
        let mut ret = String::new();
        for (key, val) in self.objects.iter() {
            ret.push_str(&format!("{}={}, ", key, val));
        }
        return ret;
    }
}
