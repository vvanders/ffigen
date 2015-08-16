extern crate syntex_syntax;

use std::path::Path;

use syntex_syntax::ast;
use syntex_syntax::abi;
use syntex_syntax::parse;
use syntex_syntax::ptr::P;
use syntex_syntax::visit;

struct FnVisitor;

impl<'v> visit::Visitor<'v> for FnVisitor {
    fn visit_item(&mut self, item: &'v ast::Item) {
        match item.node {
            ast::ItemFn(_, _, _, abi, _, _) if abi == abi::C => {
                println!("fn {:?}\n", item.ident.name)
            },
            _ => ()
        }
    }
}

/*
fn main() {
    let path = Path::new("D:/rust/ffigen/src/sample.rs");

    let cfg: Vec<P<ast::MetaItem>> = Vec::new();
    let sess = parse::ParseSess::new();
    let krate = parse::parse_crate_from_file(path, cfg, &sess);

    let mut visitor = FnVisitor;
    visit::walk_crate(&mut visitor, &krate);

    //println!("\nast {:?}", krate);
}
*/