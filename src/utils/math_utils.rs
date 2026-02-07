//Some of the utilities here are already implemented in external crates; but doing them here for learning purposes
use num_bigint::BigUint;
use num_integer::Integer;


// Custom Implementations

// This custom implementation was just to help me write the one for num_bigint: has no usage otherwise
#[allow(dead_code)]
fn custom_mod_exponentiation_u32(base: u32, exponent: u32, modulus: u32) -> u32 {
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
    result
}

#[allow(dead_code)]
fn custom_mod_exponentiation_num_bigint(base: &BigUint, exponent: &BigUint, modulus: &BigUint)-> BigUint{
    let zero = BigUint::ZERO;// cacheing to optimise
    
    assert!(*modulus != zero, "Modulus cannot be zero");
    if *modulus == BigUint::from(1u32) {return zero;}
    if *exponent == zero {return BigUint::from(1u32);}
    if *base == zero {return zero;}
    
    let mut result = BigUint::from(1u32);
    let mut exp_bin : BigUint = exponent.clone();
    let mut base_curr: BigUint = base % modulus;

    while exp_bin > zero{//the cache of zero will be useful for this
        if exp_bin.is_odd() {
            result = (&result * &base_curr) % modulus;
        }
        exp_bin >>=1u32;
        base_curr = (&base_curr * &base_curr)% modulus;
    }
    result
}

// Actual Functions
pub fn mod_exponentiation_num_bigint(base: &BigUint, exponent: &BigUint, modulus: &BigUint)-> BigUint{
    base.modpow(exponent,modulus)
    // custom_mod_exponentiation_num_bigint(base, exponent, modulus)
}

pub fn gcd(a: &BigUint, b: &BigUint)-> BigUint{
    a.gcd(b)
}

