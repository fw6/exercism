pub fn square(s: u32) -> u64 {
    if s > 64 || s < 1 {
        panic!("Square must be between 1 and 64");
    }

    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // S = 2^0 + 2^1 + 2^2 + ... + 2^n
    // 2S =      2^1 + 2^2 + ... + 2^n + 2^(n+1)
    // 2S - S = 2^(n+1) - 2^0
    // S = 2^(n+1) - 1
    2u64.overflowing_pow(64).0.overflowing_sub(1).0
}
