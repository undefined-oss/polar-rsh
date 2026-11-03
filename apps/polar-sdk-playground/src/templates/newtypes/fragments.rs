use proc_macro2::TokenStream;
use quote::quote;

use super::Scalar;

pub(super) struct Check {
    condition: TokenStream,
    variant: syn::Ident,
    message: String,
}

impl Check {
    fn new(condition: TokenStream, variant: &str, message: String) -> Self {
        Self {
            condition,
            variant: syn::Ident::new(variant, proc_macro2::Span::call_site()),
            message,
        }
    }

    pub(super) fn validation(&self, error: &syn::Ident) -> TokenStream {
        let Self { condition, variant, .. } = self;
        quote! {
            if #condition {
                return Err(#error::#variant);
            }
        }
    }

    pub(super) fn error_variant(&self) -> TokenStream {
        let Self { variant, message, .. } = self;
        quote! {
            #[error(#message)]
            #variant,
        }
    }
}

pub(super) fn checks(scalar: &Scalar) -> Vec<Check> {
    let mut checks = Vec::new();
    match scalar {
        Scalar::String { minimum, maximum } | Scalar::Url { minimum, maximum } => {
            if let Some(minimum) = minimum {
                checks.push(Check::new(
                    quote!(length < #minimum),
                    "TooShort",
                    format!("must contain at least {minimum} characters"),
                ));
            }
            if let Some(maximum) = maximum {
                checks.push(Check::new(
                    quote!(length > #maximum),
                    "TooLong",
                    format!("must contain at most {maximum} characters"),
                ));
            }
        }
        Scalar::Integer { minimum, maximum, .. } => {
            if let Some(minimum) = minimum {
                checks.push(Check::new(
                    quote!(i128::from(value) < #minimum),
                    "BelowMinimum",
                    format!("must be at least {minimum}"),
                ));
            }
            if let Some(maximum) = maximum {
                checks.push(Check::new(
                    quote!(i128::from(value) > #maximum),
                    "AboveMaximum",
                    format!("must be at most {maximum}"),
                ));
            }
        }
        Scalar::Number { minimum, maximum } => {
            checks.push(Check::new(
                quote!(!value.is_finite()),
                "NotFinite",
                "must be finite".into(),
            ));
            if let Some(bound) = minimum {
                let value = bound.value;
                let comparison = if bound.exclusive { quote!(<=) } else { quote!(<) };
                let relation = if bound.exclusive { "greater than" } else { "at least" };
                checks.push(Check::new(
                    quote!(value #comparison #value),
                    "BelowMinimum",
                    format!("must be {relation} {value}"),
                ));
            }
            if let Some(bound) = maximum {
                let value = bound.value;
                let comparison = if bound.exclusive { quote!(>=) } else { quote!(>) };
                let relation = if bound.exclusive { "less than" } else { "at most" };
                checks.push(Check::new(
                    quote!(value #comparison #value),
                    "AboveMaximum",
                    format!("must be {relation} {value}"),
                ));
            }
        }
    }
    checks
}

pub(super) fn preparation(scalar: &Scalar) -> TokenStream {
    match scalar {
        Scalar::String { .. } => quote!(let length = value.chars().count() as u64;),
        Scalar::Url { .. } => quote!(let length = value.as_str().chars().count() as u64;),
        _ => TokenStream::new(),
    }
}

pub(super) fn constructor(scalar: &Scalar, ty: &syn::Type, error: &syn::Ident) -> TokenStream {
    let (input, value) = match scalar {
        Scalar::String { .. } => (quote!(impl Into<String>), quote!(value.into())),
        _ => (quote!(#ty), quote!(value)),
    };
    quote! {
        /// Validate a value.
        pub fn try_new(value: #input) -> Result<Self, #error> {
            Self::try_from(#value)
        }
    }
}

pub(super) fn conversions(scalar: &Scalar, name: &syn::Ident, error: &syn::Ident) -> TokenStream {
    match scalar {
        Scalar::String { .. } => quote! {
            impl TryFrom<&str> for #name {
                type Error = #error;

                fn try_from(value: &str) -> Result<Self, Self::Error> {
                    Self::try_from(value.to_owned())
                }
            }

            impl AsRef<str> for #name {
                fn as_ref(&self) -> &str {
                    &self.0
                }
            }
        },
        Scalar::Url { .. } => quote! {
            impl AsRef<url::Url> for #name {
                fn as_ref(&self) -> &url::Url {
                    &self.0
                }
            }
        },
        _ => TokenStream::new(),
    }
}
