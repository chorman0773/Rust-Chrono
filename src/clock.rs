
use clocks::HackForSoundness;

use crate::prelude::*;
use std::{num::Wrapping, ops::*};


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
/// and that the Self::now() call initializing t1 *happens-before* the Self::now() call initializing t2,
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

///
/// A TrivialClock, which yields system time points, relative to the unix epoch.
/// 
pub enum SystemClock{}

#[cfg(unix)]
mod clocks{
        use std::ops::{Mul, Div, Add};

    use libc::clock_gettime;
    use libc::CLOCK_REALTIME;

    use libc::CLOCK_MONOTONIC;

    use libc::timespec;

    use crate::prelude::*;

    pub fn get_system_time<_Period: Period,Repr>() -> Repr
    where Repr: Mul<Output=Repr> + Div<Output=Repr> + Add<Output=Repr>, i64: Into<Repr>
    {
 
        let mut ts: timespec = unsafe{core::mem::zeroed()};
        unsafe{clock_gettime(CLOCK_REALTIME, &mut ts)};
        let val = ((Into::<Repr>::into(ts.tv_sec)*_Period::NUMERATOR.into())/_Period::DENOMINATOR.into())
            + ((Into::<Repr>::into(ts.tv_nsec)*RatioMultiply::<_Period,Nano>::NUMERATOR.into())/RatioMultiply::<_Period,Nano>::DENOMINATOR.into());
        

        val
    }

    pub fn get_steady_time<_Period: Period,Repr>() -> Repr
    where Repr: Mul<Output=Repr> + Div<Output=Repr> + Add<Output=Repr>, i64: Into<Repr>
    {
 
        let mut ts: timespec = unsafe{core::mem::zeroed()};
        unsafe{clock_gettime(CLOCK_MONOTONIC, &mut ts)};
        let val = ((Into::<Repr>::into(ts.tv_sec)*_Period::NUMERATOR.into())/_Period::DENOMINATOR.into())
            + ((Into::<Repr>::into(ts.tv_nsec)*RatioMultiply::<_Period,Nano>::NUMERATOR.into())/RatioMultiply::<_Period,Nano>::DENOMINATOR.into());
        

        val
    }

    // Hack for soundness of impl TrivialClock for {System,Steady}Clock. 
    #[repr(transparent)]
    pub struct HackForSoundness(pub i64);

    impl From<i64> for HackForSoundness{
        #[inline(always)]
        fn from(v: i64) -> Self {
            Self(v)
        }   
    }

    impl From<HackForSoundness> for i64{
        #[inline(always)]
        fn from(v: HackForSoundness) -> Self {
            v.0
        }
    }

    impl Add for HackForSoundness{
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            self.0.saturating_add(rhs.0).into()
        }
    }

    impl Mul for HackForSoundness{
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            self.0.saturating_mul(rhs.0).into()
        }
    }
    impl Div for HackForSoundness{
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            self.0.wrapping_div(rhs.0).into()
        }
    }
}




impl Clock for SystemClock{
    type Repr = <Miliseconds as IsDuration>::Repr;
    type Period = Mili;
    type Duration = Duration<Self::Repr,Self::Period>;

    fn now() -> TimePoint<Self,Self::Duration>{
        TimePoint::new(Duration::new(clocks::get_system_time::<Self::Period,HackForSoundness>().0))
    }
}

unsafe impl TrivialClock for SystemClock{}


pub enum SteadyClock{}

impl Clock for SteadyClock{
    type Repr = <Nanoseconds as IsDuration>::Repr;
    type Period = Nano;
    type Duration = Duration<Self::Repr,Self::Period>;

    fn now() -> TimePoint<Self,Self::Duration>{
        TimePoint::new(Duration::new(clocks::get_steady_time::<Self::Period,HackForSoundness>().0 as i128))
    }
}


unsafe impl TrivialClock for SteadyClock{}
unsafe impl Monotonic for SteadyClock{}