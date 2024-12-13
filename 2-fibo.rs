fn main() {
    let x: i32 = 5;
    let ans = fibo(x);      
    println!("{}", ans);
}

// fibo of a number it takes as input
fn fibo(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }

    // Fix: Corrected range to 2..=num to calculate correctly.
    for _i in 2..=num {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}
