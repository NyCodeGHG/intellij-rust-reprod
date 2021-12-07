use itertools::Itertools;

fn main() {
    // itertools is NOT available in this crate, but not shown as an error
    let _test = vec![1, 2, 3].iter().collect_vec();
}
