mod american_greeks;
mod europian_greeks;
use crate::*;

pub trait Greeks{
	fn get_d(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> (FloatType, FloatType);
	fn delta(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType;
	fn gamma(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType;
	fn theta(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType;
	fn rho(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType;
	fn vega(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType;
	
}

impl Greeks for Tick{
	fn get_d(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> (FloatType, FloatType) {
		match option_style{
			OptionStyle::Europian => <Tick as europian_greeks::EuropianGreeks>::get_d(self, &risk_free_rate, &initial_price, &t),
			OptionStyle::American => <Tick as american_greeks::AmericanGreeks>::get_d(self, &risk_free_rate, &initial_price, &t),
		}
	}

	fn delta(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType {
		match option_style{
			OptionStyle::Europian => <Tick as europian_greeks::EuropianGreeks>::delta(self, &risk_free_rate, &initial_price, &t),
			OptionStyle::American => <Tick as american_greeks::AmericanGreeks>::delta(self, &risk_free_rate, &initial_price, &t),
		}
	}

	fn gamma(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType {
		match option_style{
			OptionStyle::Europian => <Tick as europian_greeks::EuropianGreeks>::gamma(self, &risk_free_rate, &initial_price, &t),
			OptionStyle::American => <Tick as american_greeks::AmericanGreeks>::gamma(self, &risk_free_rate, &initial_price, &t),
		}
	}
	fn theta(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType {
		match option_style{
			OptionStyle::Europian => <Tick as europian_greeks::EuropianGreeks>::theta(self, &risk_free_rate, &initial_price, &t),
			OptionStyle::American => <Tick as american_greeks::AmericanGreeks>::theta(self, &risk_free_rate, &initial_price, &t),
		}
	}
	fn rho(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType {
		match option_style{
			OptionStyle::Europian => <Tick as europian_greeks::EuropianGreeks>::rho(self, &risk_free_rate, &initial_price, &t),
			OptionStyle::American => <Tick as american_greeks::AmericanGreeks>::rho(self, &risk_free_rate, &initial_price, &t),
		}
	}
	fn vega(&self, risk_free_rate:&FloatType, initial_price:&FloatType, t:&FloatType, option_style:&OptionStyle) -> FloatType {
		match option_style{
			OptionStyle::Europian => <Tick as europian_greeks::EuropianGreeks>::vega(self, &risk_free_rate, &initial_price, &t),
			OptionStyle::American => <Tick as american_greeks::AmericanGreeks>::vega(self, &risk_free_rate, &initial_price, &t),
		}
	}


}


