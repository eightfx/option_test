mod structs;
mod greeks;
mod black_scholes;
use crate::structs::*;
use greeks::*;
use crate::black_scholes::*;

use polars::prelude::*;
use std::time::{Duration, Instant};
use polars_lazy::prelude::*;

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

	let df = CsvReader::from_path("data/nk225e2203_20230215_call.csv").unwrap().finish().unwrap();
	let start = Instant::now();
	for i in 0..df.height(){
		let S = df.column("原資産終値").unwrap().f64().unwrap().get(i).unwrap();
		let K = df.column("権利行使価格").unwrap().f64().unwrap().get(i).unwrap();
		let T:f64 = 24./365.;
		let call_price:f64 = df.column("理論価格C").unwrap().f64().unwrap().get(i).unwrap();
		let r:f64 = 0.1492*0.01;

		let option = OptionTick::builder().strike(K).asset_price(S).risk_free_rate(r).expiry(T).option_type(OptionType::Call).premium(call_price).build();
		println!("{:?}", option.get_implied_volatility(0.2, 0.0001));
	}
	println!("{:?}", start.elapsed());




	
}
