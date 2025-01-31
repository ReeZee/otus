// Слайсы. (мы спрашиваем эту задачку на собеседования на уровено Junior Engineer)
// Ring Buffer (кольцевой буффер) - структура данных, которая позволяет очень удобно реализовывать очередь на массиве фиксированного размера.
// https://ru.wikipedia.org/wiki/%D0%9A%D0%BE%D0%BB%D1%8C%D1%86%D0%B5%D0%B2%D0%BE%D0%B9_%D0%B1%D1%83%D1%84%D0%B5%D1%80
// Ключевая идея в том, что заполняя буффер до конца мы переходим в начало
// Пример API, вызовов и как меняется состояние буффера:
// [ _ _ _ ] create(3)
// [ a b _ ] write "ab" -> return 2
// [ a b c ] write "cd" -> return 1
// [ _ b c ] read(1) -> return "a"
// [ e b c ] write "e" -> return 1
// [ e _ _ ] read(2) -> return "bc"
// Ваша задача написать такой буффер и добавить тесты

struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
}

fn create(size: usize) -> RingBuffer {
    RingBuffer{read_idx: 0, write_idx: 0, data: Vec::with_capacity(size)}
}

fn write(rb: &mut RingBuffer, elements: &[u8]) -> usize {
    let mut written = 0;
    if rb.data.len() <= rb.data.capacity() {
        for element in elements.into_iter() {
            rb.data.push(element);
        }
    }
    written
}

fn read(rb: &mut RingBuffer, num_of_elements: usize) -> () {
}

#[cfg(test)]
mod tests {
    use super::*;

    // обязательно добавьте тесты
    #[test]
    fn test_1() {
        let mut rb = create(10);
        println!("{}", rb.data.capacity());
        rb.data.push(42);
        println!("{}", rb.data.len());
        rb.data.push(42);
        write(&mut rb, "as".as_bytes());
        println!("{}", rb.data.len());
    }
}