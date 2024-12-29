pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    fn steps(current: u64, count: u64) -> u64 {
        match current {
            1 => count,
            _ if current % 2 == 0 => steps(current / 2, count + 1),
            _ => steps(current * 3 + 1, count + 1),
        }
    }

    Some(steps(n, 0))
}
