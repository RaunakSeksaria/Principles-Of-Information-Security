// AI generated: mostly
use crate::utils::math_utils::*;
use num_bigint::BigUint;
use num_integer::Integer;

#[cfg(test)]
mod mod_exponentiation_tests {
    use super::*;

    // ========================================================================
    // EDGE CASES - Test boundary conditions
    // ========================================================================

    #[test]
    fn test_modulus_one() {
        // Anything mod 1 is 0
        let result = mod_exponentiation_num_bigint(
            &BigUint::from(5u32),
            &BigUint::from(3u32),
            &BigUint::from(1u32)
        );
        assert_eq!(result, BigUint::from(0u32));
    }

    #[test]
    fn test_exponent_zero() {
        // x^0 = 1 for any x != 0
        let result = mod_exponentiation_num_bigint(
            &BigUint::from(5u32),
            &BigUint::from(0u32),
            &BigUint::from(10u32)
        );
        assert_eq!(result, BigUint::from(1u32));
    }

    #[test]
    fn test_base_zero() {
        // 0^n = 0 for any n > 0
        let result = mod_exponentiation_num_bigint(
            &BigUint::from(0u32),
            &BigUint::from(5u32),
            &BigUint::from(10u32)
        );
        assert_eq!(result, BigUint::from(0u32));
    }

    #[test]
    #[should_panic(expected = "attempt to calculate with zero modulus!")]
    fn test_panic_on_zero_modulus() {
        mod_exponentiation_num_bigint(
            &BigUint::from(5u32),
            &BigUint::from(3u32),
            &BigUint::ZERO
        );
    }

    // ========================================================================
    // CORRECTNESS - Verify against known results
    // ========================================================================

    #[test]
    fn test_small_calculation() {
        // 2^3 mod 5 = 8 mod 5 = 3
        let result = mod_exponentiation_num_bigint(
            &BigUint::from(2u32),
            &BigUint::from(3u32),
            &BigUint::from(5u32)
        );
        assert_eq!(result, BigUint::from(3u32));
    }

    #[test]
    fn test_medium_calculation() {
        // 2^100 mod 1000000007 (known result)
        let result = mod_exponentiation_num_bigint(
            &BigUint::from(2u32),
            &BigUint::from(100u32),
            &BigUint::from(1000000007u32)
        );
        assert_eq!(result, BigUint::from(976371285u32));
    }

    #[test]
    fn test_fermat_little_theorem() {
        // Fermat's Little Theorem: a^(p-1) ≡ 1 (mod p) for prime p
        // Test: 3^6 mod 7 = 1 (since 7 is prime)
        let result = mod_exponentiation_num_bigint(
            &BigUint::from(3u32),
            &BigUint::from(6u32),  // p-1 where p=7
            &BigUint::from(7u32)
        );
        assert_eq!(result, BigUint::from(1u32));
    }

    // ========================================================================
    // LARGE NUMBER HANDLING - Test beyond u64 limits
    // ========================================================================

    #[test]
    fn test_large_base() {
        let base = BigUint::parse_bytes(b"123456789012345678901234567890", 10).unwrap();
        let exp = BigUint::from(50u32);
        let modulus = BigUint::parse_bytes(b"987654321098765432109876543210987", 10).unwrap();
        
        let result = mod_exponentiation_num_bigint(&base, &exp, &modulus);
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_exponent() {
        let base = BigUint::from(7u32);
        let exp = BigUint::parse_bytes(b"98765432109876543210987654321", 10).unwrap();
        let modulus = BigUint::from(1000000007u32);
        
        let result = mod_exponentiation_num_bigint(&base, &exp, &modulus);
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_modulus() {
        let base = BigUint::from(12345u32);
        let exp = BigUint::from(6789u32);
        let modulus = BigUint::parse_bytes(b"999999999999999999999999999999999", 10).unwrap();
        
        let result = mod_exponentiation_num_bigint(&base, &exp, &modulus);
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_large() {
        // All parameters beyond u64::MAX
        let base = BigUint::parse_bytes(b"987654321098765432109876543210", 10).unwrap();
        let exp = BigUint::parse_bytes(b"123456789012345678901234567", 10).unwrap();
        let modulus = BigUint::parse_bytes(b"999999999999999999999999999999991", 10).unwrap();
        
        let result = mod_exponentiation_num_bigint(&base, &exp, &modulus);
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    // ========================================================================
    // STRESS TESTS - Performance and correctness under load
    // ========================================================================

    #[test]
    fn test_very_large_exponent() {
        // 2^(10^30) mod (large prime)
        let base = BigUint::from(2u32);
        let exp = BigUint::parse_bytes(b"1000000000000000000000000000000", 10).unwrap();
        let modulus = BigUint::parse_bytes(b"1000000007", 10).unwrap();
        
        let result = mod_exponentiation_num_bigint(&base, &exp, &modulus);
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_result_less_than_modulus() {
        // Verify result is always < modulus
        let base = BigUint::from(12345u32);
        let exp = BigUint::from(67890u32);
        let modulus = BigUint::from(98765u32);
        
        let result = mod_exponentiation_num_bigint(&base, &exp, &modulus);
        
        assert!(result < modulus);
    }
}

#[cfg(test)]
mod gcd_tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        let result = gcd(&BigUint::from(48u32), &BigUint::from(18u32));
        assert_eq!(result, BigUint::from(6u32));
    }

    #[test]
    fn test_gcd_coprime() {
        // GCD of coprime numbers is 1
        let result = gcd(&BigUint::from(17u32), &BigUint::from(19u32));
        assert_eq!(result, BigUint::from(1u32));
    }

    #[test]
    fn test_gcd_one_zero() {
        // GCD(n, 0) = n
        let result = gcd(&BigUint::from(42u32), &BigUint::ZERO);
        assert_eq!(result, BigUint::from(42u32));
    }

    #[test]
    fn test_gcd_same_number() {
        // GCD(n, n) = n
        let n = BigUint::from(123u32);
        let result = gcd(&n, &n);
        assert_eq!(result, BigUint::from(123u32));
    }

    #[test]
    fn test_gcd_large() {
        let a = BigUint::parse_bytes(b"123456789012345678901234567890", 10).unwrap();
        let b = BigUint::parse_bytes(b"987654321098765432109876543210", 10).unwrap();
        
        let result = gcd(&a, &b);
        let expected = a.gcd(&b);
        
        assert_eq!(result, expected);
    }
}