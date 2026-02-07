// These are almost completely AI generated as of right now
use crate::utils::math_utils::*;
use num_bigint::BigUint;

#[cfg(test)]
mod mod_exponentiation_u32_tests {
    use super::*;

    #[test]
    fn test_edge_case_modulus_1() {
        // Any number mod 1 is always 0
        assert_eq!(mod_exponentiation_u32(5, 3, 1), 0);
        assert_eq!(mod_exponentiation_u32(100, 100, 1), 0);
    }

    #[test]
    fn test_edge_case_exponent_0() {
        // Any number^0 = 1
        assert_eq!(mod_exponentiation_u32(5, 0, 10), 1);
        assert_eq!(mod_exponentiation_u32(999, 0, 1000), 1);
        assert_eq!(mod_exponentiation_u32(2, 0, 5), 1);
    }

    #[test]
    fn test_edge_case_base_0() {
        // 0^n = 0 for any positive n
        assert_eq!(mod_exponentiation_u32(0, 5, 10), 0);
        assert_eq!(mod_exponentiation_u32(0, 100, 999), 0);
    }

    #[test]
    fn test_base_1() {
        // 1^n mod m = 1 for any n and m > 1
        assert_eq!(mod_exponentiation_u32(1, 10, 100), 1);
        assert_eq!(mod_exponentiation_u32(1, 999, 1000), 1);
    }

    #[test]
    fn test_small_values() {
        // 2^3 mod 5 = 8 mod 5 = 3
        assert_eq!(mod_exponentiation_u32(2, 3, 5), 3);
        
        // 3^4 mod 7 = 81 mod 7 = 4
        assert_eq!(mod_exponentiation_u32(3, 4, 7), 4);
        
        // 5^2 mod 13 = 25 mod 13 = 12
        assert_eq!(mod_exponentiation_u32(5, 2, 13), 12);
    }

    #[test]
    fn test_original_example() {
        // Original test case
        assert_eq!(mod_exponentiation_u32(5, 13, 23), 21);
    }

    #[test]
    fn test_power_of_2() {
        // 2^10 mod 1000 = 1024 mod 1000 = 24
        assert_eq!(mod_exponentiation_u32(2, 10, 1000), 24);
        
        // 2^8 mod 256 = 256 mod 256 = 0
        assert_eq!(mod_exponentiation_u32(2, 8, 256), 0);
        
        // 2^5 mod 31 = 32 mod 31 = 1
        assert_eq!(mod_exponentiation_u32(2, 5, 31), 1);
    }

    #[test]
    fn test_base_larger_than_modulus() {
        // 10^2 mod 7 = (10 mod 7)^2 mod 7 = 3^2 mod 7 = 2
        assert_eq!(mod_exponentiation_u32(10, 2, 7), 2);
        
        // 100^3 mod 13 = (100 mod 13)^3 mod 13 = 9^3 mod 13 = 729 mod 13 = 1
        assert_eq!(mod_exponentiation_u32(100, 3, 13), 1);
    }

    #[test]
    fn test_large_exponent() {
        // 3^100 mod 7
        assert_eq!(mod_exponentiation_u32(3, 100, 7), 4);
        
        // 2^31 mod 1000000 (testing large power)
        assert_eq!(mod_exponentiation_u32(2, 31, 1000000), 483648);
    }

    #[test]
    fn test_prime_modulus() {
        // Fermat's Little Theorem: a^(p-1) ≡ 1 (mod p) for prime p and gcd(a,p)=1
        // 2^(17-1) mod 17 = 2^16 mod 17 = 1
        assert_eq!(mod_exponentiation_u32(2, 16, 17), 1);
        
        // 3^(11-1) mod 11 = 3^10 mod 11 = 1
        assert_eq!(mod_exponentiation_u32(3, 10, 11), 1);
    }

    #[test]
    fn test_result_equals_base() {
        // 7^1 mod 10 = 7
        assert_eq!(mod_exponentiation_u32(7, 1, 10), 7);
        
        // 13^1 mod 100 = 13
        assert_eq!(mod_exponentiation_u32(13, 1, 100), 13);
    }

    #[test]
    #[should_panic(expected = "Modulus cannot be zero")]
    fn test_panic_on_zero_modulus() {
        mod_exponentiation_u32(5, 3, 0);
    }
}

#[cfg(test)]
mod mod_exponentiation_bigint_tests {
    use super::*;

    #[test]
    fn test_edge_case_modulus_1() {
        let result = mod_exponentiation_num_bigint(
            BigUint::from(5u32),
            BigUint::from(3u32),
            BigUint::from(1u32)
        );
        assert_eq!(result, BigUint::from(0u32));
    }

    #[test]
    fn test_edge_case_exponent_0() {
        let result = mod_exponentiation_num_bigint(
            BigUint::from(5u32),
            BigUint::from(0u32),
            BigUint::from(10u32)
        );
        assert_eq!(result, BigUint::from(1u32));
    }

    #[test]
    fn test_edge_case_base_0() {
        let result = mod_exponentiation_num_bigint(
            BigUint::from(0u32),
            BigUint::from(5u32),
            BigUint::from(10u32)
        );
        assert_eq!(result, BigUint::from(0u32));
    }

    #[test]
    fn test_small_values() {
        // 2^3 mod 5 = 3
        let result = mod_exponentiation_num_bigint(
            BigUint::from(2u32),
            BigUint::from(3u32),
            BigUint::from(5u32)
        );
        assert_eq!(result, BigUint::from(3u32));
    }

    #[test]
    fn test_large_numbers() {
        // Test with numbers beyond u32 range
        // 2^100 mod 1000000007
        let result = mod_exponentiation_num_bigint(
            BigUint::from(2u32),
            BigUint::from(100u32),
            BigUint::from(1000000007u32)
        );
        assert_eq!(result, BigUint::from(976371285u32));
    }

    #[test]
    fn test_very_large_exponent() {
        // 3^1000 mod 10000
        let result = mod_exponentiation_num_bigint(
            BigUint::from(3u32),
            BigUint::from(1000u32),
            BigUint::from(10000u32)
        );
        assert_eq!(result, BigUint::from(1u32));
    }

    #[test]
    fn test_extremely_large_base() {
        // Base larger than u64::MAX
        let base = BigUint::parse_bytes(b"123456789012345678901234567890", 10).unwrap();
        let exp = BigUint::from(50u32);
        let modulus = BigUint::parse_bytes(b"987654321098765432109876543210987", 10).unwrap();
        
        let result = mod_exponentiation_num_bigint(base.clone(), exp.clone(), modulus.clone());
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extremely_large_exponent() {
        // Exponent larger than u64::MAX
        let base = BigUint::from(7u32);
        let exp = BigUint::parse_bytes(b"98765432109876543210987654321", 10).unwrap();
        let modulus = BigUint::from(1000000007u32);
        
        let result = mod_exponentiation_num_bigint(base.clone(), exp.clone(), modulus.clone());
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extremely_large_modulus() {
        // Modulus larger than u64::MAX
        let base = BigUint::from(12345u32);
        let exp = BigUint::from(6789u32);
        let modulus = BigUint::parse_bytes(b"999999999999999999999999999999999", 10).unwrap();
        
        let result = mod_exponentiation_num_bigint(base.clone(), exp.clone(), modulus.clone());
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_large_values() {
        // All parameters extremely large
        let base = BigUint::parse_bytes(b"987654321098765432109876543210", 10).unwrap();
        let exp = BigUint::parse_bytes(b"123456789012345678901234567", 10).unwrap();
        let modulus = BigUint::parse_bytes(b"999999999999999999999999999999991", 10).unwrap();
        
        let result = mod_exponentiation_num_bigint(base.clone(), exp.clone(), modulus.clone());
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_consistency_with_u32_version() {
        // Same inputs should give same results
        let base = 5u32;
        let exp = 13u32;
        let modulus = 23u32;
        
        let result_u32 = mod_exponentiation_u32(base, exp, modulus);
        let result_bigint = mod_exponentiation_num_bigint(
            BigUint::from(base),
            BigUint::from(exp),
            BigUint::from(modulus)
        );
        
        assert_eq!(result_bigint, BigUint::from(result_u32));
    }

    #[test]
    fn test_rsa_like_scenario() {
        // Simulating RSA-like small example: encrypt then decrypt
        // Choose p=61, q=53, n=p*q=3233
        // φ(n) = (p-1)(q-1) = 3120
        // e=17 (public exponent), d=2753 (private exponent, e*d ≡ 1 mod φ(n))
        
        let message = BigUint::from(123u32);
        let e = BigUint::from(17u32);
        let d = BigUint::from(2753u32);
        let n = BigUint::from(3233u32);
        
        // Encrypt: c = m^e mod n
        let ciphertext = mod_exponentiation_num_bigint(message.clone(), e, n.clone());
        
        // Decrypt: m = c^d mod n
        let decrypted = mod_exponentiation_num_bigint(ciphertext, d, n);
        
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_rsa_like_scenario_large() {
        // More realistic RSA with larger primes
        // p and q are 20-digit primes
        let p = BigUint::parse_bytes(b"98765432109876543211", 10).unwrap();
        let q = BigUint::parse_bytes(b"12345678901234567891", 10).unwrap();
        let n = &p * &q;
        
        let message = BigUint::parse_bytes(b"424242424242424242", 10).unwrap();
        let e = BigUint::from(65537u32); // Common RSA public exponent
        
        // For this test, we just verify encryption works (not full RSA key generation)
        let ciphertext = mod_exponentiation_num_bigint(message.clone(), e.clone(), n.clone());
        let expected = message.modpow(&e, &n);
        
        assert_eq!(ciphertext, expected);
        assert!(ciphertext < n);
    }

    #[test]
    fn test_verify_against_builtin_modpow_random_cases() {
        // Verify implementation matches built-in modpow for various cases
        let test_cases = vec![
            ("123", "456", "789"),
            ("999999999", "888888888", "777777777"),
            ("18446744073709551617", "100", "18446744073709551629"), // Beyond u64::MAX
            ("2", "1000", "1000000000000000000000000000057"), // Large prime modulus
            ("123456789012345678901234567890", "9876543210", "999999999999999999"),
            ("7", "123456789012345", "999999999989"),
        ];
        
        for (base_str, exp_str, mod_str) in test_cases {
            let base = BigUint::parse_bytes(base_str.as_bytes(), 10).unwrap();
            let exp = BigUint::parse_bytes(exp_str.as_bytes(), 10).unwrap();
            let modulus = BigUint::parse_bytes(mod_str.as_bytes(), 10).unwrap();
            
            let result = mod_exponentiation_num_bigint(base.clone(), exp.clone(), modulus.clone());
            let expected = base.modpow(&exp, &modulus);
            
            assert_eq!(result, expected, 
                "Failed for base={}, exp={}, mod={}", base_str, exp_str, mod_str);
        }
    }

    #[test]
    fn test_large_base_small_exponent() {
        let base = BigUint::parse_bytes(b"999999999999999999999999999999999999999", 10).unwrap();
        let exp = BigUint::from(3u32);
        let modulus = BigUint::parse_bytes(b"1000000000000000000000000000000000000000", 10).unwrap();
        
        let result = mod_exponentiation_num_bigint(base.clone(), exp.clone(), modulus.clone());
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_small_base_large_exponent() {
        let base = BigUint::from(2u32);
        let exp = BigUint::parse_bytes(b"999999999999999999999999999999", 10).unwrap();
        let modulus = BigUint::parse_bytes(b"1000000007", 10).unwrap();
        
        let result = mod_exponentiation_num_bigint(base.clone(), exp.clone(), modulus.clone());
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mersenne_prime_like() {
        // Testing with Mersenne prime-like numbers (2^n - 1)
        let base = BigUint::from(3u32);
        let exp = BigUint::from(10000u32);
        let modulus = BigUint::parse_bytes(b"2305843009213693951", 10).unwrap(); // 2^61 - 1
        
        let result = mod_exponentiation_num_bigint(base.clone(), exp.clone(), modulus.clone());
        let expected = base.modpow(&exp, &modulus);
        
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Modulus cannot be zero")]
    fn test_panic_on_zero_modulus() {
        mod_exponentiation_num_bigint(
            BigUint::from(5u32),
            BigUint::from(3u32),
            BigUint::ZERO
        );
    }
}
