
use serde::{Serialize, Deserialize};
use serde_json;

mod defaults {
    pub(crate) fn default_bool<const Val: bool>() -> bool {
        Val
    }
    pub(crate) fn default_i64<const Val: i64>() -> i64 {
        Val
    }
    pub(crate) fn default_u64<const Val: u64>() -> u64 {
        Val
    }
}

include!("../gen/api.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
