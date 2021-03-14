use crate::leven;
use crate::individual::{Individual};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BananaIndivid {
    value: String,
}

impl Individual for BananaIndivid {
    type Storage = Option<u8>;

    fn fitness(&self) -> f64 {
	let p = leven::levenshtein(self.value.as_str(), "banana");
	if p == 0 {
	    10000.0
	} else {
	    1.0 / p as f64
	}
    }

    fn ea_name(&self) -> String {
	String::from("BANANAAS")
    }

    fn mutate(&self, _t: &mut Self::Storage) -> Self {
	Self::default()
    }

    fn crossover(&self, _other: &Self, _t: &mut Self::Storage) -> Self {
	Self::default()
    }
}

impl Default for BananaIndivid {
    fn default() -> Self {
	Self {
	    value: String::from(""),
	}
    }
}
