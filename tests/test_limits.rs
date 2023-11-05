mod helper_kv_store;

#[cfg(test)]
mod kv_store_test {
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_kv_store::*;

    #[test]
    fn test_insert_and_update() {
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        let mut key = 430;
        let mut offset = 1;
        for i in 1..i32::MAX {
            helper.insert(key, Decimal::from(key));
            helper.execute_expect_success(true);
            helper.insert(key, Decimal::from(0));
            helper.execute_expect_success(true);
            helper.reset(key);
            offset = offset* 2;
            key = key + offset;
            println!("inserts: {}", key);
        }
    }
    // #[test]
    fn test_insert() {
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        let mut key = 430;
        let mut offset = 1;
        for i in 1..i32::MAX {
            helper.insert(key, Decimal::from(key));
            helper.execute_expect_success(true);
            helper.reset(key);
            offset = offset* 2;
            key = key + offset;
            println!("inserts: {}", key);
        }
    }
}

// TODO how many read write in one operation
 // max size of KV store