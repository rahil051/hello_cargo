pub fn exec(nth_length: i32) -> i32 {
    if nth_length == 1 {
        0
    } else if nth_length == 2 {
        1
    } else {
        exec(nth_length - 1) + exec(nth_length - 2)
    }
}
