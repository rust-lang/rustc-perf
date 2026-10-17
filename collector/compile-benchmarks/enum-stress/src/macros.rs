macro_rules! ast_struct {
    (
        $(#[$attr:meta])*
        pub struct $name:ident $($rest:tt)*
    ) => {
        $(#[$attr])*
        #[derive(PartialEq)]
        pub struct $name $($rest)*
    };
}

macro_rules! ast_enum {
    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident { $($variants:tt)* }
    ) => (
        $(#[$enum_attr])*
        #[derive(PartialEq)]
        pub enum $name {
            $($variants)*
        }
    )
}

macro_rules! ast_enum_of_structs {
    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_attr:meta])*
                pub $variant:ident($member:ident $($rest:tt)*),
            )*
        }
    ) => (
        ast_enum! {
            $(#[$enum_attr])*
            pub enum $name {
                $(
                    $(#[$variant_attr])*
                    $variant($member),
                )*
            }
        }

        $(
            maybe_ast_struct! {
                $(#[$variant_attr])*
                pub struct $member $($rest)*
            }
        )*
    )
}

macro_rules! maybe_ast_struct {
    (
        $(#[$attr:meta])*
        pub struct $name:ident
    ) => ();

    ($($rest:tt)*) => (ast_struct! { $($rest)* });
}
