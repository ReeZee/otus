#![allow(dead_code)]
/*
    Дана строка, состоящая только из символов '{', '}', '(', ')', '[', ']'.
    Такая строка является корректной, если:
    - каждой открывающей скобке соответствует закрывающая того же типа
    - соблюдается порядок закрытия скобок
    - для каждой закрывающей скобки есть соответствующая открывающая пара

    Написать функцию, которая проверит корректность данной строки.
*/

fn validate_paren(s: &str) -> bool {
    let mut chars: Vec<i8> = Vec::new();
    for p in s.chars() {
        match p {
            '(' => chars.push(1),
            ')' => chars.push(-1),
            '[' => chars.push(2),
            ']' => chars.push(-2),
            '{' => chars.push(3),
            '}' => chars.push(-3),
            _ => return false,
        }
    }
    println!("{:?}", chars);
    let mut value = 0;
    let mut moddified = chars.clone();
    'outer: loop {
        for (i, p) in chars.clone().into_iter().enumerate() {
            value += p;
            if p < 0 {
                if moddified[i - 1] + p == 0 {
                    moddified[i] = 0;
                    moddified[i - 1] = 0;
                }
            }
        }
        for (i, p) in moddified.clone().into_iter().enumerate() {
            if p != 0 {
                continue 'outer;
            } else {
                moddified.remove(i);
                continue 'outer;
            }
        }
        break;
    }
    println!("Moddified: {:?}\n", moddified);

    if value == 0 {
        true
    } else {
        false
    }
}

/*
fn validate_paren(s: &str) -> bool {
    let mut last_change = 0;
    let mut value = 0;
    println!("For string: {s}");
    for p in s.chars() {
        let mut current_change: i32 = 0;
        match p {
            '(' => current_change += 1,
            ')' => current_change -= 1,
            '[' => current_change += 2,
            ']' => current_change -= 2,
            '{' => current_change += 3,
            '}' => current_change -= 3,
            _ => return false,
        }
        if current_change < 0 {
            println!(
                "last_change + current_change: {} + {} = {}",
                last_change,
                current_change,
                last_change + current_change
            );
            if last_change + current_change != 0 && value == 0 {
                return false;
            } else if value + current_change < 0 {
                println!(
                    "value + current_change: {} + {} = {}",
                    value,
                    current_change,
                    value + current_change
                );
                return false;
            }
        } else {
            last_change = current_change;
        }
        value += current_change;
        //        println!(
        //            "Change: {}, value: {}, last_value: {}",
        //            change, value, last_value
        //        );
    }
    println!("Result: {value}");
    if value == 0 {
        true
    } else {
        false
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(validate_paren("()"), true);
        assert_eq!(validate_paren("()[]{}"), true);
        assert_eq!(validate_paren("({[]()})"), true);
        assert_eq!(validate_paren("(}"), false);
        assert_eq!(validate_paren("({{{{{{{{)}}}}}}}}"), false);
        assert_eq!(validate_paren("{((((((((}))))))))"), false);
        assert_eq!(validate_paren("()]"), false);
        assert_eq!(validate_paren("(}){"), false);
        assert_eq!(validate_paren("(){]}"), false);
        assert_eq!(validate_paren("([){]}"), false);
    }
}
