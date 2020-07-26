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
    gen_consts(&box_list, scope);
}

fn intention(record: &Record) -> String {
    record.intention
        .lines()
        .map(|l| format!("> {}", l)).collect::<Vec<String>>().join("\n")
}

fn gen_consts(records: &[Record], scope: &mut Scope) {
    // struct FormatIdentifier itself is in lib.rs, only the const definitions are auto-generated
    let mut struct_impl = codegen::Impl::new("FormatIdentifier");
    for id in records {
        let code = &id.ascii_value;
        let var_name = create_const_name(&code);
        let mut con = Const::new(&var_name, "FormatIdentifier", &format!("FormatIdentifier(FourCC(*b{:?}))", code));
        con.vis("pub");
        con.doc(&format!("FourCC: `{}`, Registered by: _{}_\n\nIntention:\n\n{}", id.ascii_value, id.org_name, intention(id)));
        struct_impl.push_const(con);
    }
    scope.push_impl(struct_impl);
}
