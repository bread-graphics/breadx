// MIT/Apache2 License

#[cfg(feature = "std")]
#[macro_export]
macro_rules! cfg_std {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! cfg_std {
    ($($i:item)*) => {};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! cfg_no_std {
    ($($i:item)*) => {};
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! cfg_no_std {
    ($($i:item)*) => {$($i)*};
}

#[cfg(all(feature = "std", unix))]
#[macro_export]
macro_rules! cfg_std_unix {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(all(feature = "std", unix)))]
#[macro_export]
macro_rules! cfg_std_unix {
    ($($i:item)*) => {};
}

#[cfg(all(feature = "std", windows))]
#[macro_export]
macro_rules! cfg_std_windows {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(all(feature = "std", windows)))]
#[macro_export]
macro_rules! cfg_std_windows {
    ($($i:item)*) => {};
}

#[cfg(feature = "async")]
#[macro_export]
macro_rules! cfg_async {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(feature = "async"))]
#[macro_export]
macro_rules! cfg_async {
    ($($i:item)*) => {};
}

#[cfg(test)]
#[macro_export]
macro_rules! cfg_test {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(test))]
#[macro_export]
macro_rules! cfg_test {
    ($($i:item)*) => {};
}
