#![allow(dead_code)]
/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при этом цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/

fn digit_product(n: u32) -> u8 {

    // When 0..10 return the value
    if is_single_digit(n) {
        return n as u8;
    } else if n == 10 {
        return 1;
    }
    let mut n = n;
    let mut current = 1;
    let product = loop {
        let (left, digit) = (n / 10, n % 10); // Get the last digit and keep what is left
        // Skip digit 0
        if digit == 0 {
            n /= 10;
            continue;
        }
        current = current * digit; // Current product
        // Break the loop when there is nothing left and the product is a single digit
        if left == 0 && is_single_digit(current) {
            break current;
        }
        // Continue the loop with the current product
        if left == 0 {
            n = current;
            current = 1;
            continue;
        }
        n = left;
    };
     product as u8
}

fn is_single_digit(n: u32) -> bool {
    if n < 10{
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2); // 9*8*7=504, 5*4=20, 2
        assert_eq!(digit_product(123456), 4); // 1*2*3*4*5*6=720, 7*2=14, 1*4=4
        assert_eq!(digit_product(123454321), 6); // 1*2*3*4*5*4*3*2*1=2880, 2*8*8=128, 1*2*8=16, 1*6=6
        assert_eq!(digit_product(1200031), 6); // 1*2*3*4*5*4*3*2*1=2880, 2*8*8=128, 1*2*8=16, 1*6=6
        assert_eq!(digit_product(17000321), 8); // 1*2*3*4*5*4*3*2*1=2880, 2*8*8=128, 1*2*8=16, 1*6=6
        assert_eq!(digit_product(77777), 2); // 1*2*3*4*5*4*3*2*1=2880, 2*8*8=128, 1*2*8=16, 1*6=6
        assert_eq!(digit_product(7700777), 2); // 1*2*3*4*5*4*3*2*1=2880, 2*8*8=128, 1*2*8=16, 1*6=6
    }

    #[test]
    fn single_digit() {
        assert_eq!(is_single_digit(0), true);
        assert_eq!(is_single_digit(5), true);
        assert_eq!(is_single_digit(9), true);
        assert_eq!(is_single_digit(3), true);
        assert_eq!(is_single_digit(10), false);
        assert_eq!(is_single_digit(12), false);
        assert_eq!(is_single_digit(50), false);
        assert_eq!(is_single_digit(18), false);
    }
}
