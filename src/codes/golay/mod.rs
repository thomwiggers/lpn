
macro_rules! useit {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}

useit!(golay_23_12);
useit!(golay_24_12);
