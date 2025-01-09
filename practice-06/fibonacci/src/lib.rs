#![allow(dead_code)]
/*
    Последовательностью Фибоначчи называется последовательность чисел,
    которая удовлетворяет следующим условиям:
    - элемент последовательности с индексом 0 - число 0
    - элемент с индексом 1 - число 1
    - каждый последующий элемент равен сумме двух предыдущих.

    0, 1, 1, 2, 3, 5, 8, 13, 21 ...

    Написать функцию, которая вычислит элемент последовательности с индексом n.

    * Написать вторую функцию, которая вернёт последовательность Фибонначи
      от первого элемента до n-ого. Написать тесты
*/

fn fib(n: u32) -> u32 {
    let fib_type = Fib::FibOptimized;
    if n <= 1 {
        return n;
    };

    match fib_type {
        Fib::FibBasic => {
            if n > 20 {
                panic!("May take too long!");
            }
            let mut num = 0;
            let mut i = 1;
            while i < n {
                num = fib(i - 1) + fib(i);
                i += 1;
            }
            num
        }
        Fib::FibOptimized => {
            let num = fib_with_previous(0, n, (0, 1));
            num.0

        }
    }
}

fn fib_with_previous(mut current: u32, goal:u32 , mut values: (u32, u32)) -> (u32, u32) {
    if current == goal {
        values
    } else {
        current += 1;
        values = (values.0 + values.1, values.0);

        fib_with_previous(current, goal, values)
    }
}

fn fib_list(n: u32) -> Vec<u32> {
    let mut fib_list: Vec<u32> = Vec::new();
    for num in 0..=n {
        fib_list.push(fib(num));
    }
    fib_list
}

enum Fib {
    FibBasic,
    FibOptimized
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
        assert_eq!(fib(22), 17711);
        assert_eq!(fib(23), 28657);
        assert_eq!(fib(24), 46368);
        assert_eq!(fib(25), 75025);
        assert_eq!(fib(26), 121393);
        println!("{}",fib(47));
    }
    #[test]
    fn list_works() {
        assert_eq!(fib_list(8), vec!(0, 1, 1, 2, 3, 5, 8, 13, 21));
        println!("{:?}", fib_list(47));
    }
}
