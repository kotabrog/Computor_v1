pub mod math_utility {
    pub fn checked_abs(a: i64) -> Option<i64> {
        if a < 0 {
            a.checked_neg()
        } else {
            Some(a)
        }
    }

    fn gcd_loop(a: i64, b: i64) -> i64 {
        let mut a = a;
        let mut b = b;
        while b > 0 {
            (a, b) = (b, a - (a / b) * b);
        }
        a
    }

    pub fn checked_gcd(a: i64, b: i64) -> Option<i64> {
        let a = match checked_abs(a) {
            Some(v) => v,
            None => return None
        };
        let b = match checked_abs(b) {
            Some(v) => v,
            None => return None
        };
        if a == 0 {
            return Some(b);
        }
        if b == 0 {
            return Some(a);
        }
        Some(gcd_loop(a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::math_utility::*;

    #[test]
    fn checked_abs_zero() {
        assert_eq!(checked_abs(0), Some(0));
    }

    #[test]
    fn checked_abs_plus() {
        assert_eq!(checked_abs(1), Some(1));
    }

    #[test]
    fn checked_abs_neg() {
        assert_eq!(checked_abs(-1), Some(1));
    }

    #[test]
    fn checked_abs_overflow() {
        assert_eq!(checked_abs(-9223372036854775808), None);
    }

    #[test]
    fn checked_gcd_0_0() {
        assert_eq!(checked_gcd(0, 0), Some(0));
    }

    #[test]
    fn checked_gcd_2_0() {
        assert_eq!(checked_gcd(2, 0), Some(2));
    }

    #[test]
    fn checked_gcd_0_2() {
        assert_eq!(checked_gcd(0, 2), Some(2));
    }

    #[test]
    fn checked_gcd_small() {
        assert_eq!(checked_gcd(4, 6), Some(2));
    }

    #[test]
    fn checked_gcd_big() {
        assert_eq!(checked_gcd(96, 84), Some(12));
    }

    #[test]
    fn checked_gcd_minus_plus() {
        assert_eq!(checked_gcd(-4, 6), Some(2));
    }

    #[test]
    fn checked_gcd_plus_minus() {
        assert_eq!(checked_gcd(4, -6), Some(2));
    }

    #[test]
    fn checked_gcd_minus_minus() {
        assert_eq!(checked_gcd(-4, -6), Some(2));
    }
}