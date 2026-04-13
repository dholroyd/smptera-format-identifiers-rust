use crate::database::Record;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

fn create_const_name(text: &str) -> String {
    create_base_name(text.trim())
        .to_uppercase()
}
fn create_variant_name(text: &str) -> String {
    create_base_name(text)
        .to_case(Case::Pascal).to_string()
}
fn create_base_name(text: &str) -> String {
    text.replace("-", "_")
}

pub(crate) fn gen(records: &[Record]) -> TokenStream {
    gen_consts(records)
}

fn intention(record: &Record) -> String {
    record.intention
        .lines()
        .map(|l| format!("> {}", l)).collect::<Vec<String>>().join("\n")
}

fn gen_consts(records: &[Record]) -> TokenStream {
    let consts: Vec<TokenStream> = records.iter().map(|id| {
        let code = &id.ascii_value;
        let var_name = Ident::new(&create_const_name(code), Span::call_site());
        let doc_text = format!(
            "FourCC: `{}`, Registered by: _{}_\n\nIntention:\n\n{}",
            id.ascii_value, id.org_name, intention(id)
        );
        let doc_lines: Vec<TokenStream> = doc_text.lines().map(|line| {
            let line = format!(" {}", line);
            quote! { #[doc = #line] }
        }).collect();
        let byte_str = syn::LitByteStr::new(code.as_bytes(), Span::call_site());
        quote! {
            #(#doc_lines)*
            pub const #var_name: FormatIdentifier = FormatIdentifier(FourCC(*#byte_str));
        }
    }).collect();

    quote! {
        impl FormatIdentifier {
            #(#consts)*
        }
    }
}
