fn fibo(i: i32) -> i32 {
    if i == 1 {
        return 0;
    }
    if i == 2 {
        return 1;
    }
    return fibo(i - 1) + fibo(i - 2);
}

fn main() -> i32 {
    let a: i32 = 7;
    return fibo(a);
}
