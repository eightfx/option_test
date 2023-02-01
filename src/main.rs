mod structs;
mod greeks;
use crate::structs::*;
use crate::greeks::Greeks;
#[macro_use]
extern crate assert_float_eq;

fn main() {
	let strike = 250.;
	let spot = 100.;
	let ts_expiry = 60.*60.*24.*30.;
	let ts_now = 0.;
	let iv = 10.;
	let risk_free_rate = 0.001;
	let dividend_yield = 0.;

	dbg!(greeks::greeks::delta(&strike, &spot, &ts_expiry, &ts_now, &iv, &OptionType::Call, &OptionStyle::European, &risk_free_rate,&dividend_yield));

}
