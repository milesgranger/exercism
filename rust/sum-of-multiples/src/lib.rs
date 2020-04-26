pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {

    let factors_wo_zeros: Vec<&u32> = factors.iter().filter(|f| **f != 0).collect();

    (1..limit)
        .filter(|v| factors_wo_zeros.iter().any(|f| v % *f == 0))
        .sum()
}
