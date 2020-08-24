
use crate::prelude::*;
use std::ops::*;


pub trait Clock: Sized{
    type Period: Period;
    type Repr: Sized;
    type Duration: Sized + IsDuration<Repr=Self::Repr,Period=Self::Period>;

    fn now() -> TimePoint<Self,Self::Duration>;
}

/// Represents a Trivial Clock, that is, a clock with a Repr type which is Add, Sub, Div, Mul,
///  Rem, PartialEq, PartialOrd, and Copy, and with a Duration Type which satisfies the above
/// Additionally, the now function, of the implementation is known not to panic. 
///
/// Safety: A type which implements this trait MUST NOT panic in the now method. 
/// Consumers of this trait may assume that the method is incapable of panicking
pub unsafe trait TrivialClock: Clock<Repr: Add + Sub + Div + Mul + PartialEq + PartialOrd + Copy>{}

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

impl<_Clock: Clock> ClockTimeConversion<_Clock> for _Clock{
    fn into_other<D: IsDuration>(value: TimePoint<Self,D>) -> TimePoint<Self,D>{
        value
    }
}

pub struct SystemClock;

use libc::time_t;
use libc::time;

impl Clock for SystemClock{
    type Repr = time_t;
    type Period = Unit;
    type Duration = Duration<Self::Repr,Self::Period>;

    fn now() -> TimePoint<Self,Self::Duration>{
        TimePoint::new(Duration::new(unsafe{time(std::ptr::null_mut())}))
    }
}

unsafe impl TrivialClock for SystemClock{}



