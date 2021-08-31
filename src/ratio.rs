use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Ratio<const N: i64, const D: i64>;

#[doc(hidden)]
mod sealed {
    pub trait Sealed {}
}

pub trait Period: sealed::Sealed {
    const NUMERATOR: i64;
    const DENOMINATOR: i64;
}

const fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

const fn gcd128(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd128(b, a % b)
    }
}

const fn signum(a: i64) -> i64 {
    match a {
        n if n < 0 => -1,
        0 => 0,
        _ => 1,
    }
}

impl<const N: i64, const D: i64> From<Ratio<N, D>> for f64 {
    fn from(_: Ratio<N, D>) -> f64 {
        (Ratio::<N, D>::NUMERATOR as f64) / (Ratio::<N, D>::DENOMINATOR as f64)
    }
}

impl<const N: i64, const D: i64> sealed::Sealed for Ratio<N, D> {}

impl<const N: i64, const D: i64> Period for Ratio<N, D> {
    const NUMERATOR: i64 = signum(D) * N / gcd(N, D);
    const DENOMINATOR: i64 = signum(D) * D / gcd(N, D);
}

const fn ratio_divide(r1: (i64, i64), r2: (i64, i64)) -> (i64, i64) {
    let num = (r1.0 as i128) * (r2.1 as i128);
    let denom = (r2.0 as i128) * (r1.1 as i128);
    (
        (num / gcd128(num, denom)) as i64,
        (denom / gcd128(num, denom)) as i64,
    )
}

const fn ratio_multiply(r1: (i64, i64), r2: (i64, i64)) -> (i64, i64) {
    let num = (r1.0 as i128) * (r1.1 as i128);
    let denom = (r2.0 as i128) * (r2.1 as i128);
    (
        (num / gcd128(num, denom)) as i64,
        (denom / gcd128(num, denom)) as i64,
    )
}

pub struct RatioMultiply<A, B>(PhantomData<A>, PhantomData<B>);

impl<A: sealed::Sealed, B: sealed::Sealed> sealed::Sealed for RatioMultiply<A, B> {}

impl<A: Period, B: Period> Period for RatioMultiply<A, B> {
    const NUMERATOR: i64 = ratio_multiply(
        (A::NUMERATOR, A::DENOMINATOR),
        (B::NUMERATOR, B::DENOMINATOR),
    )
    .0;
    const DENOMINATOR: i64 = ratio_multiply(
        (A::NUMERATOR, A::DENOMINATOR),
        (B::NUMERATOR, B::DENOMINATOR),
    )
    .1;
}

pub struct RatioDivide<A, B>(PhantomData<A>, PhantomData<B>);

impl<A: sealed::Sealed, B: sealed::Sealed> sealed::Sealed for RatioDivide<A, B> {}

impl<A: Period, B: Period> Period for RatioDivide<A, B> {
    const NUMERATOR: i64 = ratio_divide(
        (A::NUMERATOR, A::DENOMINATOR),
        (B::NUMERATOR, B::DENOMINATOR),
    )
    .0;
    const DENOMINATOR: i64 = ratio_divide(
        (A::NUMERATOR, A::DENOMINATOR),
        (B::NUMERATOR, B::DENOMINATOR),
    )
    .1;
}

pub struct Reciprocal<R>(PhantomData<R>);

impl<R: sealed::Sealed> sealed::Sealed for Reciprocal<R> {}

impl<R: Period> Period for Reciprocal<R> {
    const NUMERATOR: i64 = R::DENOMINATOR;
    const DENOMINATOR: i64 = R::NUMERATOR;
}

pub type Tera = Ratio<1_000_000_000_000, 1>;
pub type Giga = Ratio<1_000_000_000, 1>;
pub type Mega = Ratio<1_000_000, 1>;
pub type Kilo = Ratio<1_000, 1>;
pub type Unit = Ratio<1, 1>;
pub type Mili = Ratio<1, 1_000>;
pub type Micro = Ratio<1, 1_000_000>;
pub type Nano = Ratio<1, 1_000_000_000>;
pub type Pico = Ratio<1, 1_000_000_000_000>;
