extern crate syntex_syntax;

use std::fs;
use std::path::Path;

use syntex_syntax::ast;
use syntex_syntax::abi;
use syntex_syntax::parse;
use syntex_syntax::ptr::P;
use syntex_syntax::visit;

#[derive(Debug)]
pub enum ParseError {
    FileIO,
    Metadata
}

pub struct FuncDecl {
    name: ast::Name,
    ret: ast::FunctionRetTy,
    args: Vec<ast::Arg>
}

struct FnVisitor<'a> {
    exports: &'a mut Vec<FuncDecl>
}

impl<'a> FnVisitor<'a> {
    fn new(exports: &'a mut Vec<FuncDecl>) -> FnVisitor<'a> {
        FnVisitor { exports: exports }
    }
}

impl<'v> visit::Visitor<'v> for FnVisitor<'v> {
    fn visit_item(&mut self, item: &'v ast::Item) {
        match item.node {
            ast::ItemFn(ref decl, _, _, abi, _, _) if abi == abi::C => {
                self.exports.push(FuncDecl {
                            name: item.ident.name,
                            ret: decl.output.clone(),
                            args: decl.inputs.clone()
                        });

                println!("fn {:?} {:?} {:?}\n", item.ident.name, decl, item.vis)
            },
            _ => ()
        }
    }
}

pub fn parse_dir(path: &Path) -> Result<Vec<FuncDecl>, ParseError> {
    let mut exports = Vec::new();

    println!("Searching {:?}", path);
    let paths = fs::read_dir(path);

    match paths {
        Err(e) => {
            println!("Unable to read {:?} {}", path, e);
            return Err(ParseError::FileIO);
        },
        _ => ()
    }

    for tpath in paths.unwrap() {
        let path = match tpath {
            Ok(v) => v,
            Err(e) => {
                println!("Unable to read path {}", e);
                return Err(ParseError::FileIO);
            }
        };

        let meta = match path.metadata() {
            Ok(v) => v,
            Err(e) => {
                println!("Unable to get metadata for {:?} {}", path.file_name(), e);
                return Err(ParseError::Metadata);
            }
        };

        if meta.is_file() {
            let src = String::from(
                    match path.path().to_str() {
                        Some(v) => v,
                        None => {
                            println!("Unable to get os path");
                            return Err(ParseError::FileIO);
                        }
                    });
            println!("Parsing {}", src);
            parse_src(&src, &mut exports);
        }
    }

    Ok(exports)
}

pub fn parse_src(src: &String, exports: &mut Vec<FuncDecl>) {
    let path = Path::new(src);

    let cfg: Vec<P<ast::MetaItem>> = Vec::new();
    let sess = parse::ParseSess::new();
    let krate = parse::parse_crate_from_file(path, cfg, &sess);

    let mut visitor = FnVisitor::new(exports);
    visit::walk_crate(&mut visitor, &krate);
}