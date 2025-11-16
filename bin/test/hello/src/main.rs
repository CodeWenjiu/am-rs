#![cfg_attr(not(test), no_std, no_main)]

#[cfg(not(test))]
runtime::binInit!();
#[cfg(not(test))]
runtime::entry!(main);

fn main() {
    println!("Hello_world!");
}

#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn test_hello() {
        println!("hello test!!!");
        assert_eq!(1, 1);
    }

    #[test]
    fn test_main() {
        main();
    }
}
