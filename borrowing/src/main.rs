fn take_ownership_sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += value
    }
    sum
}

fn borrow_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += *value
    }
    sum
}

fn example_1() {
    let values = vec![1, 2, 3, 4, 5];
    let sum_1 = take_ownership_sum(values);

    // Works
    println!("sum = {}", sum_1);

    // Does not work
    // borrow of moved value: `values`
    // println!("sum of {} items = {}", values.len(), sum_1);
}

fn example_2() {
    let values = vec![1, 2, 3, 4, 5];
    let sum_2 = borrow_sum(&values);

    // Works
    println!("sum = {}", sum_2);

    // Works
    println!("sum of {} items = {}", values.len(), sum_2);
}

fn cap_values_owned(max: i32, mut v: Vec<i32>) -> Vec<i32> {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max
        }
    }
    v
}

fn example_3() {
    let mut values = vec![1, 2, 3, 10000, 5];
    values = cap_values_owned(10, values);

    for v in values {
        println!("{}", v)
    }
}

fn cap_values_borrowed(max: i32, v: &mut Vec<i32>) {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max
        }
    }
}

fn example_4() {
    let mut values = vec![1, 2, 3, 10000, 5];
    cap_values_borrowed(10, &mut values);

    for v in values {
        println!("{}", v)
    }
}

fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
}
