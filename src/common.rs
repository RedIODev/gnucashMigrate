use std::{fmt::Display, iter::Sum, ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub, SubAssign}};

pub trait Denominator: PartialEq + Default + Div<Output = Self> + PartialOrd + Clone {}
pub trait Numerator<D: Denominator>: Default + Mul<Output = Self> + From<D> {}

impl<T> Denominator for T where T: PartialEq + Default + Div<Output = Self> + PartialOrd + Clone {}
impl<T,D> Numerator<D> for T where T: Default + Mul<Output = Self> + From<D>, D:Denominator {}

#[derive(Debug, Clone, Copy)]
pub struct Fixed<N, D=u16>(pub N, pub D) where N: Numerator<D>, D: Denominator;

impl<N, D> Fixed<N, D>
where N: Numerator<D>, D: Denominator {
    fn equalize_fractions(a:Self, b:Self) -> (Self, Self) {
        if a.1 == b.1 {
            return  (a,b);
        }
        if a.1 > b.1 {
            let factor = a.1.clone() / b.1.clone();
            let a1 = a.1.clone();
            (a, Fixed(b.0*factor.into(), a1))
        } else {
            let factor = b.1.clone() / a.1.clone();
            let b1 = b.1.clone();
            (Fixed(a.0*factor.into(), b1), b)
        }
    }
}

impl<N, D> Fixed<N, D> 
where  N: Numerator<D> + PartialEq, D: Denominator {
    pub fn is_zero(&self) -> bool {
        self.0 == N::default()
    }
}

impl<N,D> Fixed<N, D>
where N: Numerator<D> + Display, D: Denominator + Display {
    pub fn to_string_raw(&self) -> String {
        format!("{}/{}", self.0, self.1)
    }
}

impl<N, D> Display for Fixed<N,D>
where N: Numerator<D> + Clone, D: Denominator, f64: From<N> + From<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", f64::from(self.0.clone()) / f64::from(self.1.clone()))
    }
}

impl<N, D> Default for Fixed<N, D>
where N: Numerator<D>, D: Denominator + From<u8> {
    fn default() -> Self {
        Self(N::default(), 1.into())
    }
}

impl <N, D> PartialEq for Fixed<N, D>
where N: Numerator<D> + PartialEq + Clone, D:Denominator {
    fn eq(&self, other: &Self) -> bool {
        let (a, b) = Self::equalize_fractions(self.clone(), other.clone());
        a.0 == b.0
    }
}

impl<N, D> Add for Fixed<N, D> 
where N: Numerator<D> + Add<Output = N>, D: Denominator {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (a,b) = Self::equalize_fractions(self, rhs);
        Fixed(a.0+b.0, a.1)
    }
}

impl<N, D> AddAssign for Fixed<N, D>
where N: Numerator<D> + Add<Output = N> + Clone, D: Denominator {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl<N, D> Sum for Fixed<N, D>
where N: Numerator<D> + Add<Output = N> + Clone, D: Denominator + From<u8> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Fixed::default();
        for i in iter {
            sum += i;
        }
        sum
    }
}

impl<N, D> Sub for Fixed<N, D>
where N: Numerator<D> + Sub<Output = N>, D:Denominator {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (a,b) = Self::equalize_fractions(self, rhs);
        Fixed(a.0-b.0, a.1)
    }
}

impl<N, D> SubAssign for Fixed<N, D>
where N: Numerator<D> + Sub<Output = N> + Clone, D: Denominator {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs
    }
}

impl<N,D> Neg for Fixed<N,D>
where N: Numerator<D> + Neg<Output = N>, D: Denominator {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fixed(-self.0, self.1)
    }
}




#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumExtender<T>(pub T);
impl<T: Sum> Default for SumExtender<T> {
    fn default() -> Self {
        Self(std::iter::empty().sum())
    }
}
impl<T, E> Extend<E> for SumExtender<T>
where
    T: std::ops::AddAssign<E>,
{
    fn extend<I: IntoIterator<Item = E>>(&mut self, iter: I) {
        for it in iter {
            self.0 += it;
        }
    }
}

impl<T: Sum> Deref for SumExtender<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}