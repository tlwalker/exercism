fn sum_of_nums(n: u32) -> u32 {
    (n + n.pow(2)) / 2
}
pub fn square_of_sum(n: u32) -> u32 {
    sum_of_nums(n).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}