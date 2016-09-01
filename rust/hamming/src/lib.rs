pub fn hamming_distance<'a>(strand_a: &str, strand_b: &str) -> Result<usize, &'a str> {
    if strand_a.len() != strand_b.len() {
        return Result::Err("Invalid strand comparison.");
    }
    let distance = strand_a.chars().zip(strand_b.chars())
        .filter(|&(a, b)| a != b)
        .count();
    Result::Ok(distance)
}