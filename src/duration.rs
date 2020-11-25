
use crate::prelude::*;
use std::marker::PhantomData;

use std::ops::*;
use std::cmp::*;

use std::{hash::{Hasher, Hash}, convert::TryInto};

#[cfg(feature="step")]
use std::iter::Step;


/// Marker Trait equivalent to std::chrono::treat_as_floating_point
/// presently unused, though in the future, it will be used to assist in the implementations of the From and DurationCast
pub unsafe trait TreatAsFloatingPoint{}

unsafe impl TreatAsFloatingPoint for f32{}
unsafe impl TreatAsFloatingPoint for f64{}


/// Trait representing values of the Duration type
pub trait DurationValues{
    /// The Zero value for the type, IE. the addative identity.
    const ZERO: Self;
    /// The Minimum Value for the type
    const MIN: Self;
    /// The Maximum Value of the type
    const MAX: Self;
}

impl DurationValues for i8{
    const ZERO: Self = 0;
    const MIN: Self = -128;
    const MAX: Self = 127;
}

impl DurationValues for i16{
    const ZERO: Self = 0;
    const MIN: Self = -32768;
    const MAX: Self = 32767;
}

impl DurationValues for i32{
    const ZERO: Self = 0;
    const MIN: Self = i32::min_value();
    const MAX: Self = i32::max_value();
}

impl DurationValues for i64{
    const ZERO: Self = 0;
    const MIN: Self = i64::min_value();
    const MAX: Self = i64::max_value();
}

impl DurationValues for i128{
    const ZERO: Self = 0;
    const MIN: Self = i128::min_value();
    const MAX: Self = i128::max_value();
}

impl DurationValues for isize{
    const ZERO: Self = 0;
    const MIN: Self = isize::min_value();
    const MAX: Self = isize::max_value();
}

impl DurationValues for u8{
    const ZERO: Self = 0;
    const MIN: Self = 0;
    const MAX: Self = 255;
}

impl DurationValues for u16{
    const ZERO: Self = 0;
    const MIN: Self = 0;
    const MAX: Self = 65535;
}

impl DurationValues for u32{
    const ZERO: Self = 0;
    const MIN: Self = 0;
    const MAX: Self = u32::max_value();
}

impl DurationValues for u64{
    const ZERO: Self = 0;
    const MIN: Self = 0;
    const MAX: Self = u64::max_value();
}

impl DurationValues for u128{
    const ZERO: Self = 0;
    const MIN: Self = 0;
    const MAX: Self = u128::max_value();
}

impl DurationValues for usize{
    const ZERO: Self = 0;
    const MIN: Self = 0;
    const MAX: Self = usize::max_value();
}

impl DurationValues for f32{
    const ZERO: Self = 0.0;
    const MIN: Self = ::std::f32::MIN;
    const MAX: Self = ::std::f32::MAX;
}

impl DurationValues for f64{
    const ZERO: Self = 0.0;
    const MIN: Self = ::std::f64::MIN;
    const MAX: Self = ::std::f64::MAX;
}

/// A type which Represents a Duration, as a Repr value and a Period.
/// Duration is a repr(transparent) structure of its Repr type, it is safe to transmute between Duration<Repr>, and Repr.
/// There are no Limitations to _Period, though it is recommended that it be a specialization of crate::ratio::Ratio (and therefore implement the Period trait)
/// 
/// Note:
/// The representation of Duration<Repr,Period> is entirely agnostic of Period
#[repr(transparent)]
pub struct Duration<Repr,Period = Unit>{
    value: Repr,
    _phantom: PhantomData<*mut Period>
}
// Because the use of PhantomData<*mut Period> suppressed the auto impls. 
unsafe impl<Repr: Send,Period> Send for Duration<Repr,Period>{}
unsafe impl<Repr: Sync,Period> Sync for Duration<Repr,Period>{}

impl<Repr: DurationValues,_Period> DurationValues for Duration<Repr,_Period>{
    const ZERO: Self = Duration::new(Repr::ZERO);
    const MIN: Self = Duration::new(Repr::MIN);
    const MAX: Self = Duration::new(Repr::MAX);
}

impl<Repr, _Period> Duration<Repr,_Period>{
    /// 
    /// Constructs a new Duration from its representation. 
    pub const fn new(x: Repr) -> Self{
        Self{value: x,_phantom: PhantomData}
    }

    /// Obtains the representation of the Duration value
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

impl<R1: AddAssign<R2>,R2,_P> AddAssign<Duration<R2,_P>> for Duration<R1,_P>{
    fn add_assign(&mut self, rhs: Duration<R2,_P>) {
        self.value.add_assign(rhs.value)
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

impl<R1: SubAssign<R2>,R2,_P> SubAssign<Duration<R2,_P>> for Duration<R1,_P>{
    fn sub_assign(&mut self, rhs: Duration<R2,_P>) {
        self.value.sub_assign(rhs.value)
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

impl<R1: MulAssign<R2>,R2,_P> MulAssign<R2> for Duration<R1,_P>{
    fn mul_assign(&mut self, rhs: R2) {
        self.value.mul_assign(rhs)
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

impl<R1: DivAssign<R2>,R2,_P> DivAssign<R2> for Duration<R1,_P>{
    fn div_assign(&mut self, rhs: R2) {
        self.value.div_assign(rhs)
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

/// Trait for specializations of the Duration type. 
/// 
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
                _phantom: PhantomData
            })
        }
}


#[cfg(feature="step")]
unsafe impl<R1: Step,P1> Step for Duration<R1,P1>{
    fn steps_between(start: &Self,end: &Self) -> Option<usize>{
        Step::steps_between(&start.value,&end.value)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self>{
        Step::forward_checked(start.value,count)
            .map(|value|Duration::new(value))
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self>{
        Step::backward_checked(start.value,count)
            .map(|value|Duration::new(value))
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

