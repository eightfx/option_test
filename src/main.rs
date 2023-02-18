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
	let df = CsvReader::from_path("data/nk225e2203_20230215_call.csv").unwrap().finish().unwrap();
	let start = Instant::now();
	for i in 0..df.height(){
		let S = df.column("原資産終値").unwrap().f64().unwrap().get(i).unwrap();
		let K = df.column("権利行使価格").unwrap().f64().unwrap().get(i).unwrap();
		let T:f64 = 24./365.;
		let call_price:f64 = df.column("理論価格C").unwrap().f64().unwrap().get(i).unwrap();
		let r:f64 = 0.1492*0.01;

		let option = OptionTick{
			strike:K,
			asset_price:S,
			expiry:T,
			risk_free_rate:r,
			dividend_yield:0.0,
			option_type:OptionType::Call,
			option_value:OptionValue::Price(call_price),
		};
		let a= option.get_implied_volatility(0.2, 0.0001).get_value();
		println!("{}",a);
	}
	println!("{} ms", start.elapsed().as_micros());

	let mut ts = TimeSeries::new();
	ts.push(OptionTick::builder().strike(250.).asset_price(100.).risk_free_rate(0.001).option_value(OptionValue::ImpliedVolatility(20.)).expiry(30./365.).option_type(OptionType::Put).build());
	ts.push(OptionTick::builder().strike(250.).asset_price(100.).risk_free_rate(0.001).option_value(OptionValue::ImpliedVolatility(20.)).expiry(30./365.).option_type(OptionType::Put).build());
	dbg!(ts.map(OptionTick::delta));
	dbg!(ts.0[1].delta());
	

	
}
