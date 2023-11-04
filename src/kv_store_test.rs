use scrypto::prelude::*;


#[blueprint]
mod kv_store_test {
    struct KVStoreTest {
        store: KeyValueStore<Decimal, Node>,
    }

    impl KVStoreTest {
        pub fn instantiate() -> Global<KVStoreTest> {
            let store: KeyValueStore<Decimal, Node> = KeyValueStore::new();
            let component = (Self { store })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            component
        }

        pub fn insert(&mut self, key: Decimal, value: PreciseDecimal) {
            let tick = Tick {
                delta_liquidity: value.clone(),
                total_liquidity: value.clone(),
                price_sqrt: value.clone(),
                x_fee_outside: value.clone(),
                y_fee_outside: value.clone(),
            };
            let node = Node {
                key,
                value: tick,
                left_child: Some(key.clone()),
                right_child: Some(key.clone()),
                parent: Some(key.clone()),
                next: Some(key.clone()),
                prev: Some(key.clone()),
                balance_factor: 0,
            };
            self.store.insert(key, node);
        }
    }
}
#[derive(ScryptoSbor, Clone)]
pub struct Tick {
    pub delta_liquidity: PreciseDecimal,
    pub total_liquidity: PreciseDecimal,
    pub price_sqrt: PreciseDecimal,
    pub x_fee_outside: PreciseDecimal,
    pub y_fee_outside: PreciseDecimal,
}
#[derive(ScryptoSbor, Clone)]
pub(crate) struct Node {
    /// Unique key for this node
    pub(crate) key: Decimal,
    pub(crate) value: Tick,
    /// The left and right children of this node in the tree
    pub(crate) left_child: Option<Decimal>,
    pub(crate) right_child: Option<Decimal>,
    /// The parent of this node in the tree
    pub(crate) parent: Option<Decimal>,
    /// The next and previous nodes in double linked list. The double linked list is ordered by the keys.
    /// So to get a sorted list of all keys, we can iterate over these pointers.
    pub(crate) next: Option<Decimal>,
    pub(crate) prev: Option<Decimal>,
    /// Balance factor: height of right subtree - height of left subtree.
    /// The heights are never calculated, but the balance factor is updated
    /// based on the operations (insert, delete, balance) in the tree.
    pub(crate) balance_factor: i32,
}
