use probability::prelude::*;
use crate::*;

pub trait EuropianGreeks{
	fn get_d(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> (FloatType, FloatType);
	fn delta(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
	fn gamma(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
	fn theta(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
	fn rho(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
	fn vega(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType;
}
impl EuropianGreeks for Tick{
	fn get_d(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> (FloatType, FloatType) {

		let t = tick.get_expiry(t);
		let d1:FloatType;
		let d2:FloatType;
		d1 = ((initial_price / &tick.strike).log10() + (risk_free_rate + 0.5 * tick.implied_volatility * tick.implied_volatility) * t) / (&tick.implied_volatility * t.sqrt()) ;

		d2 = d1 - &tick.implied_volatility * &t.sqrt();

		(d1, d2)
	}
	
	fn delta(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		let (d1, _) = <Tick as EuropianGreeks>::get_d(tick, &risk_free_rate, &initial_price, &t);
		let g = Gaussian::new(0.0, 1.0);
		match tick.option_type{
			OptionType::Call => g.distribution(d1),
			OptionType::Put => g.distribution(d1) - 1.0,
		}
	}

	fn gamma(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		let (d1, _) = <Tick as EuropianGreeks>::get_d(tick, &risk_free_rate, &initial_price, &t);
		let t = tick.get_expiry(t);
		(-0.5 * (d1 * d1)).exp() / ((tick.implied_volatility * initial_price * t.sqrt()) * ((2. * FloatType::from(std::f64::consts::PI)).sqrt()))
	}

	fn theta(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		let (d1, d2) = <Tick as EuropianGreeks>::get_d(tick, &risk_free_rate, &initial_price, &t);
		let t = tick.get_expiry(t);
		let g = Gaussian::new(0.0, 1.0);
		match tick.option_type{
			OptionType::Call =>
				- risk_free_rate * tick.strike * (-risk_free_rate * t).exp() * g.distribution(d2) - tick.implied_volatility * initial_price * (-0.5 * (d1 * d1)).exp() / (2. * t.sqrt() * ((2. * FloatType::from(std::f64::consts::PI)).sqrt())),
			OptionType::Put =>
				- risk_free_rate * tick.strike * (-risk_free_rate * t).exp() * (g.distribution(d2)-1.) + tick.implied_volatility * initial_price * (-0.5 * (d1 * d1)).exp() / (2. * t.sqrt() * ((2. * FloatType::from(std::f64::consts::PI)).sqrt())),
		}

	}

	fn rho(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		let (_, d2) = <Tick as EuropianGreeks>::get_d(tick, &risk_free_rate, &initial_price, &t);
		let t = tick.get_expiry(t);
		let g = Gaussian::new(0.0, 1.0);
		match tick.option_type{
			OptionType::Call => t*tick.strike*(-risk_free_rate*t).exp()*g.distribution(d2),
			OptionType::Put => - t*tick.strike*(-risk_free_rate*t).exp()*g.distribution(-d2)

		}

	}

	fn vega(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		let (d1, _) = <Tick as EuropianGreeks>::get_d(tick, &risk_free_rate, &initial_price, &t);
		let t = tick.get_expiry(t);

		tick.strike * t.sqrt() * (-0.5 * (d1 * d1)).exp() / ((2. * FloatType::from(std::f64::consts::PI)).sqrt())
	}
	
}
