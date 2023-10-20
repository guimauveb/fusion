extern crate proc_macro;

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, DeriveInput, Type},
};

/// Merge two instances of the same type by replacing fields in the source instance with
/// the fields containing a value in the second instance (fields set to `Some(thing)` or fields not wrapped in an `Option`).
///
/// Fields set to `None` in the second instance are left untouched in the source instance.
///
/// The `#[fusion]` attribute is used when the field type implements `Fusion` and that it should
/// be called when merging the parent struct.
///
/// # Example
/// ```rust
/// #[derive(Debug, PartialEq, Eq, Fusion)]
/// struct Foo {
///     a: Option<String>,
///     b: Option<usize>,
///     c: String,
/// }
///
/// let mut src = Foo {
///     a: Some("Bar".into()),
///     b: Some(7),
///     c: "One".into(),
/// };
/// let update = Foo {
///     a: None,
///     b: Some(8),
///     c: "Two".into(),
/// };
/// src.merge(update);
/// assert_eq!(
///     src,
///     Foo {
///         a: Some("Bar".into()),
///         b: Some(8),
///         c: "Two".into(),
///     }
/// );
/// ```
#[proc_macro_derive(Fusion, attributes(fusion))]
pub fn merge(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let data = if let syn::Data::Struct(data) = input.data {
        data
    } else {
        unimplemented!();
    };

    let fields = data.fields.iter().map(|f| {
        let is_opt = match &f.ty {
            Type::Path(typath) => {
                let idents_of_path =
                    typath
                        .path
                        .segments
                        .iter()
                        .fold(String::new(), |mut acc, v| {
                            acc.push_str(&v.ident.to_string());
                            acc.push(':');
                            acc
                        });
                ["Option:", "std:option:Option:", "core:option:Option:"]
                    .into_iter()
                    .find(|s| idents_of_path == *s)
                    .and_then(|_| typath.path.segments.last())
                    .is_some()
            }
            _ => false,
        };

        let ident = &f.ident;
        if f.attrs
            .iter()
            .find(|a| a.path().is_ident("fusion"))
            .is_some()
        {
            if is_opt {
                quote! {
                    if let Some(update) = update.#ident {
                        if let Some(state) = self.#ident.as_mut() {
                            state.merge(update);
                        } else {
                            self.#ident.replace(update);
                        }
                    }
                }
            } else {
                quote! {
                    self.#ident.merge(update);
                }
            }
        } else {
            if is_opt {
                quote! {
                    if let Some(update) = update.#ident {
                        self.#ident.replace(update);
                    }
                }
            } else {
                quote! {
                    self.#ident = update.#ident;
                }
            }
        }
    });

    let expanded = quote!(
        impl #name {
            /// Merge two instances of the same type by replacing fields in the source instance with
            /// the fields containing a value in the second instance (fields set to `Some(thing)` or fields not wrapped in an `Option`).
            ///
            /// Fields set to `None` in the second instance are left untouched in the source instance.
            /// ```rust ignore
            /// #[derive(Debug, PartialEq, Eq, Fusion)]
            /// struct Foo {
            ///     a: Option<String>,
            ///     b: Option<usize>,
            ///     c: String,
            /// }
            ///
            /// let mut src = Foo {
            ///     a: Some("Bar".into()),
            ///     b: Some(7),
            ///     c: "One".into(),
            /// };
            /// let update = Foo {
            ///     a: None,
            ///     b: Some(8),
            ///     c: "Two".into(),
            /// };
            /// src.merge(update);
            /// assert_eq!(
            ///     src,
            ///     Foo {
            ///         a: Some("Bar".into()),
            ///         b: Some(8),
            ///         c: "Two".into(),
            ///     }
            /// ```
            pub fn merge(&mut self, update: Self) {
                #(#fields)*
            }
        }
    );

    expanded.into()
}
