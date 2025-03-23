#[macro_export]
macro_rules! serde_struct {
    (
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident : $type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
        pub struct $name {
            $(
                $(#[$field_meta])*
                pub $field: $type
            ),*
        }
    };
}

#[macro_export]
macro_rules! serde_enum {
    (
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant
            ),*
        }
    };
}
