mod structs;
mod greeks;
mod black_scholes;
mod exposure;
use crate::structs::*;
use greeks::*;
use crate::black_scholes::*;

use polars::prelude::*;
use std::time::{Duration, Instant};
use polars_lazy::prelude::*;
use anyhow::{Result, anyhow, ensure};
use crate::exposure::*;


fn main(){

	let mut ts:TimeSeries<OptionChain<OptionTick>> = TimeSeries::new();
	let mut oc:OptionChain<OptionTick> = OptionChain::new();
	let ot = OptionTick::builder().strike(27800.).asset_price(27602.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).side(OptionSide::Bid)
		.additional_data(AdditionalOptionData::builder().open_interest(2.).build())
		.build();
	oc.push(ot.clone());
	oc.push(ot.clone());

	ts.push(oc.clone());
	ts.push(oc);
	dbg!(ts.map(OptionChain::delta_exposure).unwrap());
	dbg!(ts.map(OptionChain::dual_delta_exposure).unwrap());

		 
}


fn eg(){
	let df = CsvReader::from_path("data/nk225e2203_20230215_call.csv").unwrap().finish().unwrap();
	let start = Instant::now();
	for i in 0..df.height(){
		let S = df.column("原資産終値").unwrap().f64().unwrap().get(i).unwrap();
		let K = df.column("権利行使価格").unwrap().f64().unwrap().get(i).unwrap();
		let T:f64 = 24./365.;
		let call_price:f64 = df.column("理論価格C").unwrap().f64().unwrap().get(i).unwrap();
		let r:f64 = 0.1492*0.01;

		let option = OptionTick::builder().strike(K).asset_price(S).expiry(T).risk_free_rate(r)
			.option_type(OptionType::Call).option_value(OptionValue::Price(call_price)).build();
		let a= option.get_implied_volatility().get_value();
		// println!("{}",a);
	}
	// println!("{} ms", start.elapsed().as_micros());


	// 各ストライクごとのbidとaskの板情報を作成
	let mut sb = StrikeBoard::new();
	sb.push(OptionTick::builder().strike(27800.).asset_price(27602.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).side(OptionSide::Bid).build());
	sb.push(OptionTick::builder().strike(27800.).asset_price(27602.).risk_free_rate(0.0015).option_value(OptionValue::Price(230.)).expiry(0.06575).option_type(OptionType::Call).side(OptionSide::Bid).build());
	sb.push(OptionTick::builder().strike(27800.).asset_price(27602.).risk_free_rate(0.0015).option_value(OptionValue::Price(250.)).expiry(0.06575).option_type(OptionType::Call).side(OptionSide::Ask).build());
	sb.push(OptionTick::builder().strike(27800.).asset_price(27602.).risk_free_rate(0.0015).option_value(OptionValue::Price(270.)).expiry(0.06575).option_type(OptionType::Call).side(OptionSide::Ask).build());

	// 特定の満期、特定のストライクのbid ask板情報の時系列データを作成
	let mut ts:TimeSeries<StrikeBoard> = TimeSeries::new();
	ts.push(sb.clone());
	ts.push(sb.clone());
	ts.push(sb.clone());

	// 板の仲値を計算し、そのIVを計算し、グリークスの時系列データを作成
	// 時系列の中身はStrikeBoard -> OptionTick -> FloatTypeに推移する
	let ts1:TimeSeries<FloatType> = ts.map(StrikeBoard::mid).map(OptionTick::get_implied_volatility).map(OptionTick::vega);
	let ts2:TimeSeries<FloatType> = ts.map(StrikeBoard::best_bid).map(OptionTick::get_implied_volatility).map(OptionTick::vanna);
	dbg!(&ts1);
	dbg!(&ts2);


	let mut oc:OptionChain<OptionTick> = OptionChain::new();
	oc.data.push(OptionTick::builder().strike(27700.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).build());
	oc.data.push(OptionTick::builder().strike(27800.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).build());
	oc.data.push(OptionTick::builder().strike(27900.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).build());
	oc.data.push(OptionTick::builder().strike(28000.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).build());

	oc.data.push(OptionTick::builder().strike(27700.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Put).build());
	oc.data.push(OptionTick::builder().strike(27800.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Put).build());
	oc.data.push(OptionTick::builder().strike(27900.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Put).build());
	oc.data.push(OptionTick::builder().strike(28000.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Put).build());

	// OptionChainの時系列データを作成
	let mut ts:TimeSeries<OptionChain<OptionTick>> = TimeSeries::new();
	ts.push(oc.clone());
	ts.push(oc.clone());
	ts.push(oc.clone());

	// OptionChainの時系列データを計算
	// ATMのIVを計算し、そのVegaを計算する
	dbg!(ts.map(OptionChain::atm).map(OptionTick::get_implied_volatility).map(OptionTick::vega));

	// 25deltaのストライクのIV差を計算
	let call25delta = ts.map(OptionChain::call_25delta).map(OptionTick::get_implied_volatility).map(OptionTick::iv);
	let put25delta = ts.map(OptionChain::put_25delta).map(OptionTick::get_implied_volatility).map(OptionTick::iv);
	let ts_delta25 = call25delta - put25delta;
	dbg!(ts_delta25);
	
	
}
