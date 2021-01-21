
macro_rules! useit {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}

useit!(wagner_20_11);
useit!(wagner_22_13);
useit!(wagner_23_14);
useit!(wagner_25_15);
useit!(wagner_26_16);
useit!(wagner_27_17);
useit!(wagner_28_18);
useit!(wagner_29_19);
useit!(wagner_30_20);
useit!(wagner_32_21);
