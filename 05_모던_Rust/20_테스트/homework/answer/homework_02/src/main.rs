// 핵심 포인트:
// - 0, 1 은 소수가 아니므로 일찍 분기.
// - i * i <= n 조건으로 √n 까지만 검사 — 오버플로 걱정 없는 안전한 방식.

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i: u32 = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    println!("is_prime(7) = {}", is_prime(7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn below_two() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
    }

    #[test]
    fn primes() {
        for n in [2, 3, 7, 13] {
            assert!(is_prime(n), "{n} 는 소수여야 함");
        }
    }

    #[test]
    fn composites() {
        assert!(!is_prime(4));
        assert!(!is_prime(9));
    }
}
