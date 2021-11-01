#[macro_use] extern crate hello_macro_macro;
#[macro_use] extern crate enum_iterator;

#[derive(HelloMacro)]
struct FrenchToast;

#[derive(HelloMacro)]
struct Waffles;

#[derive(EnumIterator, Debug)]
pub enum Bruce {
    Wenxin,
    Grace
}

fn main() {
    FrenchToast::hello_macro();
    Waffles::hello_macro();

    for e in Bruce::enum_iterator() {
        println!("Bruce::{:?}", e);
    }
}
