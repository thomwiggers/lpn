/// These codes are from the supplemental material of
/// Bogos and Vaudenay, 2016 which appeared at Asiacrypt
macro_rules! useit {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    };
}


useit!(bogosrnd_18_6);
useit!(bogosrnd_19_6);
useit!(bogosrnd_19_7);
