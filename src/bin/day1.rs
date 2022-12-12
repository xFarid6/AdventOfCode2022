fn main () {
    // Read the file into a string
    let input = std::fs::read_to_string("src/bin/input.txt").unwrap();
    let mut max = 0;
    let mut sum = 0;

    for line in input.lines() {
        if !line.is_empty() {
            let num = line.parse::<i32>().unwrap();
            sum += num;
            if sum > max {
                max = sum;
            }
        } else {
            sum = 0;
        }
    }
    println!("Max: {}", max);
}
