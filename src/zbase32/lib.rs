#![crate_id = "zbase32"]
#![license = "MIT"]
#![crate_type = "lib"]


use std::str;
use std::fmt;
pub trait ToZBase32 {
    fn to_zbase32(&self) -> ~str;
}

impl<'a> ToZBase32 for &'a [u8] {
    fn to_zbase32(&self) -> ~str {
        let bytes = bytes!("ybndrfg8ejkmcpqxot1uwisza345h769");
        let mut v = Vec::new();
        let mut i = 0;
        let len = self.len();
        while i < len - (len % 5) {
            let n = (self[i] as u32) << 32 |
                (self[i + 1] as u32) << 24 |
                (self[i + 2] as u32) << 16 |
                (self[i + 3] as u32) << 8 |
                (self[i + 4] as u32);
            println!("{:t}", n);

            // This 40-bit number gets separated into eight 5-bit numbers.
            v.push(bytes[((n >> 35) & 31) as uint]);
            v.push(bytes[((n >> 30) & 31) as uint]);
            v.push(bytes[((n >> 25) & 31) as uint]);
            v.push(bytes[((n >> 20) & 31) as uint]);
            v.push(bytes[((n >> 15) & 31) as uint]);
            v.push(bytes[((n >> 10) & 31) as uint]);
            v.push(bytes[((n >> 5) & 31) as uint]);
            v.push(bytes[(n & 31) as uint]);

            i += 5;
        }

        match len % 5 {
            0 => (),
            1 => {
                let n = (self[i] as u32) << 16;
                v.push(bytes[((n >> 15) & 31) as uint]);
                v.push(bytes[((n >> 10) & 31) as uint]);
                println!("{:t}", n);
            }
            2 => {
                let n = (self[i] as u32) << 16 |
                    (self[i + 1u] as u32) << 8;
                v.push(bytes[((n >> 15) & 31) as uint]);
                v.push(bytes[((n >> 10) & 31) as uint]);
                v.push(bytes[((n >> 5) & 31) as uint]);
                println!("{:t}", n);
            }
            3 => {
                let n = (self[i] as u32) << 32 |
                    (self[i + 1u] as u32) << 16 |
                    (self[i + 2u] as u32) << 8;
                v.push(bytes[((n >> 20) & 31) as uint]);
                v.push(bytes[((n >> 15) & 31) as uint]);
                v.push(bytes[((n >> 10) & 31) as uint]);
                v.push(bytes[((n >> 5) & 31) as uint]);
                println!("{:t}", n);

            }
            4 => {
                let n = (self[i] as u32) << 32 |
                    (self[i + 1u] as u32) << 24 |
                    (self[i + 2u] as u32) << 16;
                    (self[i + 3u] as u32) << 8;
                v.push(bytes[((n >> 25) & 31) as uint]);
                v.push(bytes[((n >> 20) & 31) as uint]);
                v.push(bytes[((n >> 15) & 31) as uint]);
                v.push(bytes[((n >> 10) & 31) as uint]);
                v.push(bytes[((n >> 5) & 31) as uint]);
                println!("{:t}", n);

            }
            _ => ()
        }
        unsafe {
            str::raw::from_utf8_owned(v.move_iter().collect())
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    extern crate zbase32;
    use zbase32::ToZBase32;

    #[test]
    fn test_to_zbase32_basic() {
        assert_eq!("".as_bytes().to_zbase32(), "".to_owned());
        assert_eq!("ab".as_bytes().to_zbase32(), "".to_owned());
    }
}

