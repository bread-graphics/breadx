// MIT/Apache2 License

#[cfg(feature = "std")]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_std {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(feature = "std"))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_std {
    ($($i:item)*) => {};
}

#[cfg(feature = "std")]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_no_std {
    ($($i:item)*) => {};
}

#[cfg(not(feature = "std"))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_no_std {
    ($($i:item)*) => {$($i)*};
}

#[cfg(all(feature = "std", unix))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_std_unix {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(all(feature = "std", unix)))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_std_unix {
    ($($i:item)*) => {};
}

#[cfg(all(feature = "std", windows))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_std_windows {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(all(feature = "std", windows)))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_std_windows {
    ($($i:item)*) => {};
}

#[cfg(feature = "async")]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_async {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(feature = "async"))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_async {
    ($($i:item)*) => {};
}

#[cfg(feature = "sync_display")]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_sync {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(feature = "sync_display"))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_sync {
    ($($i:item)*) => {};
}

#[cfg(feature = "pl")]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_pl {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(feature = "pl"))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_pl {
    ($($i:item)*) => {};
}

#[cfg(not(feature = "pl"))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_not_pl {
    ($($i:item)*) => {$($i)*};
}

#[cfg(feature = "pl")]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_not_pl {
    ($($i:item)*) => {};
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_test {
    ($($i:item)*) => {$($i)*};
}

#[cfg(not(test))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_test {
    ($($i:item)*) => {};
}
