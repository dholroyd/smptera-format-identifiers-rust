use crate::database::Record;
use codegen::{Scope, Const};
use convert_case::{Case, Casing};

fn create_const_name(text: &str) -> String {
    create_base_name(text)
        .to_uppercase()
}
fn create_variant_name(text: &str) -> String {
    create_base_name(text)
        .to_case(Case::Pascal).to_string()
}
fn create_base_name(text: &str) -> String {
    text.replace("-", "_")
}


pub(crate) fn gen(box_list: &[Record], scope: &mut Scope) {
    gen_enum(&box_list, scope);
    gen_from_fourcc(&box_list, scope);
    gen_consts(&box_list, scope);
}

fn gen_enum(records: &[Record], scope: &mut Scope) {
    let mut boxes = codegen::Enum::new("FormatIdentifier");
    boxes.doc("_Format Identifier_ values recorded by the SMPTE Registration Authority")
        .derive("Debug")
        .derive("Clone")
        .derive("Copy")
        .vis("pub");
    for id in records {
        let code = &id.ascii_value;
        let var_name = create_variant_name(&code);
        boxes.new_variant(&var_name)
            .doc(&format!("FourCC: `{}`, Registered by: _{}_\n\nIntention:\n\n{}", id.ascii_value, id.org_name, intention(id)));
    }
    boxes.new_variant("Unknown")
        .tuple("four_cc::FourCC");
    scope.push_enum(boxes);
}

fn intention(record: &Record) -> String {
    record.intention
        .lines()
        .map(|l| format!("> {}", l)).collect::<Vec<String>>().join("\n")
}

fn gen_from_fourcc(records: &[Record], scope: &mut Scope) {
    let mut try_from_impl = codegen::Impl::new("FormatIdentifier");
    try_from_impl.impl_trait("From<four_cc::FourCC>");
    let from_fn = try_from_impl.new_fn("from");
    from_fn.ret("Self")
        .arg("val", "four_cc::FourCC");
    from_fn.line("match val {");
    for id in records {
        let code = &id.ascii_value;
        let con_name = create_const_name(&code);
        let var_name = create_variant_name(&code);
        from_fn.line(format!("    {}::{} => {}::{},", "format_identifier", &con_name, "FormatIdentifier", &var_name));
    }
    from_fn.line(format!("    _ => {}::{},", "FormatIdentifier", "Unknown(val)"));
    from_fn.line("}");

    scope.push_impl(try_from_impl);
}

fn gen_consts(records: &[Record], scope: &mut Scope) {
    let module = scope.new_module("format_identifier");
    module.doc(&format!("`FourCC`'s for _Format Identifiers_ recorded by the SMPTE Registration Authority"));
    module.vis("pub");
    for id in records {
        let code = &id.ascii_value;
        let var_name = create_const_name(&code);
        let mut con = Const::new(&var_name, "four_cc::FourCC", &format!("four_cc::FourCC(*b{:?})", code));
        con.vis("pub");
        con.doc(&format!("FourCC: `{}`, Registered by: _{}_\n\nIntention:\n\n{}", id.ascii_value, id.org_name, intention(id)));
        module.push_const(con);
    }
}
