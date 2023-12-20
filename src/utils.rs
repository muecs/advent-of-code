/// utility functions

/// greatest common divisor
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Default + PartialEq,
    T: std::ops::Rem<Output = T>,
{
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

/// least common multiple
pub fn lcm<T>(vals: &[T]) -> T
where
    T: Copy + Default + PartialEq,
    T: std::ops::Div<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Rem<Output = T>,
{
    if vals.len() == 1 {
        return vals[0];
    }
    let a = vals[0];
    let b = lcm(&vals[1..]);
    a * b / gcd(a, b)
}
