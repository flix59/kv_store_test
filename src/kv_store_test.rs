use scrypto::prelude::*;


#[blueprint]
mod kv_store_test {
    struct KVStoreTest {
        store: KeyValueStore<Decimal, Decimal>,
    }

    impl KVStoreTest {
        pub fn instantiate() -> Global<KVStoreTest> {
            let store: KeyValueStore<Decimal, Decimal> = KeyValueStore::new();
            let component = (Self { store })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            component
        }

        pub fn insert(&mut self, key: Decimal, value: Decimal) {
            self.store.insert(key, value);
        }
        pub fn reset(&mut self, last_key: i32) {
            for i in 0..last_key {
                let key = Decimal::from(i);
                self.store.remove(&key);
            }
        }
    }
}
