
mod defaults {
    pub(crate) fn default_bool<const VAL: bool>() -> bool {
        VAL
    }
    pub(crate) fn default_i64<const VAL: i64>() -> i64 {
        VAL
    }
    pub(crate) fn default_u64<const VAL: u64>() -> u64 {
        VAL
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
