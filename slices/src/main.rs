fn main() {
    let a = vec![1036, 7, 2345, 774];
    let sla = &a[1..2];
    println!("slice of a = {:?}", sla); // [7]

    let b = String::from("Hello");
    let slb = &b[0..2];
    println!("slice of b = {}", slb); // He
}
