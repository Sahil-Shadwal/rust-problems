fn main() {
    let ans = is_even(14+1);
    println!("{}", ans);
}

fn is_even(n: i32) -> bool {
    if n%2 == 0{
        return true;
    } else{
        return false;
    }
}