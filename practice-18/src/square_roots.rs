#[derive(Debug, PartialEq)]
pub enum Roots {
    None,
    Single(f64),
    Both(f64, f64)
}
pub fn find_square_roots(a: f64, b: f64, c: f64) ->  Roots {

    let d = b.powf(2.0) - 4.0 * a * c;
    if d < 0.0 {
        return Roots::None;
    }
    return if d == 0.0 {
        Roots::Single(-b / 2.0 * a)
    } else {
        Roots::Both((-b - f64::sqrt(d)) / 2.0 * a, (-b + f64::sqrt(d)) / 2.0 * a)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test = Roots::Single(2.0);
        let r = find_square_roots(1.0, -4.0, 4.0);
        assert_eq!(test, r);

        let test = Roots::Both(-3.0, 1.0);
        let r = find_square_roots(1.0, 2.0, -3.0);
        assert_eq!(test, r);

        let test = Roots::None;
        let r = find_square_roots(2.0, 2.0, 54.0);
        assert_eq!(test, r);
    }
}
