use std::fmt;

#[derive(Clone)]
pub struct Fraction {
    numerator: u128,
    denominator: u128,
    is_negative: bool,
}

impl From<f64> for Fraction {
    fn from(value: f64) -> Fraction {
        Fraction::new_zero()
    }
}

impl From<i128> for Fraction {
    fn from(value: i128) -> Fraction {
        Fraction::new(value.abs() as u128, 1, value < 0)
    }
}

impl From<&str> for Fraction {
    fn from(value: &str) -> Fraction {
        Fraction::new_zero()
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_negative {
            write!(f, "-{}/{}", self.numerator, self.denominator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl Fraction {
    pub fn new(numerator: u128, denominator: u128, is_negative: bool) -> Fraction {
        Fraction {
            numerator,
            denominator,
            is_negative,
        }
    }

    pub fn new_zero() -> Fraction {
        Fraction {
            numerator: 0,
            denominator: 1,
            is_negative: false,
        }
    }

    pub fn new_natural(value: u128) -> Fraction {
        Fraction {
            numerator: value,
            denominator: 1,
            is_negative: false,
        }
    }

    pub fn new_nan() -> Fraction {
        Fraction::new(0, 0, false)
    }

    pub fn numerator(&self) -> u128 {
        self.numerator
    }

    pub fn denominator(&self) -> u128 {
        self.denominator
    }

    pub fn is_negative(&self) -> bool {
        self.is_negative
    }

    pub fn is_positive(&self) -> bool {
        !self.is_negative
    }

    pub fn abs(&self) -> Fraction {
        Fraction::new(self.numerator, self.denominator, false)
    }

    pub fn neg(&self) -> Fraction {
        Fraction::new(self.numerator, self.denominator, !self.is_negative)
    }

    pub fn add(&self, other: &Fraction) -> Fraction {
        if let Some(result) = self.process_rare_numbers_for_add(other) {
            return result;
        }

        let (unified_self, unified_other) = self.unify(other);

        if unified_self.is_negative == unified_other.is_negative {
            return Fraction::new(
                unified_self.numerator + unified_other.numerator,
                unified_self.denominator,
                unified_self.is_negative,
            )
            .simplify();
        }

        if unified_self.is_abs_equal(&unified_other) {
            return Fraction::new_zero();
        }

        if unified_self.is_abs_bigger(&unified_other) {
            Fraction::new(
                unified_self.numerator - unified_other.numerator,
                unified_self.denominator,
                unified_self.is_negative,
            )
            .simplify()
        } else {
            Fraction::new(
                unified_other.numerator - unified_self.numerator,
                unified_self.denominator,
                unified_other.is_negative,
            )
            .simplify()
        }
    }

    #[inline]
    fn process_rare_numbers_for_add(&self, other: &Fraction) -> Option<Fraction> {
        if other.is_zero() {
            return Some(self.clone());
        }
        if self.is_zero() {
            return Some(other.clone());
        }

        if self.is_nan() || other.is_nan() {
            return Some(Fraction::new_nan());
        }

        if self.is_negative_infinity() && other.is_positive_infinity() {
            return Some(Fraction::new_nan());
        }
        if self.is_positive_infinity() && other.is_negative_infinity() {
            return Some(Fraction::new_nan());
        }

        if self.is_infinity() || other.is_infinity() {
            return Some(self.clone());
        }

        None
    }

    pub fn sub(&self, other: &Fraction) -> Fraction {
        self.add(&other.neg())
    }

    pub fn mul(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
            self.is_negative ^ other.is_negative,
        )
        .simplify()
    }

    pub fn div(&self, other: &Fraction) -> Fraction {
        self.mul(&other.reverse())
    }

    pub fn reverse(&self) -> Fraction {
        Fraction::new(self.denominator, self.numerator, self.is_negative)
    }

    pub fn is_natural(&self) -> bool {
        !self.is_negative && self.numerator != 0 && self.denominator == 1
    }

    pub fn is_nan(&self) -> bool {
        self.numerator == 0 && self.denominator == 0
    }

    pub fn is_negative_infinity(&self) -> bool {
        self.is_negative && self.is_infinity()
    }

    pub fn is_positive_infinity(&self) -> bool {
        !self.is_negative && self.is_infinity()
    }

    pub fn is_infinity(&self) -> bool {
        self.numerator != 0 && self.denominator == 0
    }

    pub fn is_zero(&self) -> bool {
        self.numerator == 0 && self.denominator != 0
    }

    #[inline]
    fn mul_with_number(&self, mut number: i128) -> Fraction {
        let mut is_negative = false;
        if number < 0 {
            number *= -1;
            is_negative = true;
        }

        Fraction::new(
            self.numerator * (number as u128),
            self.denominator * (number as u128),
            self.is_negative ^ is_negative,
        )
    }

    #[inline]
    fn simplify(&self) -> Fraction {
        if self.is_infinity() || self.is_zero() || self.is_nan() {
            return self.clone();
        }

        if self.numerator == 1 || self.denominator == 1 {
            return self.clone();
        }

        let gcd = self.find_gcd(self.numerator, self.denominator);
        Fraction::new(
            self.numerator / gcd,
            self.denominator / gcd,
            self.is_negative,
        )
    }

    #[inline]
    fn find_gcd(&self, mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            let c = b;
            b = a % b;
            a = c;
        }

        a
    }

    #[inline]
    fn is_abs_equal(&self, other: &Fraction) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }

    #[inline]
    fn is_abs_bigger(&self, other: &Fraction) -> bool {
        if self.denominator == other.denominator {
            self.numerator > other.numerator
        } else {
            let (unified_self, unified_other) = self.unify(other);
            unified_self.numerator > unified_other.numerator
        }
    }

    #[inline]
    fn unify(&self, other: &Fraction) -> (Fraction, Fraction) {
        match self.denominator {
            x if x == other.denominator => (self.clone(), other.clone()),
            x if other.denominator % x == 0 => {
                let scale = (other.denominator / x) as i128;
                (self.mul_with_number(scale), other.clone())
            }
            x if x % other.denominator == 0 => {
                let scale = (x / other.denominator) as i128;
                (self.clone(), other.mul_with_number(scale))
            }
            _ => (
                self.mul_with_number(other.denominator as i128),
                other.mul_with_number(self.denominator as i128),
            ),
        }
    }
}
