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
	dbg!(tick.delta(&0.001, &100.0, &1609439200.0, &OptionStyle::Europian));

	

}
