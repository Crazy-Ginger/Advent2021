fn main() {
    part_one();
    part_two();
}

fn part_one() {
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

    for i in 0..(bits.len() as u32) {
        if bits[bits.len() - (1 + i as usize)] >= thresh {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }
    println!("gamma: {:05b}, {}", gamma, gamma);
    println!("epsilon: {:05b}, {}", epsilon, epsilon);
    println!("product: {}", gamma * epsilon);
}

fn part_tooer(mut oxy: Vec<String>, flag: bool) -> u32 {
    let bit_length = oxy[0].len();

    for b in 0..bit_length {
        if oxy.len() == 1 {
            break;
        }

        let oxy_sum = oxy
            .iter()
            .filter(|num| num.chars().nth(b) == Some('1'))
            .count();

        let bit_threshold = (oxy.len() as f32 / 2.0).ceil() as usize;
        let oxy_flag = if flag {
            if oxy_sum >= bit_threshold {
                '1'
            } else {
                '0'
            }
        } else {
            if oxy_sum < bit_threshold {
                '1'
            } else {
                '0'
            }
        };
        oxy = oxy
            .into_iter()
            .filter(|o| o.chars().nth(b) != Some(oxy_flag))
            .collect();
    }

    return u32::from_str_radix(&oxy[0], 2).unwrap();
}

fn part_two() {
    let oxy: Vec<String> = std::fs::read_to_string("./day3.txt")
        .expect("Failed to read")
        .lines()
        .map(str::to_string)
        .collect();
    let scrub = oxy.clone();

    println!(
        "answer: {}",
        part_tooer(oxy, true) * part_tooer(scrub, false)
    );
}
