// MIT/Apache2 License

// A helper macro to create paramaterizing structures.
#[doc(hidden)]
#[macro_export]
macro_rules! create_paramaterizer {
    (
        $vis: vis struct $sname: ident : ($flags: ident, $base: ident) {
            $($field: ident ($setter: ident, $var: ident) : $fty: ty),*
        }
    ) => {
        #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
        $vis struct $sname {
            $(pub $field: Option<$fty>),*
        }

        impl $sname {
            /// Convert to the flags and set the appropriate fields on the request.
            #[inline]
            pub(crate) fn convert_to_flags(&self, req: &mut $base) -> $flags {
                // create the default instance of the flags to modify
                let mut flags: $flags = Default::default();

                $(
                    if let Some(ref t) = self.$field {
                        <$flags>::$setter(&mut flags, true);
                        req.$var = t.clone();
                    }
                )*

                flags
            }
        }
    }
}
