mod helper_kv_store;

#[cfg(test)]
mod kv_store_test {
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_kv_store::*;

    #[test]
    fn test_() {
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        for i in 1..i32::MAX {
            let key = Decimal::from(i);
            helper.insert(key.clone(), key.clone());
            helper.execute_expect_success(true);
            println!("inserts: {}", i);
        }
    }
}

// TODO how many read write in one operation
 // max size of KV store