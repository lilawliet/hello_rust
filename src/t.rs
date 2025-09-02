fn compare<T: PartialOrd>(a: &T, b: &T) -> std::cmp::Ordering {
    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
}

fn main() {
    let result1 = compare(&5, &3);
    println!("5 与 3 比较 Result1: {:?}", result1);

    let result2 = compare(&"apple", &"banana");
    println!("apple 与 banana 比较 Result2: {:?}", result2);

    let result3 = compare(&3.14, &3.14);
    println!("3.14 与 3.14 比较 Result3: {:?}", result3);

}