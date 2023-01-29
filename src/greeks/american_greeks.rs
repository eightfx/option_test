use probability::prelude::*;
use crate::*;

pub trait AmericanGreeks{

	fn get_d(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> (FloatType, FloatType);
	fn delta(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
	fn gamma(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
	fn theta(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
	fn rho(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
	fn vega(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
}
impl AmericanGreeks for Tick{
	fn get_d(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> (FloatType, FloatType) {
		// TODO
		(0.0, 0.0)
	}
	fn delta(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		// TODO
		0.0
	}
	fn gamma(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		// TODO
		0.0
	}
	fn theta(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		// TODO
		0.0
	}
	fn rho(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		// TODO
		0.0
	}

	fn vega(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		// TODO
		0.0
	}

}
