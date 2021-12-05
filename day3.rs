fn main () {
    let str_in: String = std::fs::read_to_string("./day3.txt").expect("Failed to read");
    
    let mut bits: Vec<u32> = vec![0; 12];
    let mut thresh: u32 = 0;

    
    for line in str_in.lines() {
        thresh += 1;
        for i in 0..line.len() {
            bits[i] += line.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
    }

    thresh /= 2;
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;

    println!("thresh: {}", thresh);
    println!("bits: {:?}", bits);
    for i in 0..(bits.len() as u32) {

        println!("index: {}", bits[i as usize]);

        if bits[bits.len() - (1 + i as usize)] >= thresh {
            gamma += 1 << i;
        }
        else {
            epsilon += 1 << i;
        }
    }
    println!("gamma: {:05b}, {}", gamma, gamma);
    println!("epsilon: {:05b}, {}", epsilon, epsilon);
    println!("product: {}", gamma*epsilon);
}