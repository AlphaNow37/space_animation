macro_rules! imports {
    (
        $($name: ident),* $(,)?
    ) => {
        $(

            pub mod $name;
            #[allow(unused_imports)]
            pub use $name::*;
        )*
    };
}

imports!(axis,);
