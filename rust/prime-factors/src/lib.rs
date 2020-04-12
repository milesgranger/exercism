pub fn factors(n: u64) -> Vec<u64> {

    let mut fctrs = vec![];

    let mut nn = n;

    (2..=n).map(|v| v)
        .filter(is_prime)
        .for_each(|v| {
            while nn % v == 0 {
                nn /= v;
                fctrs.push(v);
            }
        });
    fctrs
}

fn is_prime(n: &u64) -> bool {
    !(2..*n - (*n as f64 / 2.0) as u64).any(|v| n % &v == 0)
}
