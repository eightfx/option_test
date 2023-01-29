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
		d1 = ((initial_price / &tick.strike).log(std::f64::consts::E) + (risk_free_rate + 0.5 * tick.implied_volatility * tick.implied_volatility) * t) / (&tick.implied_volatility * t.sqrt()) ;

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
		(-0.5 * d1*d1).exp() / (tick.implied_volatility * initial_price * t.sqrt() *  (2. * FloatType::from(std::f64::consts::PI)).sqrt())
	}

	fn theta(tick:&Tick, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType) -> FloatType {
		let (d1, d2) = <Tick as EuropianGreeks>::get_d(tick, &risk_free_rate, &initial_price, &t);
		let t = tick.get_expiry(t);
		let g = Gaussian::new(0.0, 1.0);
		match tick.option_type{
			OptionType::Call =>
				- risk_free_rate * tick.strike * (-risk_free_rate * t).exp() * g.distribution(d2) - tick.implied_volatility * initial_price * (-0.5 * (d1 * d1)).exp() / (2. * t.sqrt() * ((2. * FloatType::from(std::f64::consts::PI)).sqrt())),
			OptionType::Put =>
				risk_free_rate * tick.strike * (-risk_free_rate * t).exp() * (g.distribution(-d2)) - tick.implied_volatility * initial_price * (-0.5 * (d1 * d1)).exp() / (2. * t.sqrt() * ((2. * FloatType::from(std::f64::consts::PI)).sqrt())),
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

		initial_price* t.sqrt() * (-0.5 * (d1 * d1)).exp() / ((2. * FloatType::from(std::f64::consts::PI)).sqrt())
	}
	
}


#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn greeks_call(){
		let tick = Tick{
			strike: 250.0,
			option_type: OptionType::Call,
			expiry: 60.*60.*24.*30.,
			open_interest: 0.0,
			implied_volatility: 10.,
		};
		let risk_free_rate = 0.001;
		let underlying_price = 100.0;
		let now_timestamp = 0.;
		
		assert_float_relative_eq!(tick.delta(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian), 0.867,0.01);
		assert_float_relative_eq!(tick.theta(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian), -374.163,0.01);
		assert_float_relative_eq!(tick.rho(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian),0.818,0.01);
		assert_float_relative_eq!(tick.gamma(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian), 0.0007483,0.0001);
		assert_float_relative_eq!(tick.vega(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian), 6.151,0.01);

	}

	#[test]
	fn greeks_put(){
		let tick = Tick{
			strike: 250.0,
			option_type: OptionType::Put,
			expiry: 60.*60.*24.*30.,
			open_interest: 0.0,
			implied_volatility: 10.,
		};
		let risk_free_rate = 0.001;
		let underlying_price = 100.0;
		let now_timestamp = 0.;
		
		assert_float_relative_eq!(tick.delta(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian), -0.132666,0.01);
		assert_float_relative_eq!(tick.theta(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian), -373.9,0.01);
		assert_float_relative_eq!(tick.rho(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian), -19.7285,0.01);
		assert_float_relative_eq!(tick.gamma(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian), 0.0007483,0.0001);
		assert_float_relative_eq!(tick.vega(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian), 6.151,0.01);

	}
	
} 
