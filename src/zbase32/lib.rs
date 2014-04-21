#![crate_id = "zbase32"]
#![license = "MIT"]
#![crate_type = "lib"]

mod zbase32 {

    use std::str;
    pub trait ToZBase32 {
        fn to_zbase32(&self) -> ~str;
    }

    impl<'a> ToZBase32 for &'a [u8] {
        fn to_zbase32(&self) -> ~str {
            let mut v = Vec::new();
            unsafe {
                str::raw::from_utf8_owned(v.move_iter().collect())
            }
        }
    }

}
#[cfg(test)]
mod tests {
    extern crate test;
    use zbase32::ToZBase32;

    #[test]
    fn test_to_zbase32_basic() {
        assert_eq!("".as_bytes().to_zbase32(), "".to_owned());
    }
}

