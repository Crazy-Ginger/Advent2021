fn main() {
    let str_in: String = std::fs::read_to_string("./day1.txt").expect("Failed to read");
    let input: Vec<u32> = str_in.lines().map(|l| l.parse().unwrap()).collect();

    let mut counter: i64 = 0;
    //for i in 0..input.len() - 1 {
    //if input[i] < input[i + 1] {
    //counter += 1;
    //}
    //}
    for i in 3..input.len() {
        if (input[i - 3] + input[i - 2] + input[i - 1]) < (input[i - 2] + input[i - 1] + input[i]) {
            counter += 1;
        }
    }
    println!("counter: {}", counter);
}
