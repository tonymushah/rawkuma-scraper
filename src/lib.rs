mod client;
pub mod constant;
pub mod parser;
pub mod types;
pub use client::RawKumaClient;
pub mod enums;
pub use url::Url;

#[cfg(feature = "specta")]
mod impls {
    use specta::*;
    macro_rules! impl_as {
        ($($ty:path as $tty:ident)+) => {$(
            impl Type for $ty {
                fn inline(opts: DefOpts, generics: &[DataType]) -> Result<DataType, ExportError> {
                    <$tty as Type>::inline(opts, generics)
                }

                fn reference(opts: DefOpts, generics: &[DataType]) -> Result<DataType, ExportError> {
                    <$tty as Type>::reference(opts, generics)
                }
            }
        )+};
    }
    impl_as!(
        crate::Url as String
    );
}
