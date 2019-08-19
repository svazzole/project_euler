fn main() {
    println!("Hello, Rust!");
    println!("This is Problem 1.");
    problem1();
    problem2();
    problem3();
}

fn problem1() {
    println!("Running problem 1...");
    let m: u32 = 1000;
    let mut s: u32 = 0; 
    for i in 3..m {
        if (i%3 == 0) | (i%5 == 0) {
            s += i
        }
    }
    println!("The sum is {}", s);
}

fn problem2() {
    println!("Running problem 2...");
    let m: u32 = 4000000;
    let mut s: u32 = 0;
    let mut prev_fib_num: u32 = 1;
    let mut cur_fib_num: u32 = 1;
    let mut tmp_fib_num: u32;

    while cur_fib_num < m {
        if cur_fib_num % 2 == 0 {
            s += cur_fib_num;
        }
        tmp_fib_num = cur_fib_num;
        cur_fib_num = prev_fib_num + tmp_fib_num;
        prev_fib_num = tmp_fib_num;
    }

    println!("The sum is {}", s);
}

fn is_prime(x: u64) -> bool {
    if x%2 == 0 {
        return false;
    } else {
        let sqrt_x: u64 = (x as f64).sqrt() as u64;
        for i in (3..sqrt_x).step_by(2) {
            if x%i == 0 {
                return false;
            }
        }
    }

    return true;

}

fn problem3() {
    println!("Running problem 3...");
    let prime: u64 = 600851475143;
    let sqrt_prime: u64 = (prime as f64).sqrt() as u64;
    let mut x = sqrt_prime;
    if x%2 == 0 {
        x -= 1;
    }
    let mut largest_factor: u64 = 2;
    let mut stop = false;

    while !stop {
        if (prime%x == 0) & (is_prime(x)) {
            largest_factor = x;
            stop = true;
        }
        x -= 2;
    }
    println!("The largest prime factor of {} is {}", prime, largest_factor)
}