fn main() {
    let mut b = 108400;
    let mut h = 0;
    loop {
        for d in 2..b {
            if b % d == 0 {
                h += 1;
                break;
            }
        }
        if b == 125400 {
            break;
        }
        b += 17;
    }
    println!("h: {}", h);
}