
macro_rules! useit {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}

useit!(mds_3_2);
useit!(mds_4_3);
useit!(mds_5_4);
