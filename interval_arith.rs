use std::ops;

/// A bounded integer.
///
/// This has two value parameter, respectively representing an upper and a lower bound.
pub struct BoundedInt<const lower: usize, const upper: usize>
    // Compile time constraints.
    where lower <= upper {
    /// The inner runtime value.
    n: usize,
}

// To see how this holds the `where` clause above, see the section on `identities`.
impl<const n: usize> BoundedInt<n, n> {
    fn new() -> Self {
        BoundedInt {
            n: n,
        }
    }
}

/// Addition of two `BoundedInt` will simply add their bounds.
///
/// We check for overflow making it statically overflow-free calculations.
impl<const upper_a: usize,
     const lower_a: usize,
     const upper_b: usize,
     const lower_b: usize> ops::Add<BoundedInt<lower_b, upper_b>> for BoundedInt<lower_a, upper_a>
     // We have to satisfy the constraints set out in the struct definition.
     where lower_a <= upper_a,
           lower_b <= upper_b,
           // Check for overflow by some `const fn`.
           is_overflow_safe(upper_a, upper_b) {
    // These parameters are constant expression.
    type Output = BoundedInt<lower_a + lower_b, upper_a + upper_b>;

    fn add(self, rhs: BoundedInt<lower_b, upper_b>) -> Self::Output {
        BoundedInt {
            n: self.n + rhs.n,
        }
    }
}

/// Multiplication of two `BoundedInt` will simply multiply their bounds.
///
/// We check for overflow making it statically overflow-free calculations.
impl<const upper_a: usize,
     const lower_a: usize,
     const upper_b: usize,
     const lower_b: usize> ops::Mul<BoundedInt<lower_b, upper_b>> for BoundedInt<lower_a, upper_a>
     // We have to satisfy the constraints set out in the struct definition.
     where lower_a <= upper_a,
           lower_b <= upper_b,
           // Check for overflow by some `const fn`.
           is_overflow_safe_mul(upper_a, upper_b) {
    // These parameters are constant expression.
    type Output = BoundedInt<lower_a * lower_b, upper_a * upper_b>;

    fn mul(self, rhs: BoundedInt<lower_b, upper_b>) -> Self::Output {
        BoundedInt {
            n: self.n * rhs.n,
        }
    }
}

impl<const upper_a: usize,
     const lower_a: usize,
     const upper_b: usize,
     const lower_b: usize> From<BoundedInt<lower_b, upper_b>> for BoundedInt<lower_a, upper_a>
     where lower_a <= upper_a,
           lower_b <= upper_b,
           // We will only extend the bound, never shrink it without runtime
           // checks, thus we add this clause:
           lower_b <= lower_a && upper_b >= upper_a {
    fn from(from: BoundedInt<lower_b, upper_b>) -> Self {
        BoundedInt {
            n: from.n,
        }
    }
}
