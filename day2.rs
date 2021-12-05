fn main() {
    let str_in: String = std::fs::read_to_string("./day2.txt").expect("Failed to read");
    // let input: Vec<String> = str_in.lines().map(|l| l.parse().unwrap()).collect();

    let mut aim: i32 = 0;
    let mut height: i32 = 0;
    let mut pos: i32 = 0;

    for i in str_in.lines() {
        let mut splitter = i.split_whitespace();
        
        // println!("{}", i);
        match splitter.next() {
        
            Some("forward") => {
                let tmp: i32 = splitter.next().unwrap().parse().unwrap();
                pos += tmp; 
                height += aim * tmp
            },
            
            Some("down") => aim += splitter.next().unwrap().parse::<i32>().unwrap(),
        
            Some("up") => aim -= splitter.next().unwrap().parse::<i32>().unwrap(),

            _ => println!("panicked"),
        }
    }
    println!("height: {}", height);
    println!("pos: {}", pos);
    println!("product: {}", pos*height);
}
