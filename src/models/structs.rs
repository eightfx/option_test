use serde::{Serialize, Deserialize};
use anyhow::{anyhow,Result, ensure};
use typed_builder::TypedBuilder;
use chrono::{DateTime, Utc};
use crate::black_scholes::BlackScholes;
use crate::greeks::EuropeanGreeks;
use super::extract_common_info::*;

pub type FloatType = f64;

#[derive(Clone,Debug, PartialEq, Serialize, Deserialize)]
pub enum OptionType{
	Put,
	Call
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum OptionStyle{
	European,
	American
}

#[derive(Clone,Debug, PartialEq, Serialize, Deserialize)]
pub enum OptionSide{
	Bid,
	Ask
}


#[derive(Clone,Debug, PartialEq, Serialize, Deserialize)]
pub enum OptionValue{
	Price(FloatType),
	ImpliedVolatility(FloatType)
}

#[derive(Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct AdditionalOptionData{
	pub open_interest: Option<FloatType>,
	pub volume: Option<FloatType>,
}

#[derive(Clone,Debug, TypedBuilder, Serialize, Deserialize)]
pub struct OptionTick{
	pub strike: FloatType,
	pub maturity: DateTime<Utc>,
	pub asset_price: FloatType,
	#[builder(default=0.001)]
	pub risk_free_rate: FloatType,
	#[builder(default=0.)]
	pub dividend_yield: FloatType,

	pub option_type: OptionType,
	pub option_value: OptionValue,
	#[builder(default=None, setter(strip_option))]
	pub side: Option<OptionSide>,

	#[builder(default=None, setter(strip_option))]
	pub additional_data: Option<AdditionalOptionData>,
}


impl OptionTick{
	/// Retrieve value from option_value
	/// Caution:
	/// Be careful when handling this function, as it extracts the value without distinguishing between premium and iv.
	pub fn get_value(&self) -> FloatType{
		match self.option_value{
			OptionValue::Price(p) => p,
			OptionValue::ImpliedVolatility(v) => v
		}
	}

	/// This function is used to calculate the implied volatility of an option. The function takes no input parameters and returns a value of type FloatIype.
	///
	/// The function first checks the value of the option_value field in the instance of the struct that the method is called on. If it is an OptionValue::Price, the method then calculates the implied volatility by calling the get_implied_volatility() method. If option_value is already an OptionValue::ImpliedVolatility, the function simply returns the implied volatility value.
	pub fn iv(&self) -> FloatType{
		match self.option_value{
			OptionValue::Price(_) => {
				let tick = self.get_implied_volatility();
				match tick.option_value{
					OptionValue::ImpliedVolatility(iv) => iv,
					OptionValue::Price(_) => panic!("IV calculation failed")
				}
			},
			OptionValue::ImpliedVolatility(v) => v
		}
		
	}

	pub fn tau(&self) -> FloatType{
		let now = Utc::now();
		(self.maturity - now).num_seconds() as FloatType / 31536000.
	}

}

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct StrikeBoard(pub Vec<OptionTick>);

impl StrikeBoard{
	/// The best_bid() function is a method of the StrikeBoard struct in Rust. It takes the self reference to an instance of StrikeBoard and returns the OptionTick instance with the highest value for bids.
	pub fn best_bid(&self) -> Result<OptionTick>{

		let ticks = self.0.clone();
		let bid_ticks = ticks.iter().filter(|t| matches!(t.side.as_ref().unwrap(),OptionSide::Bid)).collect::<Vec<&OptionTick>>();
		ensure!(!bid_ticks.is_empty(), "No bid ticks in strikeboard");

		let mut best_bid = bid_ticks[0].clone();
		for tick in bid_ticks{
			if tick.get_value() > best_bid.get_value(){
				best_bid = tick.clone();
			}
		}

		best_bid.side = None;
		Ok(best_bid)
	}

	/// The best_ask() function is a method of the StrikeBoard struct in Rust. It takes the self reference to an instance of StrikeBoard and returns the OptionTick instance with the lowest value for asks.
	pub fn best_ask(&self) -> Result<OptionTick>{

		let ticks = self.0.clone();
		let ask_ticks = ticks.iter().filter(|t| matches!(t.side.as_ref().unwrap(),OptionSide::Ask)).collect::<Vec<&OptionTick>>();
		ensure!(!ask_ticks.is_empty(), "No ask ticks in strikeboard");

		let mut best_ask = ask_ticks[0].clone();
		for tick in ask_ticks{
			if tick.get_value() < best_ask.get_value(){
				best_ask = tick.clone();
			}
		}

		best_ask.side = None;
		Ok(best_ask)
	}

	/// The mid() function is a method of the StrikeBoard struct in Rust. It takes the self reference to an instance of StrikeBoard and calculates the mid-point between the OptionTick instance with the highest bid value and the OptionTick instance with the lowest ask value. It then returns an OptionTick instance with the calculated mid-point value.
	pub fn mid(&self) -> Result<OptionTick>{
		let best_bid = self.best_bid();
		let best_ask = self.best_ask();

		let mid_tick =  match (best_bid, best_ask){
			(Ok(bid), Ok(ask)) => {
				let mut tick = bid.clone();
				let mid = (bid.get_value() + ask.get_value())/2.;
				tick.option_value = OptionValue::Price(mid);
				tick.side = None;
				tick
			}
			(Err(_), Ok(ask)) => {
				ask
			}
			(Ok(bid), Err(_)) => {
				bid
			}
			(Err(_), Err(_)) => {
				return Err(anyhow!("No bid or ask ticks in strikeboard"));
			}
		};
		Ok(mid_tick)
			
	}

	pub fn mid_weighted(&self) -> Result<OptionTick>{
		let best_bid = self.best_bid();
		let best_ask = self.best_ask();

		let mid_tick =  match (best_bid, best_ask){
			(Ok(bid), Ok(ask)) => {
				let mut tick = bid.clone();
				let mid = (bid.get_value() * bid.additional_data.as_ref().unwrap().volume.unwrap() + ask.get_value() * ask.additional_data.as_ref().unwrap().volume.unwrap())/(bid.additional_data.as_ref().unwrap().volume.unwrap() + ask.additional_data.as_ref().unwrap().volume.unwrap());
				tick.option_value = OptionValue::Price(mid);
				tick.side = None;
				tick
			}
			(Err(_), Ok(ask)) => {
				ask
			}
			(Ok(bid), Err(_)) => {
				bid
			}
			(Err(_), Err(_)) => {
				return Err(anyhow!("No bid or ask ticks in strikeboard"));
			}
		};
		Ok(mid_tick)
			
			
			
	}
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionChain<T: OptionBase>(pub Vec<T>);
impl<T> OptionChain<T>
where T:OptionBase
{
	pub fn map<U:OptionBase>(&self, f : impl Fn(&T) -> U) -> OptionChain<U>{
		OptionChain(
			self.0.iter().map(f).collect(),
		)
	}


}

// impl<T> OptionChain<Result<T>>{
// 	pub fn unwrap(self) -> OptionChain<T>{
// 		OptionChain(self.0.into_iter().map(|x| x.unwrap()).collect())
// 	}

// }

impl OptionChain<OptionTick>{
	pub fn otm(&self) -> Self{
		let asset_price = self.asset_price().unwrap();
		let mut otm_chain = self.clone();
		// Extract a call if the strike is higher than the underlying asset price, and extract a put if the strike is lower.
		otm_chain.0.retain(|t| (matches!(t.option_type, OptionType::Call) && t.strike >= asset_price) || (matches!(t.option_type,  OptionType::Put) && t.strike < asset_price));


		otm_chain
	}
	// pub fn atm(&self) -> OptionTick{
	// 	let asset_price = self.asset_price().unwrap();
	// 	let atm_chain = self.clone();
	// 	// Get the OptionTick of the strike closest to the underlying asset price.
	// 	//If put and call are available for the same strike, call is selected.
	// 	let mut atm_tick = atm_chain.0[0].clone();
	// 	for tick in atm_chain.0{
	// 		if (tick.strike - asset_price).abs() < (atm_tick.strike - asset_price).abs(){
	// 			atm_tick = tick.clone();
	// 		}
	// 	}
	// 	atm_tick

	// }


	pub fn atm(&self) -> OptionTick{
		let asset_price = self.asset_price().unwrap();
		let atm_chain = self.clone().otm();
		// Perform linear interpolation of put and call
		let put = atm_chain.put();
		let call = atm_chain.call();
		// Get the otm put and call closest to asset_price
		let best_put:&OptionTick;
		let best_call:&OptionTick;
		if put.0.is_empty() && call.0.is_empty(){
			panic!("There is no put or call in the option chain.");
		}
		else if put.0.is_empty(){
			best_call = call.0.iter().min_by(|a, b| (a.strike - asset_price).abs().partial_cmp(&(b.strike - asset_price).abs()).unwrap()).unwrap();
			best_put = best_call;
		}
		else if call.0.is_empty(){
			best_put = put.0.iter().min_by(|a, b| (a.strike - asset_price).abs().partial_cmp(&(b.strike - asset_price).abs()).unwrap()).unwrap();
			best_call = best_put;
		} else{
			best_put = put.0.iter().min_by(|a, b| (a.strike - asset_price).abs().partial_cmp(&(b.strike - asset_price).abs()).unwrap()).unwrap();
			best_call = call.0.iter().min_by(|a, b| (a.strike - asset_price).abs().partial_cmp(&(b.strike - asset_price).abs()).unwrap()).unwrap();
			
		}
		
		// linear interpolation
		let strike = asset_price;
		let value = best_put.get_value() + (best_call.get_value() - best_put.get_value()) * (asset_price - best_put.strike) / (best_call.strike - best_put.strike);

		let option_value:OptionValue = match best_put.option_value{
			OptionValue::Price(_) => OptionValue::Price(value),
			OptionValue::ImpliedVolatility(_) => OptionValue::ImpliedVolatility(value),
		};

		let mut tick: OptionTick = best_put.to_owned();

		tick.strike = strike;
		tick.option_value = option_value;
		tick
			
	}

	pub fn call(&self) -> Self{
		let mut call_chain = self.clone();
		call_chain.0.retain(|t| matches!(t.option_type, OptionType::Call));


		call_chain
	}

	pub fn put(&self) -> Self{
		let mut put_chain = self.clone();
		put_chain.0.retain(|t| matches!(t.option_type, OptionType::Put));

		put_chain
	}

	pub fn sort_by_strike(&self) -> Self{
		let mut sorted_chain = self.clone();
		sorted_chain.0.sort_by(|a,b| a.strike.partial_cmp(&b.strike).unwrap());
		sorted_chain
	}

	pub fn call_25delta(&self) -> OptionTick{
		let call_chain = self.call();
		let delta_chain:Vec<FloatType> = call_chain.0.iter().map(|t| t.delta()).collect();

		// Get the index with delta closest to 0.25
		let mut index = 0;
		let mut delta = delta_chain[0];
		for (i, d) in delta_chain.iter().enumerate(){
			if (d - 0.25).abs() < (delta - 0.25).abs(){
				index = i;
				delta = *d;
			}
	}

		call_chain.0[index].clone()
	}

	pub fn call_50delta(&self) -> OptionTick{
		let call_chain = self.call();
		let delta_chain:Vec<FloatType> = call_chain.0.iter().map(|t| t.delta()).collect();

		// Get the index with delta closest to 0.5
		let mut index = 0;
		let mut delta = delta_chain[0];
		for (i, d) in delta_chain.iter().enumerate(){
			if (d - 0.5).abs() < (delta - 0.5).abs(){
				index = i;
				delta = *d;
			}
		}
		call_chain.0[index].clone()
	}

	pub fn put_25delta(&self) -> OptionTick{
		let put_chain = self.put();
		let delta_chain:Vec<FloatType> = put_chain.0.iter().map(|t| t.delta()).collect();

		// Get the index with delta closest to -0.25
		let mut index = 0;
		let mut delta = delta_chain[0];
		for (i, d) in delta_chain.iter().enumerate(){
			if (d + 0.25).abs() < (delta + 0.25).abs(){
				index = i;
				delta = *d;
			}
		}
		put_chain.0[index].clone()
	}

	pub fn put_50delta(&self) -> OptionTick{
		let put_chain = self.put();
		let delta_chain:Vec<FloatType> = put_chain.0.iter().map(|t| t.delta()).collect();

		// Get the index with delta closest to -0.5
		let mut index = 0;
		let mut delta = delta_chain[0];
		for (i, d) in delta_chain.iter().enumerate(){
			if (d + 0.5).abs() < (delta + 0.5).abs(){
				index = i;
				delta = *d;
			}
		}
		put_chain.0[index].clone()
	}

}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OptionBoard<T:OptionBase>(pub Vec<OptionChain<T>>);


impl<T> OptionBoard<T>
where T:OptionBase 
{
	// pub fn get_front_month(&self) -> OptionChain<T>{
	// 	let mut front_month = self.0[0].clone();
	// 	for chain in self.0.iter(){
	// 		if chain.maturity() < front_month.maturity(){
	// 			front_month = chain.clone();
	// 		}
	// 	}
	// 	front_month
	// }

	pub fn get0(&self) -> OptionChain<T>{
		self.0[0].clone()
	}

	// pub fn sort_by_maturity(&self) -> Self{
	// 	let mut sorted_board = self.clone();
	// 	sorted_board.0.sort_by(|a,b| a.maturity().partial_cmp(&b.maturity()).unwrap());
	// 	sorted_board
	// }

	
}

pub trait OptionBase : Clone{}
impl OptionBase for OptionTick{}
impl OptionBase for StrikeBoard{}

