// #![feature(test)]
// extern crate test;
// mod bench;
pub mod universe;
mod utils;

#[cfg(test)]
mod tests {
    use fixedbitset::FixedBitSet;

    #[test]
    fn test() {
        let mut test = FixedBitSet::with_capacity(16);
        test.clear();
        println!("{:#016b}", test);
    }
}
