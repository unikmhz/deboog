use darling::{FromDeriveInput, FromField, FromMeta, FromVariant, ast::{Data, Fields, Style}};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, Index, spanned::Spanned};

#[derive(Clone, Copy, Default, FromMeta)]
#[darling(default, rename_all = "snake_case")]
enum Masking {
    Pan,
    PanSuffix,
    #[default]
    All,
    Hidden,
}

type OptionData = Data<VariantOptions, FieldOptions>;

#[derive(FromDeriveInput)]
#[darling(attributes(deboog))]
struct Options {
    ident: Ident,
    data: OptionData,
}

#[derive(FromField)]
#[darling(attributes(deboog))]
struct FieldOptions {
    ident: Option<Ident>,
    #[darling(default)]
    skip: bool,
    #[darling(default)]
    mask: Option<Masking>,
}

#[derive(FromVariant)]
#[darling(attributes(deboog))]
struct VariantOptions {
    ident: Ident,
    fields: Fields<FieldOptions>,
}

#[proc_macro_derive(Deboog, attributes(deboog))]
pub fn derive_deboog(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Options::from_derive_input(&input).unwrap();
    let debug_impl = debug_fmt_impl(&opts.ident, &opts.data);

    let output = quote! { #debug_impl };
    output.into()
}

fn debug_fmt_impl(ident: &Ident, data: &OptionData) -> TokenStream2 {
    let debug_fmt = debug_fmt_body(ident, data);
    quote! {
        #[automatically_derived]
        impl std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                #debug_fmt
            }
        }
    }
}

fn debug_fmt_body(ident: &Ident, data: &OptionData) -> TokenStream2 {
    match data {
        Data::Enum(variants) => debug_fmt_enum(variants),
        Data::Struct(fields)=> match fields.style {
            Style::Unit => debug_fmt_unit_struct(ident),
            Style::Struct => debug_fmt_normal_struct(ident, &fields.fields),
            Style::Tuple => debug_fmt_tuple_struct(ident, &fields.fields),
        }
    }
}

fn debug_fmt_unit_struct(ident: &Ident) -> TokenStream2 {
    let ident_str = ident.to_string();
    quote! {
        f.debug_struct(#ident_str).finish()
    }
}

fn debug_fmt_normal_struct(ident: &Ident, fields: &[FieldOptions]) -> TokenStream2 {
    let ident_str = ident.to_string();
    let field_chunks = fields
        .iter()
        .filter(|f| !f.skip)
        .map(|f| {
            let field = &f.ident;
            let field_str = field.to_token_stream().to_string();
            let field_val = transform_field(quote! { &self.#field }, f);
            quote! { .field(#field_str, #field_val) }
        });
    quote! {
        f.debug_struct(#ident_str)
            #(#field_chunks)*
            .finish()
    }
}

fn debug_fmt_tuple_struct(ident: &Ident, fields: &[FieldOptions]) -> TokenStream2 {
    let ident_str = ident.to_string();
    let field_chunks = fields
        .iter()
        .enumerate()
        .filter(|(_, f)| !f.skip)
        .map(|(i, f)| {
            let i = Index::from(i);
            let field_val = transform_field(quote! { &self.#i }, f);
            quote! { .field(#field_val) }
        });
    quote! {
        f.debug_tuple(#ident_str)
            #(#field_chunks)*
            .finish()
    }
}

fn debug_fmt_enum(variants: &[VariantOptions]) -> TokenStream2 {
    let variant_chunks = variants
        .iter()
        .map(|v| {
            let var = &v.ident;
            let var_str = var.to_string();

            if v.fields.is_unit() {
                quote! {
                    Self::#var => {
                        f.debug_struct(#var_str).finish()
                    }
                }
            } else if v.fields.is_tuple() {
                let fields = v.fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        if f.skip {
                            Ident::new("_", v.ident.span())
                        } else {
                            Ident::new(&format!("f{}", i), v.ident.span())
                        }
                    });
                let field_chunks = v.fields
                    .iter()
                    .enumerate()
                    .filter(|(_, f)| !f.skip)
                    .map(|(i, f)| {
                        Ident::new(&format!("f{}", i), f.ident.span())
                    });
                quote! {
                    Self::#var(#(#fields),*) => {
                        f.debug_tuple(#var_str)
                            #(.field(#field_chunks))*
                            .finish()
                    }
                }
            } else {
                let fields = v.fields
                    .iter()
                    .filter(|f| !f.skip)
                    .map(|f| &f.ident);
                let variant_fields = fields
                    .clone()
                    .map(|i| {
                        let field_str = i.to_token_stream().to_string();
                        quote! { .field(#field_str, #i) }
                    });
                quote! {
                    Self::#var { #(#fields,)* .. } => {
                        f.debug_struct(#var_str)
                            #(#variant_fields)*
                            .finish()
                    }
                }
            }
        });
    quote! {
        match self {
            #(#variant_chunks),*
        }
    }
}

fn transform_field(field: TokenStream2, opts: &FieldOptions) -> TokenStream2 {
    match opts.mask {
        None => field,
        Some(masking) => {
            let mask_method = Ident::new(match masking {
                Masking::Pan => "mask_pan",
                Masking::PanSuffix => "mask_pan_suffix",
                Masking::All => "mask_all",
                Masking::Hidden => "mask_hide",
            }, field.span());
            // TODO: maybe avoid importing trait explicitly?
            quote! { {
                use deboog::field::DeboogField;
                #field.#mask_method()
            } }
        }
    }
}
