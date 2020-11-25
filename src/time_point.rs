
use crate::{duration::IsDuration, prelude::*};

use std::{marker::PhantomData, ops::*,cmp::*, hash::{Hash, Hasher}};

pub struct TimePoint<Clock,Duration>{
    value: Duration,
    _phantom: PhantomData<*mut Clock>
}

unsafe impl<Duration: Send,Clock> Send for TimePoint<Clock,Duration>{}
unsafe impl<Duration: Sync,Clock> Sync for TimePoint<Clock,Duration>{}

impl<Clock,Duration: IsDuration + DurationValues> TimePoint<Clock,Duration>{
    pub const EPOCH: Self = Self::new(Duration::ZERO);
    pub const MIN: Self = Self::new(Duration::MIN);
    pub const MAX: Self = Self::new(Duration::MAX);
}

impl<_Clock,_Duration> TimePoint<_Clock,_Duration>{
    pub const fn new(value: _Duration) -> Self{
        Self{
            value,
            _phantom: PhantomData
        }
    }

    pub fn into_inner(self) -> _Duration{
        self.value
    }
}


impl<_Clock: Clock,_Duration: IsDuration> TimePoint<_Clock,_Duration>{
    
    pub fn into<R2>(self) -> TimePoint<_Clock,Duration<R2,_Duration::Period>> where _Duration: DurationInto<R2>{
        TimePoint{
            value: self.value.into(),
            _phantom: PhantomData
        }
    }
    pub fn try_into<R2>(self) -> Result<TimePoint<_Clock,Duration<R2,_Duration::Period>>,
        <_Duration as DurationTryInto<R2>>::Error> where _Duration: DurationTryInto<R2>{
        Ok(TimePoint{
            value: self.value.try_into()?,
            _phantom: PhantomData
        })
    }
}

pub fn time_point_cast<D1: DurationCast<D2>,D2: IsDuration,_Clock: Clock>(_tp: TimePoint<_Clock,D1>) -> Result<TimePoint<_Clock,D2>,<D1 as DurationCast<D2>>::Error>{
    _tp.value.duration_cast().map(|value|
        TimePoint{
            value,
            _phantom: PhantomData
        }
    )
} 

impl<D1: Add<D2>,D2,_Clock: Clock> Add<D2> for TimePoint<_Clock,D1>{
    type Output = TimePoint<_Clock,<D1 as Add<D2>>::Output>;
    fn add(self,rhs: D2) -> Self::Output{
        TimePoint::new(self.value.add(rhs))
    }
}

impl<D1: AddAssign<D2>,D2,_Clock: Clock> AddAssign<D2> for TimePoint<_Clock,D1>{
    fn add_assign(&mut self, rhs: D2) {
        self.value.add_assign(rhs)
    }
}

impl<D1: Sub<D2>,D2,_Clock: Clock> Sub<D2> for TimePoint<_Clock,D1>{
    type Output = TimePoint<_Clock,<D1 as Sub<D2>>::Output>;
    fn sub(self,rhs: D2) -> Self::Output{
        TimePoint::new(self.value.sub(rhs))
    }
}

impl<D1: SubAssign<D2>,D2,_Clock: Clock> SubAssign<D2> for TimePoint<_Clock,D1>{
    fn sub_assign(&mut self, rhs: D2) {
        self.value.sub_assign(rhs)
    }
}

impl<D1: PartialEq<D2>,D2,_Clock: Clock> PartialEq<TimePoint<_Clock,D2>> for TimePoint<_Clock,D1>{
    fn eq(&self, other: &TimePoint<_Clock,D2>) -> bool {
        self.value.eq(&other.value)
    }
}

impl<D1: PartialOrd<D2>,D2,_Clock: Clock> PartialOrd<TimePoint<_Clock,D2>> for TimePoint<_Clock,D1>{
    fn partial_cmp(&self,other: &TimePoint<_Clock,D2>) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<D1: Eq,_Clock: Clock> Eq for TimePoint<_Clock,D1>{}

impl<D1: Ord,_Clock: Clock> Ord for TimePoint<_Clock,D1>{
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<D1: Hash,_Clock: Clock> Hash for TimePoint<_Clock,D1>{
    fn hash<H: Hasher>(&self,hasher: &mut H){
        self.value.hash(hasher)
    }
}

