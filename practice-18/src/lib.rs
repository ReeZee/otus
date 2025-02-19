mod square_roots;

pub fn find_needle_in_haystack(haystack: &[&str], needle: &str) -> Option<usize> {
    for (idx,option) in haystack.iter().enumerate() {
        if *option == needle {
            return Some(idx)
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = ["aa", "bb", "cc"];
        let needle = "bb";
        let r = find_needle_in_haystack(&haystack, needle);
        assert_eq!(1, r.unwrap());
    }
}
