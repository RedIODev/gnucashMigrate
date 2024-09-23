use std::{fmt::{format, Debug, Display}, iter::Sum, ops::{Add, AddAssign, Deref, Div, Mul, Neg}, str::FromStr};

use crate::common::Numerator;

pub trait FromString: FromStr {
    fn from_string(value: &String) -> Result<Self, <Self as FromStr>::Err>;
}

impl<T> FromString for T 
where T: FromStr {
    fn from_string(value: &String) -> Result<Self, <Self as FromStr>::Err> {
        T::from_str(&value)
    }
}

pub trait Update<T> {
    fn update<F>(&mut self, updater:F)
    where F: FnMut(&mut T);
}

impl<T> Update<T> for Option<T> {
    fn update<F>(&mut self, mut updater:F)
    where F: FnMut(&mut T) {
        if let Some(value) = self {
            updater(value)
        }
    }
}

#[derive(Debug, Clone)]
pub enum FixedError<NE, DE = NE> {
    MalformedSource(String),
    ParseNumeratorError(NE),
    ParseDenominatorError(DE),
}

impl<NE,DE> FixedError<NE,DE> {
    fn from_ne(value: NE) -> Self {
        Self::ParseNumeratorError(value)
    }

    fn from_de(value: DE) -> Self {
        Self::ParseDenominatorError(value)
    }
}

impl<E> Display for FixedError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl<E:Debug> std::error::Error for FixedError<E> {}

#[derive(Debug, Clone, Copy)]
pub struct Fixed<Numerator, Denominator>(pub Numerator, pub Denominator);

impl<N,D> Fixed<N,D> 
where
    N: Mul<Output = N>,
    D: PartialEq + PartialOrd + Clone + Div<D, Output=D> + Into<N> {
    fn equalize_denominator(a:Self, b:Self) -> (Self, Self) {
        if a.1 == b.1 {
            return (a,b);
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

impl<N,D> Fixed<N,D> 
where 
    N:Default + PartialEq {
    pub fn is_zero(&self) -> bool {
        self.0 == N::default()
    }
}

impl<N,D> Display for Fixed<N,D> 
where 
    N: Display,
    D: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl<N,D> Default for Fixed<N,D> 
where 
    N: Default,
    D: From<u8> {
    fn default() -> Self {
        Self(N::default(), 1.into())
    }
}

impl<N,D> PartialEq for Fixed<N,D> 
where 
    N: Mul<Output = N> + Clone + PartialEq,
    D: PartialEq + PartialOrd + Clone + Div<Output = D> + Into<N> {
    fn eq(&self, other: &Self) -> bool {
        let (a, b) = Self::equalize_denominator(self.clone(), other.clone());
        a.0 == b.0
    }
}

impl<N,D> Eq for Fixed<N,D> 
where 
    Self: PartialEq {}

impl<N,D> Add for Fixed<N,D> 
where
    N: Mul<Output = N> + Clone + Add<Output = N>,
    D: PartialEq + PartialOrd + Clone + Div<Output = D> + Into<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (a,b) = Self::equalize_denominator(self, rhs);
        Fixed(a.0+b.0, a.1)
    }
}

impl<N,D> AddAssign for Fixed<N,D>
where
    Self: Clone + Add<Output = Self> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl<N,D> Sum for Fixed<N,D> 
where 
    Self: Default + AddAssign {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Fixed::default();
        for i in iter {
            sum += i;
        }
        sum
    }
}

impl<N,D> Neg for Fixed<N,D> 
where 
    N: Neg<Output = N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fixed(-self.0, self.1)
    }
}

impl<N,D,NE,DE> FromStr for Fixed<N,D> 
where 
    N: FromStr<Err=NE>,
    D: FromStr<Err=DE> {
    type Err = FixedError<NE,DE>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((num, den)) = s.split_once('/') else {
            return  Err(FixedError::MalformedSource(s.to_string()));
        };
        Ok(Fixed(num.parse().map_err(FixedError::from_ne)?, den.parse().map_err(FixedError::from_de)?))
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

pub trait Single<T> {
    fn single(self) -> Option<T>; 
}

impl<I, T> Single<T> for I 
where I: Iterator<Item =T> {
    fn single(mut self) -> Option<T> {
        let Some(single) = self.next() else {
            return None;
        };
        if self.next().is_none(){
            Some(single)
        } else {
            None
        }
    }
}