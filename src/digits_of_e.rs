use crate::number::Number;

/// Compute this many digits of e (including the 2 in front).
const NUM_DIGITS: u32 = 1000000;

/// Compute sum of 1/n! for 0 <= n < NUM_TERMS.
/// 1/205030! ~= 2.6e-1000042
const NUM_TERMS: u32 = 205030;

pub(crate) fn calculate<T: Number>() -> String {
    // 1/1! + ... + 1/(NUM_TERMS-1)!
    let (p, q) = sum_terms::<T>(0, NUM_TERMS - 1);
    // Add 1/0! = 1.
    let p = p + &q;
    // e ~= p/q.
    // Calculate p/q * 10^(NUM_DIGITS-1) to get the answer as an integer.
    let answer_int = p * T::from(10u32).pow(NUM_DIGITS - 1) / q;
    let mut answer = answer_int.to_string();
    // Insert the decimal period.
    answer.insert(1, '.');
    answer
}

/// a! * (1/(a+1)! + 1/(a+2)! + ... + 1/b!) as a fraction p / q.
/// q = (a+1) * (a+2) * ... * (b-1) * b
/// p = (a+2)...b + (a+3)...b + ... + 1
fn sum_terms<T: Number>(a: u32, b: u32) -> (T, T) {
    if b == a + 1 {
        (1u32.into(), b.into())
    } else {
        let mid = (a + b) / 2;
        let (p_left, q_left) = sum_terms::<T>(a, mid);
        let (p_right, q_right) = sum_terms::<T>(mid, b);
        // p / q = p_left / q_left + a!/mid! * p_right / q_right
        // a! / mid! = 1 / q_left
        // p / q = (p_left * q_right + p_right) / (q_left * q_right)
        (p_left * &q_right + p_right, q_left * q_right)
    }
}
