macro_rules! useit {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}

useit!(hamming_3_1);
useit!(hamming_7_4);
useit!(hamming_15_11);
useit!(hamming_31_26);
useit!(hamming_63_57);
useit!(hamming_127_120);
