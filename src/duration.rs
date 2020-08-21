
use crate::prelude::*;
use std::marker::PhantomData;

use std::ops::*;
use std::cmp::*;

use std::{hash::{Hasher, Hash}, convert::TryInto};

pub unsafe trait TreatAsFloatingPoint{}

unsafe impl TreatAsFloatingPoint for f32{}
unsafe impl TreatAsFloatingPoint for f64{}


pub struct Duration<Repr,_Period = Unit>{
    value: Repr,
    _phantom: PhantomData<_Period>
}

impl<Repr, _Period: Period> Duration<Repr,_Period>{
    pub fn new(x: Repr) -> Self{
        Self{value: x,_phantom: PhantomData}
    }

    pub fn into_inner(self) -> Repr{
        self.value
    }

}


impl<R1: PartialEq<R2>,R2,_P> PartialEq<Duration<R2,_P>> for Duration<R1,_P>{
    fn eq(&self,rhs: &Duration<R2,_P>) -> bool{
        self.value.eq(&rhs.value)
    }
}

impl<Repr: Eq,_P> Eq for Duration<Repr,_P>{}

impl<R1: PartialOrd<R2>,R2,_P> PartialOrd<Duration<R2,_P>> for Duration<R1,_P>{
    fn partial_cmp(&self, other: &Duration<R2,_P>) -> Option<Ordering>{
        self.value.partial_cmp(&other.value)
    }
}

impl<R1: Ord,_P> Ord for Duration<R1,_P>{
    fn cmp(&self,other: &Self) -> Ordering{
        self.value.cmp(&other.value)
    }
}

impl<R1: Hash,_P> Hash for Duration<R1,_P>{
    fn hash<H: Hasher>(&self, state: &mut H){
        self.value.hash(state)
    }
}

impl<R1: Add<R2>,R2,_P> Add<Duration<R2,_P>> for Duration<R1,_P>{
    type Output = Duration<<R1 as Add<R2>>::Output,_P>;

    fn add(self,rhs: Duration<R2,_P>) -> Self::Output{
        Duration{
            value: self.value + rhs.value,
            _phantom: PhantomData
        }
    }
}

impl<R1: Sub<R2>,R2,_P> Sub<Duration<R2,_P>> for Duration<R1,_P>{
    type Output = Duration<<R1 as Sub<R2>>::Output,_P>;

    fn sub(self,rhs: Duration<R2,_P>) -> Self::Output{
        Duration{
            value: self.value - rhs.value,
            _phantom: PhantomData
        }
    }
}

impl<R1: Mul<R2>,R2,_P> Mul<R2> for Duration<R1,_P>{
    type Output = Duration<<R1 as Mul<R2>>::Output,_P>;

    fn mul(self,rhs: R2) -> Self::Output{
        Duration{
            value: self.value * rhs,
            _phantom: PhantomData
        }
    }
}

impl<R1: Div<R2>,R2,_P> Div<R2> for Duration<R1,_P>{
    type Output = Duration<<R1 as Div<R2>>::Output,_P>;

    fn div(self,rhs: R2) -> Self::Output{
        Duration{
            value: self.value / rhs,
            _phantom: PhantomData
        }
    }
}

impl<R1: Rem<R2>,R2,_P> Rem<R2> for Duration<R1,_P>{
    type Output = Duration<<R1 as Rem<R2>>::Output,_P>;

    fn rem(self,rhs: R2) -> Self::Output{
        Duration{
            value: self.value % rhs,
            _phantom: PhantomData
        }
    }
}

impl<R1: Clone,_P> Clone for Duration<R1,_P>{
    fn clone(&self) -> Self{
        Self{
            value: self.value.clone(),
            _phantom: PhantomData
        }
    }
}

impl<R1: Copy,_P> Copy for Duration<R1,_P>{}

impl<R1: Default,_P> Default for Duration<R1,_P>{
    fn default() -> Self{
        Self{
            value: Default::default(),
            _phantom: PhantomData
        }
    }
}

#[doc(hidden)]
mod sealed{
    pub trait Sealed{}
}

impl<R,_P> sealed::Sealed for Duration<R,_P>{}

pub trait IsDuration : sealed::Sealed{
    type Repr: Sized;
    type Period: Period;
    fn as_duration(self) -> Duration<Self::Repr,Self::Period>;
    
    
}

pub trait DurationInto<R2>: IsDuration{
    fn into(self) -> Duration<R2,Self::Period>;
}

pub trait DurationTryInto<R2>: IsDuration{
    type Error;

    fn try_into(self) -> Result<Duration<R2,Self::Period>,Self::Error>;
}

impl<Repr,_Period: Period> IsDuration for Duration<Repr,_Period>{
    type Repr = Repr;
    type Period = _Period;


    fn as_duration(self) -> Duration<Repr,_Period>{
        self
    }
}

impl<R1: Into<R2>,R2,P: Period> DurationInto<R2> for Duration<R1,P>{
    fn into(self) -> Duration<R2,P>{
        Duration{
            value: Into::into(self.value),
            _phantom: PhantomData
        }
    }
}

impl<R1: TryInto<R2>,R2,P: Period> DurationTryInto<R2> for Duration<R1,P>{
    type Error = R1::Error;
    fn try_into(self) -> Result<Duration<R2,P>,Self::Error>{
        self.value.try_into().map(|value|
        Duration{
            value,
            _phantom: PhantomData
        })
    }
}

pub trait DurationCast<D: IsDuration> : IsDuration{
    type Error;
    fn duration_cast(self) -> Result<D,<Self as DurationCast<D>>::Error>;
}

impl<R1,P1: Period,R2,P2: Period> DurationCast<Duration<R2,P2>> for Duration<R1,P1>
    where R1: Mul<i64>, <R1 as Mul<i64>>::Output: Div<i64,Output=R1::Output>, 
    R1::Output: TryInto<R2>{
        type Error = <Duration<<R1 as Mul<i64>>::Output,P2> as DurationTryInto<R2>>::Error;

        fn duration_cast(self) -> Result<Duration<R2,P2>,<Self as DurationCast<Duration<R2,P2>>>::Error>{
            let (num,denom) = (RatioDivide::<P1,P2>::NUMERATOR,RatioDivide::<P1,P2>::DENOMINATOR);
            let value = (self.value.mul(num)).div(denom);
            DurationTryInto::<R2>::try_into(Duration{
                value,
                _phantom: PhantomData::<P2>
            })
        }
    }


pub type Years = Duration<i64,Ratio<31556952,1>>;
pub type Months = Duration<i64,Ratio<2629746,1>>;
pub type Weeks = Duration<i64,Ratio<604800,1>>;
pub type Days = Duration<i64,Ratio<86400,1>>;
pub type Hours = Duration<i64,Ratio<3600,1>>;
pub type Minutes = Duration<i64,Ratio<60,1>>;
pub type Seconds = Duration<i64,Unit>;
pub type Miliseconds = Duration<i64,Mili>;
pub type Microseconds = Duration<i64,Micro>;
pub type Nanoseconds = Duration<i128,Nano>;

