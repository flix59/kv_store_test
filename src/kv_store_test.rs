use scrypto::prelude::*;


#[blueprint]
mod kv_store_test {
    struct KVStoreTest {
        store: KeyValueStore<i32, Decimal>,
    }

    impl KVStoreTest {
        pub fn instantiate() -> Global<KVStoreTest> {
            let store: KeyValueStore<i32, Decimal> = KeyValueStore::new();
            let component = (Self { store })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            component
        }

        pub fn insert_up_to(&mut self, max_key: i32, value: Decimal) {
            debug!("insert: {}", max_key);
            for i in 0..max_key {
                self.store.insert(i, value);
            }
        }
        pub fn reset(&mut self, max_key: i32) {
            for i in 0..max_key {
                self.store.remove(&i);
            }
        }
    }
}
