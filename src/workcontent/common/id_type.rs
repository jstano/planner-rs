use uuid::Uuid;

#[macro_export]
macro_rules! id_type {
    ($name:ident, uuid_v4) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            pub fn from(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }

            pub fn as_uuid(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
    ($name:ident, uuid_v7) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(uuid::Uuid::now_v7())
            }

            pub fn from(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }

            pub fn as_uuid(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use uuid::Uuid;
    use crate::id_type;

    // Define temporary id types for testing via the exported macro
    id_type!(TestIdV4, uuid_v4);
    id_type!(TestIdV7, uuid_v7);

    #[test]
    fn v4_new_produces_unique_ids_and_is_copy_eq_hash() {
        let mut set: HashSet<TestIdV4> = HashSet::new();
        for _ in 0..32 {
            let id = TestIdV4::new();
            assert!(set.insert(id), "duplicate UUID v4 generated unexpectedly");
        }

        let a = TestIdV4::new();
        let b = a; // copy
        assert_eq!(a, b);

        assert!(set.insert(a));
    }

    #[test]
    fn v4_from_and_as_uuid_and_display_round_trip() {
        let raw = Uuid::new_v4();
        let id = TestIdV4::from(raw);
        assert_eq!(id.as_uuid(), &raw);
        assert_eq!(id.to_string(), raw.to_string());
    }

    #[test]
    fn v7_new_produces_orderable_ids() {
        let mut ids: Vec<TestIdV7> = (0..32).map(|_| TestIdV7::new()).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        ids.sort();
        assert_eq!(sorted, ids);
    }

    #[test]
    fn v7_from_and_as_uuid_and_display_round_trip() {
        let raw = Uuid::now_v7();
        let id = TestIdV7::from(raw);
        assert_eq!(id.as_uuid(), &raw);
        assert_eq!(id.to_string(), raw.to_string());
    }
}
