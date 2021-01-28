#[allow(unused_macros)]
macro_rules! useit {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}

#[cfg(feature = "guava_11")]
useit!(guava_11_10);
#[cfg(feature = "guava_12")]
useit!(guava_12_10);
#[cfg(feature = "guava_12")]
useit!(guava_12_11);
#[cfg(feature = "guava_13")]
useit!(guava_13_10);
#[cfg(feature = "guava_13")]
useit!(guava_13_11);
#[cfg(feature = "guava_13")]
useit!(guava_13_12);
#[cfg(feature = "guava_14")]
useit!(guava_14_10);
#[cfg(feature = "guava_14")]
useit!(guava_14_11);
#[cfg(feature = "guava_14")]
useit!(guava_14_12);
#[cfg(feature = "guava_14")]
useit!(guava_14_13);
#[cfg(feature = "guava_15")]
useit!(guava_15_10);
#[cfg(feature = "guava_15")]
useit!(guava_15_11);
#[cfg(feature = "guava_15")]
useit!(guava_15_12);
#[cfg(feature = "guava_15")]
useit!(guava_15_13);
#[cfg(feature = "guava_15")]
useit!(guava_15_14);
#[cfg(feature = "guava_16")]
useit!(guava_16_10);
#[cfg(feature = "guava_16")]
useit!(guava_16_11);
#[cfg(feature = "guava_16")]
useit!(guava_16_12);
#[cfg(feature = "guava_16")]
useit!(guava_16_13);
#[cfg(feature = "guava_16")]
useit!(guava_16_14);
#[cfg(feature = "guava_16")]
useit!(guava_16_15);
#[cfg(feature = "guava_17")]
useit!(guava_17_10);
#[cfg(feature = "guava_17")]
useit!(guava_17_11);
#[cfg(feature = "guava_17")]
useit!(guava_17_12);
#[cfg(feature = "guava_17")]
useit!(guava_17_13);
#[cfg(feature = "guava_17")]
useit!(guava_17_14);
#[cfg(feature = "guava_17")]
useit!(guava_17_15);
#[cfg(feature = "guava_17")]
useit!(guava_17_16);
#[cfg(feature = "guava_18")]
useit!(guava_18_10);
#[cfg(feature = "guava_18")]
useit!(guava_18_11);
#[cfg(feature = "guava_18")]
useit!(guava_18_12);
#[cfg(feature = "guava_18")]
useit!(guava_18_13);
#[cfg(feature = "guava_18")]
useit!(guava_18_14);
#[cfg(feature = "guava_18")]
useit!(guava_18_15);
#[cfg(feature = "guava_18")]
useit!(guava_18_16);
#[cfg(feature = "guava_18")]
useit!(guava_18_17);
#[cfg(feature = "guava_19")]
useit!(guava_19_4);
#[cfg(feature = "guava_19")]
useit!(guava_19_10);
#[cfg(feature = "guava_19")]
useit!(guava_19_11);
#[cfg(feature = "guava_19")]
useit!(guava_19_12);
#[cfg(feature = "guava_19")]
useit!(guava_19_13);
#[cfg(feature = "guava_19")]
useit!(guava_19_14);
#[cfg(feature = "guava_19")]
useit!(guava_19_15);
#[cfg(feature = "guava_19")]
useit!(guava_19_16);
#[cfg(feature = "guava_19")]
useit!(guava_19_17);
#[cfg(feature = "guava_19")]
useit!(guava_19_18);
#[cfg(feature = "guava_20")]
useit!(guava_20_10);
#[cfg(feature = "guava_20")]
useit!(guava_20_11);
#[cfg(feature = "guava_20")]
useit!(guava_20_12);
#[cfg(feature = "guava_20")]
useit!(guava_20_13);
#[cfg(feature = "guava_20")]
useit!(guava_20_14);
#[cfg(feature = "guava_20")]
useit!(guava_20_15);
#[cfg(feature = "guava_20")]
useit!(guava_20_16);
#[cfg(feature = "guava_20")]
useit!(guava_20_17);
#[cfg(feature = "guava_20")]
useit!(guava_20_18);
#[cfg(feature = "guava_20")]
useit!(guava_20_19);
#[cfg(feature = "guava_21")]
useit!(guava_21_10);
#[cfg(feature = "guava_21")]
useit!(guava_21_11);
#[cfg(feature = "guava_21")]
useit!(guava_21_12);
#[cfg(feature = "guava_21")]
useit!(guava_21_13);
#[cfg(feature = "guava_21")]
useit!(guava_21_14);
#[cfg(feature = "guava_21")]
useit!(guava_21_15);
#[cfg(feature = "guava_21")]
useit!(guava_21_16);
#[cfg(feature = "guava_21")]
useit!(guava_21_17);
#[cfg(feature = "guava_21")]
useit!(guava_21_18);
#[cfg(feature = "guava_21")]
useit!(guava_21_19);
#[cfg(feature = "guava_21")]
useit!(guava_21_20);
#[cfg(feature = "guava_22")]
useit!(guava_22_10);
#[cfg(feature = "guava_22")]
useit!(guava_22_11);
#[cfg(feature = "guava_22")]
useit!(guava_22_12);
#[cfg(feature = "guava_22")]
useit!(guava_22_13);
#[cfg(feature = "guava_22")]
useit!(guava_22_14);
#[cfg(feature = "guava_22")]
useit!(guava_22_15);
#[cfg(feature = "guava_22")]
useit!(guava_22_16);
#[cfg(feature = "guava_22")]
useit!(guava_22_17);
#[cfg(feature = "guava_22")]
useit!(guava_22_18);
#[cfg(feature = "guava_22")]
useit!(guava_22_19);
#[cfg(feature = "guava_22")]
useit!(guava_22_20);
#[cfg(feature = "guava_22")]
useit!(guava_22_21);
#[cfg(feature = "guava_23")]
useit!(guava_23_10);
#[cfg(feature = "guava_23")]
useit!(guava_23_11);
#[cfg(feature = "guava_23")]
useit!(guava_23_12);
#[cfg(feature = "guava_23")]
useit!(guava_23_13);
#[cfg(feature = "guava_23")]
useit!(guava_23_14);
#[cfg(feature = "guava_23")]
useit!(guava_23_15);
#[cfg(feature = "guava_23")]
useit!(guava_23_16);
#[cfg(feature = "guava_23")]
useit!(guava_23_17);
#[cfg(feature = "guava_23")]
useit!(guava_23_18);
#[cfg(feature = "guava_23")]
useit!(guava_23_19);
#[cfg(feature = "guava_23")]
useit!(guava_23_20);
#[cfg(feature = "guava_23")]
useit!(guava_23_21);
#[cfg(feature = "guava_23")]
useit!(guava_23_22);
#[cfg(feature = "guava_24")]
useit!(guava_24_10);
#[cfg(feature = "guava_24")]
useit!(guava_24_11);
#[cfg(feature = "guava_24")]
useit!(guava_24_12);
#[cfg(feature = "guava_24")]
useit!(guava_24_13);
#[cfg(feature = "guava_24")]
useit!(guava_24_14);
#[cfg(feature = "guava_24")]
useit!(guava_24_15);
#[cfg(feature = "guava_24")]
useit!(guava_24_16);
#[cfg(feature = "guava_24")]
useit!(guava_24_17);
#[cfg(feature = "guava_24")]
useit!(guava_24_18);
#[cfg(feature = "guava_24")]
useit!(guava_24_19);
#[cfg(feature = "guava_24")]
useit!(guava_24_20);
#[cfg(feature = "guava_24")]
useit!(guava_24_21);
#[cfg(feature = "guava_24")]
useit!(guava_24_22);
#[cfg(feature = "guava_24")]
useit!(guava_24_23);
