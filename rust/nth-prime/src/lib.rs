pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2, 3];
    let cap = n as usize;

    while primes.len() <= cap {
        let mut candidate = primes.last().unwrap() + 2;

        while primes.iter().any(|p| candidate % p == 0) {
            candidate += 2;
        }
        primes.push(candidate);
    }
    primes[n as usize]
}

// pub fn nth(n: u32) -> u32 {
//     (2..).filter(|n| is_prime(*n)).nth(n as usize).unwrap()
// }

// fn is_prime(x: u32) -> bool {
//     let sqrt = (x as f32).sqrt() as u32;
//     (2..=sqrt).all(|n| x % n != 0)
// }
