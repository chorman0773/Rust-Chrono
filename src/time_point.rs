
use crate::{duration::IsDuration, prelude::*};

use std::{marker::PhantomData};

pub struct TimePoint<Clock: ?Sized,Duration>{
    value: Duration,
    _phantom: PhantomData<Clock>
}

impl<_Clock: Clock + ?Sized,_Duration: IsDuration> TimePoint<_Clock,_Duration>{
    pub fn new(value: _Duration) -> Self{
        Self{
            value,
            _phantom: PhantomData
        }
    }

    pub fn into_inner(self) -> _Duration{
        self.value
    }

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
