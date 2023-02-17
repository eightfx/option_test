mod structs;
mod greeks;
mod black_scholes;
use crate::structs::*;
use greeks::*;
use crate::black_scholes::*;


fn main(){
	// let tick = OptionTick::builder().strike(27588.).expiry(0.065).implied_volatility(0.000518)
	// 	.asset_price(27602.).premium(370.).option_type(OptionType::Call).build();
	
	let option = OptionTick::builder().strike(250.).asset_price(100.).risk_free_rate(0.001).implied_volatility(10.).expiry(30./365.).option_type(OptionType::Put).premium(30.).build();
	dbg!(option.theoretical_price());
	dbg!(option.get_implied_volatility(9., 0.0001));
	dbg!(option.delta());
	dbg!(option.gamma());
	dbg!(option.vomma());
	dbg!(option.color());


	
}
