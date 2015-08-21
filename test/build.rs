extern crate ffigen;

fn main() {
    println!("build");
    ffigen::gen_cargo();

    panic!();
}