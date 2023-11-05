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
        for i in 0..i32::MAX {
            for j in 0..i {
                let key = Decimal::from(j);
                helper.insert(key.clone(), key.clone());
                println!("inserts: {}",j);
            }
            helper.execute_expect_success(true);
            helper.reset(i);
        }
    }
}
