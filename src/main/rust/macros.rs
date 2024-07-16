

#[macro_export]
macro_rules! heap {
    ($name:ident, $type:ty) => {
        #[allow(dead_code)]
        pub mod $name {
            use std::collections::HashMap;

            static mut HEAP: Option<HashMap<i64, $type>> = None;

            pub fn get() -> &'static mut HashMap<i64, $type> {
                unsafe {
                    if HEAP.is_none() {
                        HEAP = Some(HashMap::new());
                    }
                    HEAP.as_mut().unwrap()
                }
            }

            pub fn new_address() -> i64 {
                let heap = self::get();
                let mut id = 0;
                while heap.contains_key(&id) {
                    id += 1;
                }
                id
            }

            pub fn access(id: i64) -> Option<&'static $type> {
                let heap = self::get();
                heap.get(&id)
            }

            pub fn access_mut(id: i64) -> Option<&'static mut $type> {
                let heap = self::get();
                heap.get_mut(&id)
            }

            pub fn manual_allocation(id: i64, light: $type) {
                let heap = self::get();
                heap.insert(id, light);
            }

            pub fn allocate(light: $type) -> i64 {
                let id = new_address();
                manual_allocation(id, light);
                id
            }

            pub fn free(id: i64) {
                let heap = self::get();
                heap.remove(&id);
            }
        }
    };
}
