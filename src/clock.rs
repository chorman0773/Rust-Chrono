
use crate::{ratio::Period, duration::{Duration,IsDuration}, time_point::TimePoint};
use std::ops::*;


pub trait Clock{
    type Period: Period;
    type Repr: Sized;

    fn now() -> TimePoint<Self,Duration<Self::Repr,Self::Period>>;
}

/// Represents a Trivial Clock, that is, a clock with a Repr type which is Add, Sub, Div, Mul,
///  Rem, PartialEq, PartialOrd, and Copy. 
/// Additionally, the now function, of the implementation is known not to panic. 
///
/// Safety: A type which implements this trait MUST NOT panic in the now method. 
/// Consumers of this trait may assume that the method is incapable of panicking
pub unsafe trait TrivialClock: Clock<Repr: Add + Sub + Div + Mul + Rem + PartialEq + PartialOrd + Copy>{}

/// Represents a Clock that is monotonic or steady. A Monotonic Clock is a Clock which does not
///  reset in value, and which advances at a constant rate. 
/// 
/// Safety: A type which implements this trait must ensure that, given t1 = Self::now(); t2 = Self::now();,
/// Given that the Self::now() call initializing t1 *happens-before* the Self::now() call initializing t2,
/// t1 <= t2 is always true, and there is a consistent distance between ticks. 
pub unsafe trait Monotonic: Clock<Repr: PartialOrd>{}

pub trait ClockTimeConversion<Other: Clock> : Clock{
    fn into_other<D: IsDuration>(value: TimePoint<Self,D>) -> TimePoint<Other,D>;
}

pub struct SystemClock;


