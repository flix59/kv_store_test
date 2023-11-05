use scrypto::prelude::*;


#[blueprint]
mod kv_store_test_u16 {
    struct KVStoreTestU16 {
        store: KeyValueStore<u16, ()>,
    }

    impl KVStoreTestU16 {
        pub fn instantiate() -> Global<KVStoreTestU16> {
            let store: KeyValueStore<u16, ()> = KeyValueStore::new();
            let component = (Self { store })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            component
        }

        pub fn insert(&mut self, key: u16) {
            self.store.insert(key, ());
        }
    }
}
