fn main() {
    println!("{}", a::do_macro_a!());
    println!("{}", a::do_macro_b!());
    println!("{}", a::do_macro_macro_b!());
    println!("{}", a::do_macro_macro_b2!());
    println!("{}", a::do_macro_macro_b3!());
}
