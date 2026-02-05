//Some of the utilities here are already implemented in external crates; but doing them here for learning purposes
use num_bigint::BigUint;
use num_integer::Integer;
// use super::*;

// pub fn mod_exponentiation(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
//     // Implement modular exponentiation (base^exponent mod modulus)
//     // Placeholder implementation
//     BigUint::from(0u32)
// }

pub fn mod_exponentiation_u32(base: u32, exponent: u32, modulus: u32) -> u32 {
    assert!(modulus!=0,"Modulus cannot be zero");
    if modulus == 1 {return 0;}
    if exponent == 0 {return 1;}// 1%modulus which is always 1 because modulus !=1
    if base ==0 {return 0;} 
    let mut result: u32=1;
    let mut exp_bin = exponent;
    let mut base_curr = base % modulus;
    while exp_bin>0 {
        if (exp_bin%2)==1 {
            result = ((result as u64 * base_curr as u64) % modulus as u64) as u32;
        }
        exp_bin>>=1;
        base_curr = ((base_curr as u64 * base_curr as u64) % modulus as u64) as u32;
    }
    return result;
}

pub fn mod_exponentiation_num_bigint(base: BigUint, exponent: BigUint, modulus: BigUint)-> BigUint{
    assert!(modulus != BigUint::ZERO, "Modulus cannot be zero");
    if modulus == BigUint::from(1u32) {return BigUint::ZERO;}
    if exponent == BigUint::ZERO {return BigUint::from(1u32);}
    if base == BigUint::ZERO {return BigUint::ZERO;}
    
    let mut result = BigUint::from(1u32);
    let mut exp_bin : BigUint = exponent;
    let mut base_curr: BigUint = base % &modulus;
    while exp_bin > BigUint::ZERO{
        if exp_bin.is_odd() {
            result = (&result * &base_curr) % &modulus;
        }
        exp_bin >>=1u32;
        base_curr = (&base_curr * &base_curr)% &modulus;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mod_exp_u32(){
        let result = mod_exponentiation_u32(5,13,23);
        assert_eq!(result,21);
            // 2^10 mod 1000 = 24
        assert_eq!(mod_exponentiation_u32(2, 10, 1000), 24);
        
        // 3^0 mod 7 = 1
        assert_eq!(mod_exponentiation_u32(3, 0, 7), 1);

    }
    
}
// pub fn main(){
//     print!("{}\n",mod_exponentiation_u32(5,13,23)) // gives wrong answer
// }