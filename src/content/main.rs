macro_rules! import {
    (
        $name: ident
    ) => {
        mod $name;
        pub use $name::build;
    };
}

// import!(tests);

import!(world_runner);


fn main() {
    lib_space_animation::run(build)
}
