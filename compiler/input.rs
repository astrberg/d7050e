fn test_i32(b: i32, c: i32) -> i32 {
    let a : i32 = 10 + 2 * 3;
    a = -1 - (1 - 1);
    return a + b;
}
fn test_bool(b: bool) -> bool {
    if b && true || false {
        let a : i32 = test_i32(1, 999);
        while (a < 5) {
            a = a + 1;
        }
        return false;
    }
    return (3 < 4);
}

fn main() {
    test_bool(true);
}
