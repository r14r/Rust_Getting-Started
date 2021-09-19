fn factorial_recursive(i: u64) -> u64 {
    match i {
        0 => 1,
        n => n * factorial_recursive(n - 1),
    }
}
fn factorial_iterative(i: u64) -> u64 {
    let mut acc = 1;
    for num in 2..=i {
        acc *= num;
    }
    acc
}

fn factorial_with_iterators(i: u64) -> u64 {
    (1..=i).product()
}

fn main() {
    
    for num in 20..25 {
        println!("Calculate factorial for {}", num);

        println!("Recursive:      {} {} {}" , factorial_recursive(num)
                                            , factorial_iterative(num)
                                            , factorial_with_iterators(num));
    }
}
