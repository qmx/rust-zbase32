
extern crate zbase32;

fn main() {
    use zbase32::ToZBase32;

    println!("{}", "0".as_bytes().to_zbase32());
}
