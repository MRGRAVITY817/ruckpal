use serde::Serialize;
use std::ops::{Add, Sub};
use validator::ValidationError;

#[derive(Clone, Copy, Serialize)]
pub struct Money(pub i64);

impl Money {
    pub fn is_positive(&self) -> bool {
        self.0.is_positive()
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

pub fn is_money_positive(money: &Money) -> Result<(), ValidationError> {
    if !money.is_positive() {
        return Err(
            ValidationError::new("amount of money isn't positive").into()
        );
    }

    Ok(())
}

// package io.reflectoring.buckpal.application.domain.model;
//
// import java.math.BigInteger;
//
// import lombok.NonNull;
// import lombok.Value;
//
// @Value
// public class Money {
//
// 	public static Money ZERO = Money.of(0L);
//
// 	@NonNull
// 	private final BigInteger amount;
//
// 	public boolean isPositiveOrZero(){
// 		return this.amount.compareTo(BigInteger.ZERO) >= 0;
// 	}
//
// 	public boolean isNegative(){
// 		return this.amount.compareTo(BigInteger.ZERO) < 0;
// 	}
//
// 	public boolean isPositive(){
// 		return this.amount.compareTo(BigInteger.ZERO) > 0;
// 	}
//
// 	public boolean isGreaterThanOrEqualTo(Money money){
// 		return this.amount.compareTo(money.amount) >= 0;
// 	}
//
// 	public boolean isGreaterThan(Money money){
// 		return this.amount.compareTo(money.amount) >= 1;
// 	}
//
// 	public static Money of(long value) {
// 		return new Money(BigInteger.valueOf(value));
// 	}
//
// 	public static Money add(Money a, Money b) {
// 		return new Money(a.amount.add(b.amount));
// 	}
//
// 	public Money minus(Money money){
// 		return new Money(this.amount.subtract(money.amount));
// 	}
//
// 	public Money plus(Money money){
// 		return new Money(this.amount.add(money.amount));
// 	}
//
// 	public static Money subtract(Money a, Money b) {
// 		return new Money(a.amount.subtract(b.amount));
// 	}
//
// 	public Money negate(){
// 		return new Money(this.amount.negate());
// 	}
//
// }
