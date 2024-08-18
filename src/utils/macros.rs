

macro_rules! array_key {
    (
        $vis: vis
        enum
        $name: ident
        {
            $($variant: ident),*
            $(,)?
        }
    ) => {
        #[derive(Copy, Clone, Debug)]
        $vis enum $name {
            $($variant),*
        }
        impl $name {
            #[allow(dead_code, path_statements)]
            $vis const COUNT: usize = $({Self::$variant; 1} + )* 0;
            #[allow(dead_code)]
            $vis const ARRAY: [Self; Self::COUNT] = [$(Self::$variant),*];

            #[allow(dead_code)]
            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => stringify!($variant)
                    ),*
                }
            }
        }
    };
}
pub(crate) use array_key;
