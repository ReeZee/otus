#![allow(dead_code)]
/*
    Написать функцию, которая превращает число в строку по следующим правилам:
    1. Если число кратно 3, то возвращаем строку "Fizz"
    2. Если число кратно 5, то возвращаем строку "Buzz"
    3. Если число кратно и 3, и 5, то возвращаем строку "FizzBuzz"
    4. В остальных случаях возвращаем строку, содержащую данное число

    Написать функцию fizzbuzz_list, которая получает число `n: u32` и возвращает
    список строк, содержащих строковые представления fizzbuzz
    для чисел в диапазоне от 1 до n. Написать тесты.
*/

fn fizzbuzz(num: u32) -> String {
    match (num % 3, num % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => num.to_string(),
    }
}

fn fizzbuzz_list(num: u32) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for n in 1..=num {
        list.push(fizzbuzz(n));
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(&fizzbuzz(1), "1");
        assert_eq!(&fizzbuzz(3), "Fizz");
        assert_eq!(&fizzbuzz(5), "Buzz");
        assert_eq!(&fizzbuzz(7), "7");
        assert_eq!(&fizzbuzz(9), "Fizz");
        assert_eq!(&fizzbuzz(15), "FizzBuzz");
        assert_eq!(&fizzbuzz(30), "FizzBuzz");
        assert_eq!(&fizzbuzz(49), "49");
    }

    #[test]
    fn still_works() {
        assert_eq!(&fizzbuzz_list(5), &["1", "2", "Fizz", "4", "Buzz"]);
    }
}
