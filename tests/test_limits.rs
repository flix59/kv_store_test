mod helper_kv_store;

#[cfg(test)]
mod kv_store_test {
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_kv_store::*;
    //
    // fn get_left_right_child(value: Decimal) -> (Decimal, Decimal) {
    //     let half = value * Decimal::from(3 / 4);
    //     let double = value / Decimal::from(4);
    //     (half, double)
    // }
    //
    // fn insert_in_tree_rec(value: Decimal, helper: &mut TestHelper, inserts: &mut Decimal, depth: u32) {
    //     if depth > 20 {
    //         return;
    //     }
    //     let (left, right) = get_left_right_child(value);
    //     *inserts = *inserts + Decimal::ONE;
    //     helper.insert(left, inserts.clone());
    //     *inserts = *inserts + Decimal::ONE;
    //     helper.insert(right, inserts.clone());
    //     println!("depth: {}", inserts);
    //     helper.execute_expect_success(false);
    //     insert_in_tree_rec(left, helper, inserts, depth + 1);
    //     insert_in_tree_rec(right, helper, inserts, depth + 1);
    // }
    //

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
        panic!()
    }
}

// TODO how many read write in one operation
 // max size of KV store