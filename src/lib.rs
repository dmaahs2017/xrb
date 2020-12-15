#![allow(dead_code)]
#![allow(unused_imports)]


pub mod xrb;

#[cfg(test)]
mod tests {
    use crate::xrb::types::BitGravity;
    #[test]
    fn test() {
        println!("{:?}", BitGravity::North);
        let l = BitGravity::North;
        let r = "North";
        let x: u8 = l.into();
    }
}
