extern crate ffigen;

fn main() {
    let mut context = ffigen::new_context();

    context.add_lang(ffigen::Lang::Cpp, &[ffigen::Config::Output("Cpp".to_string())]);
    ffigen::gen(&context);
}