#[cfg(not(test))]
runtime::binInit!();
#[cfg(test)]
runtime::addtest!();

fn main() {}
