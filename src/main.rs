mod structs;
mod greeks;
use crate::structs::*;
use crate::greeks::Greeks;

#[macro_use]
extern crate assert_float_eq;

fn main() {
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
	
	dbg!(tick.delta(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian));
	

}
