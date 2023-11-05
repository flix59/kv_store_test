mod helper_kv_store_u16;

#[cfg(test)]
mod kv_store_test {
    use std::u16;
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_kv_store_u16::*;

    #[test]
    fn test_() {
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        for i in 1..u16::MAX {
            helper.insert(i);
            helper.execute_expect_success(true);
            println!("inserts: {}", i);
        }
    }
}

// TODO how many read write in one operation
 // max size of KV store