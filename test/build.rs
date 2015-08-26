extern crate ffigen;

fn main() {
    let mut context = ffigen::new_context();

    context.add_lang(ffigen::Lang::CSharp, &[ffigen::Config::Output("CSharp".to_string())]);
    ffigen::gen(&context);
}