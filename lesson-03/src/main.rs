fn main() {
    println!("{}", double_int32(123));
    println!("{}", double_int64(123));
    println!("{}", double_float32(123.0));
    println!("{}", double_float64(123.0));
    println!("{}", int_plus_float_to_float(123, 123.0));
    println!("{}", int_plus_float_to_int(123, 123.0));
    println!("{}", tuple_sum((123, 123)));
    println!("{}", array_sum([123, 123, 123]));
}

fn double_int32(x: i32) -> i32 {
    x * 2
}

fn double_int64(x: i32) -> i64 {
    x as i64 * 2_i64
}

fn double_float32(x: f32) -> f32 {
    x * 2_f32
}

fn double_float64(x: f32) -> f64 {
    x as f64 * 2_f64
}

fn int_plus_float_to_float(x: i32, y: f32) -> f64{
    (x as f32 + y) as f64
}

fn int_plus_float_to_int(x: i32, y: f32) -> i64{
    (x as f32 + y) as i64
}

fn tuple_sum(tup: (i32,i32)) -> i32 {
    tup.0 + tup.1
}

fn array_sum(arr: [i32; 3]) -> i32 {
    let mut sum: i32 = 0;
    for x in arr {
        sum += x;
    }
    sum
}