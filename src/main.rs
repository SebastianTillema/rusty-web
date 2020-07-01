// used as a entry point for debugging

mod cards;

fn main() {
    println!("Hello World!");

    let a = cards::Card::new("hllow".to_string());
    print!("{}\n", a.text);
}