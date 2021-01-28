#[allow(unused_macros)]
macro_rules! useit {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}

#[cfg(feature = "wagner_20")]
useit!(wagner_20_11);
#[cfg(feature = "wagner_22")]
useit!(wagner_22_13);
#[cfg(feature = "wagner_23")]
useit!(wagner_23_14);
#[cfg(feature = "wagner_25")]
useit!(wagner_25_15);
#[cfg(feature = "wagner_26")]
useit!(wagner_26_16);
#[cfg(feature = "wagner_27")]
useit!(wagner_27_17);
#[cfg(feature = "wagner_28")]
useit!(wagner_28_18);
#[cfg(feature = "wagner_29")]
useit!(wagner_29_19);
#[cfg(feature = "wagner_30")]
useit!(wagner_30_20);
#[cfg(feature = "wagner_32")]
useit!(wagner_32_21);
