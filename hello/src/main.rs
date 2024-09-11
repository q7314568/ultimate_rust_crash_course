fn main() {
    println!("Hello, world!");

    let small_array: [i32; 32] = [0; 32]; // 可以自動實現 Debug
    println!("{:?}", small_array); // 這是合法的

    let large_array: [i32; 33] = [0; 33]; // 超過 32 的限制
    println!("{:?}", large_array); // 編譯錯誤，因為無法自動實現 Debug
}
