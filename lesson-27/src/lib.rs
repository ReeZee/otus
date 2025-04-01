/*
Описание/Пошаговая инструкция выполнения домашнего задания:

Реализуем сортировку слиянием: https://en.wikipedia.org/wiki/Merge_sort

Это одна из самых эффективных сортировок, потому что гарантирует O(N log N) сложность во всех случаях.

Вам требуется реализовать эту сортировку на Rust, но с ограничением: нельзя писать for или while циклы, используйте только итераторы.

Напишите функцию merge_sort(input: &[u64]) -> Vec и покройте свое решение тестами.

Подсказка: посмотрите на std::iter::from_fn.
*/

pub fn merge_sort(input: &[u64]) {
    let temp = std::iter::from_fn(move || divide(input))
        .inspect(|n| println!("Element: {:?}", n))
        .collect::<Vec<&[u64]>>();
    dbg!(temp);
}

fn divide(input: &[u64]) -> Option<&[u64]> {
    let length = input.len();
    if length == 1 {
        println!("{:?}", input);
        Some(input)
    } else {
        let (left, right) = input.split_at(length / 2);
        divide(left);
        divide(right);

        println!("end");
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_test() {
        let x = [6, 2, 8, 3, 9, 234, 76, 33, 75, 22, 343, 7, 45, 2, 1, 675];
        merge_sort(&x);
    }
}
