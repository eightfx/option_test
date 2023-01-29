mod structs;
mod greeks;
use crate::structs::*;
use crate::greeks::Greeks;

fn main() {
	let tick = Tick{
		strike: 100.0,
		option_type: OptionType::Call,
		expiry: 1609459200.0,
		open_interest: 0.0,
		implied_volatility: 0.2,
	};
	let risk_free_rate = 0.001;
	let underlying_price = 100.0;
	let now_timestamp = 1609359200.0;
	
	dbg!(tick.delta(&risk_free_rate, &underlying_price, &now_timestamp, &OptionStyle::Europian));

	

}
