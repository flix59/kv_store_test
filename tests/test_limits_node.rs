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
        for i in 1..i32::MAX {
            let key = Decimal::from(i);
            let value = PreciseDecimal::from(i);
            helper.insert(key, value);
            helper.execute_expect_success(true);
            println!("inserts: {}", i);
        }
    }
}

// TODO how many read write in one operation
 // max size of KV store