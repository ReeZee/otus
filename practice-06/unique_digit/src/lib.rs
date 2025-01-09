#![allow(dead_code)]
/*
    Дана строка, состоящая только из цифровых символов. В данной строке
    есть одна цифра, которая не повторяется. Написать функцию,
    которая найдёт эту цифру и вернёт её.

    * Написать похожую функцию, но только на этот раз в данной строке
    могут присутствовать любые символы, а уникальная цифра может отсутствовать.
    Но если присутствует, то не больше одной. Написать тесты.
*/

fn uniq_digit(s: &str) -> u8 {
    let hits = count_digit_hits(s);
    let mut unique: u8 = 99;
    for (digit, hit_count) in hits.into_iter().enumerate() {
        if hit_count == 1 {
            unique = digit as u8;
        }
    }
    unique
}

fn uniq_digit_or_not(s: &str) -> Option<u8> {
    let hits = count_digit_hits(s);
    let mut unique = None;
    for (digit, hit_count) in hits.into_iter().enumerate() {
        if hit_count == 1 {
            unique = Some(digit as u8);
        }
    }
    unique
}

fn count_digit_hits(s: &str) -> [u8; 10] {
    let mut hits: [u8; 10] = [0; 10];
    for symbol in s.chars().into_iter() {
        match symbol {
            '0' => hits[0] += 1,
            '1' => hits[1] += 1,
            '2' => hits[2] += 1,
            '3' => hits[3] += 1,
            '4' => hits[4] += 1,
            '5' => hits[5] += 1,
            '6' => hits[6] += 1,
            '7' => hits[7] += 1,
            '8' => hits[8] += 1,
            '9' => hits[9] += 1,
            _ => ()
        }
    }
    hits

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(uniq_digit("3"), 3);
        assert_eq!(uniq_digit("010"), 1);
        assert_eq!(uniq_digit("47343077"), 0);
        assert_eq!(uniq_digit("123454321"), 5);
        assert_eq!(uniq_digit("0987654321234567890"), 1);
        assert_eq!(uniq_digit("4444444444424444444444444"), 2);
    }

    #[test]
    fn it_works_v2() {
        assert_eq!(uniq_digit_or_not("3"), Some(3));
        assert_eq!(uniq_digit_or_not("010"), Some(1));
        assert_eq!(uniq_digit_or_not("47343077"), Some(0));
        assert_eq!(uniq_digit_or_not("123454321"), Some(5));
        assert_eq!(uniq_digit_or_not("0987654321234567890"), Some(1));
        assert_eq!(uniq_digit_or_not("4444444444424444444444444"), Some(2));
        assert_eq!(uniq_digit_or_not("444444444442222224444444444444"), None);
        assert_eq!(uniq_digit_or_not("44444dsjkfha  44444422222244444dsfdf   44444444"), None);
        assert_eq!(uniq_digit_or_not("44444dsjkfha  441444422222244444dsfdf   44444444"), Some(1));
    }
}