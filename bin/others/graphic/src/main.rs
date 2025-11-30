#![cfg_attr(not(test), no_std, no_main)]

#[cfg(not(test))]
runtime::binInit!();
#[cfg(test)]
runtime::addtest!();

fn main() {}
