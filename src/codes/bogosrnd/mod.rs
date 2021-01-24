//! Random codes from the supplemental material of Bogos and Vaudenay, 2016 which appeared at Asiacrypt

#[allow(unused_macros)]
macro_rules! useit {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}

#[cfg(feature = "bogosrnd_18")]
useit!(bogosrnd_18_6);
#[cfg(feature = "bogosrnd_19")]
useit!(bogosrnd_19_6);
#[cfg(feature = "bogosrnd_19")]
useit!(bogosrnd_19_7);
