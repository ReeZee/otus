#![allow(dead_code)]
/*
    Дан массив, который содержит n неповторяющихся чисел в диапазоне
    от 0 до n включительно.

    Написать функцию, которая вернёт единственное число, отсутствующее
    в данном массиве.

    Гарантируется, что числа в массиве не повторяются и все принадлежат
    заданному диапазону.
*/

fn missing_num(nums: &[i32]) -> i32 {
    let mut nums_vec = Vec::new();

    for num in nums {
        nums_vec.push(*num);
    }
    nums_vec.sort();
    println!("{:?}", nums_vec);
    for (idx, num) in nums_vec.iter().enumerate() {
        if idx as i32 != *num {
            return idx as i32;
        }
    }
    99
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(missing_num(&[1, 2]), 0);
        assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
        assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
    }
}
