pub fn fib_of_nth_number(n: u32) -> u32 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        fib_of_nth_number(n - 1) + fib_of_nth_number(n - 2)
    }
}
