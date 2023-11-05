mod helper_kv_store_node;

#[cfg(test)]
mod kv_store_test {
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_kv_store_node::*;

    #[test]
    fn test_() {
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        let mut key = 420;
        let mut offset = 1;
        for i in 0..1000 {
            let value = PreciseDecimal::from(key);
            helper.insert(key, value);
            helper.execute_expect_success(true);
            println!("inserts: {}", key);
            helper.reset(key);
            offset = offset* 2;
            key = key + offset;
        }
        panic!("")
    }
}

// TODO how many read write in one operation
 // max size of KV store