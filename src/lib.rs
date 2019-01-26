use std::collections::HashMap;

pub struct RateTable {
    base: HashMap<String, f32>,
    total: f32,
}

impl RateTable {
    pub fn new() -> RateTable {
        RateTable {
            base: HashMap::new(),
            total: 0f32,
        }
    }

    pub fn from_vec(in_table: HashMap<String, f32>) -> RateTable {
        let mut total = 0f32;
        for entry in &in_table {
            total += entry.1
        }

        RateTable {
            base: in_table,
            total: total,
        }
    }

    pub fn push(mut self, label: String, weight: f32) -> RateTable {
        self.base.insert(label, weight);
        self.total += weight;
        self
    }

    pub fn count(self) -> usize {
        self.base.len()
    }
}
