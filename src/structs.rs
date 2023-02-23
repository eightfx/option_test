use crate::greeks::EuropeanGreeks;
use crate::black_scholes::*;
use anyhow::{anyhow,Result, ensure};
use typed_builder::TypedBuilder;
use chrono::{DateTime, Utc};

pub type FloatType = f64;

#[derive(Clone,Debug, PartialEq)]
pub enum OptionType{
	Put,
	Call
}

#[derive(Clone, PartialEq)]
pub enum OptionStyle{
	European,
	American
}

#[derive(Clone,Debug, PartialEq)]
pub enum OptionSide{
	Bid,
	Ask
}

#[derive(Clone,Debug, PartialEq)]
pub enum OptionValue{
	Price(FloatType),
	ImpliedVolatility(FloatType)
}

#[derive(Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct AdditionalOptionData{
	pub open_interest: Option<FloatType>,
	pub volume: Option<FloatType>,
}

#[derive(Clone,Debug, TypedBuilder)]
pub struct OptionTick{
	pub strike: FloatType,
	pub expiry: FloatType,
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
	/// 	Caution:
	/// Be careful when handling this function, as it extracts the value without distinguishing between premium and iv.
	pub fn get_value(&self) -> FloatType{
		match self.option_value{
			OptionValue::Price(p) => p,
			OptionValue::ImpliedVolatility(v) => v
		}
	}

	/// This function is used to calculate the implied volatility of an option. The function takes no input parameters and returns a value of type FloatIype.
	///
	/// 	The function first checks the value of the option_value field in the instance of the struct that the method is called on. If it is an OptionValue::Price, the method then calculates the implied volatility by calling the get_implied_volatility() method. If option_value is already an OptionValue::ImpliedVolatility, the function simply returns the implied volatility value.
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

}

#[derive(Clone,Debug)]
pub struct StrikeBoard(pub Vec<OptionTick>);

impl StrikeBoard{
	pub fn new() -> Self{
		Self(Vec::new())
	}

	pub fn push(&mut self, tick: OptionTick){
		self.0.push(tick);
	}
	/// 	The best_bid() function is a method of the StrikeBoard struct in Rust. It takes the self reference to an instance of StrikeBoard and returns the OptionTick instance with the highest value for bids.
	pub fn best_bid(&self) -> Result<OptionTick>{

		let ticks = self.0.clone();
		let bid_ticks = ticks.iter().filter(|t| matches!(t.side.as_ref().unwrap(),OptionSide::Bid)).collect::<Vec<&OptionTick>>();
		ensure!(bid_ticks.len() > 0, "No bid ticks in strikeboard");

		let mut best_bid = bid_ticks[0].clone();
		for tick in bid_ticks{
			if tick.get_value() > best_bid.get_value(){
				best_bid = tick.clone();
			}
		}
		Ok(best_bid)
	}

	/// The best_ask() function is a method of the StrikeBoard struct in Rust. It takes the self reference to an instance of StrikeBoard and returns the OptionTick instance with the lowest value for asks.
	pub fn best_ask(&self) -> Result<OptionTick>{

		let ticks = self.0.clone();
		let ask_ticks = ticks.iter().filter(|t| matches!(t.side.as_ref().unwrap(),OptionSide::Ask)).collect::<Vec<&OptionTick>>();
		ensure!(ask_ticks.len() > 0, "No ask ticks in strikeboard");

		let mut best_ask = ask_ticks[0].clone();
		for tick in ask_ticks{
			if tick.get_value() < best_ask.get_value(){
				best_ask = tick.clone();
			}
		}
		Ok(best_ask)
	}

	/// The mid() function is a method of the StrikeBoard struct in Rust. It takes the self reference to an instance of StrikeBoard and calculates the mid-point between the OptionTick instance with the highest bid value and the OptionTick instance with the lowest ask value. It then returns an OptionTick instance with the calculated mid-point value.
	pub fn mid(&self) -> Result<OptionTick>{
		let best_bid = self.best_bid()?;
		let best_ask = self.best_ask()?;
		let mid = (best_bid.get_value() + best_ask.get_value())/2.;
		let mut mid_tick = best_bid.clone();
		mid_tick.option_value = OptionValue::Price(mid);
		Ok(mid_tick)
		
	}
	
}

/// This trait automatically builds OptionChain, OptionBoard, StrikeBoard, etc. by simply entering an OptionTick.
pub trait CRUD{
	// fn create(&mut self, tick: OptionTick);
	// fn read(&self, strike: FloatType) -> Option<OptionTick>;
	fn upsert(&mut self, tick: OptionTick);
	fn delete(&mut self, tick:OptionTick);
}

impl CRUD for StrikeBoard{
	fn upsert(&mut self, tick: OptionTick) {
		let mut ticks = self.0.clone();
		let mut index = 0;
		let mut found = false;
		for (i, t) in ticks.iter().enumerate(){
			if t.option_value == tick.option_value &&  t.side == tick.side{
				index = i;
				found = true;
				break;
			}
		}
		if found{
			ticks[index] = tick;
		}else{
			ticks.push(tick);
		}
		self.0 = ticks;
	}

	fn delete(&mut self, tick:OptionTick) {
		let mut ticks = self.0.clone();
		let mut index = 0;
		let mut found = false;
		for (i, t) in ticks.iter().enumerate(){
			if t.option_value == tick.option_value &&  t.side == tick.side{
				index = i;
				found = true;
				break;
			}
		}
		if found{
			ticks.remove(index);
		}
		self.0 = ticks;

	}


	
}

impl CRUD for OptionChain<StrikeBoard>{
	fn upsert(&mut self, tick: OptionTick) {
		let mut strike_boards = self.data.clone();
		let mut index = 0;
		let mut found = false;
		for (i, sb) in strike_boards.iter().enumerate(){
			if sb.strike().unwrap() == tick.strike && sb.option_type().unwrap() == tick.option_type{
				index = i;
			found = true;
				break;
			}
		}
		if found{
			strike_boards[index].upsert(tick);
		}else{
			let mut sb = StrikeBoard::new();
			sb.push(tick);
			strike_boards.push(sb);
		}
		self.data = strike_boards;

	}
	fn delete(&mut self, tick:OptionTick) {
		let mut strike_boards = self.data.clone();
		let mut index = 0;
		let mut found = false;
		for (i, sb) in strike_boards.iter().enumerate(){
			if sb.strike().unwrap() == tick.strike && sb.option_type().unwrap() == tick.option_type{
				index = i;
				found = true;
				break;
			}
		}
		if found{
			strike_boards[index].delete(tick);
			if strike_boards[index].0.len() == 0{
				strike_boards.remove(index);
			}
		}
		self.data = strike_boards;
	}
	
}

// impl CRUD for OptionBoard<StrikeBoard>{
// 	fn upsert(&mut self, tick: OptionTick) {
// 		let mut option_chains = self.data.clone();
// 		let mut index = 0;
// 		let mut found = false;
// 		for (i, oc) in option_chains.iter().enumerate(){
// 			if oc.expiry().unwrap() == tick.expiry{
// 				index = i;
// 				found = true;
// 				break;
// 			}
// 		}
// 		if found{
// 			option_chains[index].upsert(tick);
// 		}else{
// 			let mut oc = OptionChain::new();
// 			oc.push(tick);
// 			option_chains.push(oc);
// 		}
// 		self.data = option_chains;
// 	}
// 	fn delete(&mut self, tick:OptionTick) {
// 		let mut option_chains = self.data.clone();
// 		let mut index = 0;
// 		let mut found = false;
// 		for (i, oc) in option_chains.iter().enumerate(){
// 			if oc.expiration().unwrap() == tick.expiration{
// 				index = i;
// 				found = true;
// 				break;
// 			}
// 		}
// 		if found{
// 			option_chains[index].delete(tick);
// 			if option_chains[index].data.len() == 0{
// 				option_chains.remove(index);
// 			}
// 		}
// 		self.data = option_chains;
// 	}


// }


/// Trait to retrieve common information
/// For example, since OptionChain is a set of OptionTicks with the same maturity, this trait can be used to retrieve maturity information.
/// If it tries to retrieve information that is not common information, it returns None.
pub trait ExtractCommonInfo{
	fn strike(&self) -> Result<FloatType>{Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))}
	fn expiry(&self) -> Result<FloatType>{Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))}
	fn asset_price(&self) -> Result<FloatType>{Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))}
	fn risk_free_rate(&self) -> Result<FloatType>{Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))}
	fn dividend_yield(&self) -> Result<FloatType>{Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))}
	fn option_type(&self) -> Result<OptionType>{Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))}
	fn option_value(&self) -> Result<OptionValue>{Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))}
	fn side(&self) -> Result<OptionSide>{Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))}

} 

impl ExtractCommonInfo for OptionChain<OptionTick>{
	fn asset_price(&self) -> Result<FloatType>{
		Ok(self.data[0].asset_price)
	}
	
}

impl ExtractCommonInfo for OptionChain<StrikeBoard>{
	fn expiry(&self) -> Result<FloatType>{
		Ok(self.data[0].0[0].expiry)
	}
	fn risk_free_rate(&self) -> Result<FloatType>{
		Ok(self.data[0].0[0].risk_free_rate)
	}
	fn dividend_yield(&self) -> Result<FloatType>{
		Ok(self.data[0].0[0].dividend_yield)
	}

	
}


impl ExtractCommonInfo for StrikeBoard{
	fn strike(&self) -> Result<FloatType>{
		Ok(self.0[0].strike)
	}
	fn option_type(&self) -> Result<OptionType> {
		Ok(self.0[0].option_type.clone())
	}

	
}

#[derive(Clone, Debug, TypedBuilder)]
pub struct OptionChain<T>{
	#[builder(default=Vec::new())]
	pub data : Vec<T>,
	pub maturity: DateTime<Utc>,
	
}
impl<T> OptionChain<T>{
	pub fn map<U>(&self, f : impl Fn(&T) -> U) -> OptionChain<U>{
		OptionChain{
			data: self.data.iter().map(f).collect(),
			maturity: self.maturity.clone()
		}
	}
	
}

impl OptionChain<OptionTick>{
	pub fn push(&mut self, tick: OptionTick){
		self.data.push(tick);
	}

	pub fn otm(&self) -> Self{
		let asset_price = self.asset_price().unwrap();
		let mut otm_chain = self.clone();
		// Extract a call if the strike is higher than the underlying asset price, and extract a put if the strike is lower.
		otm_chain.data = otm_chain.data.into_iter().filter(|t| (matches!(t.option_type, OptionType::Call) && t.strike >= asset_price) || (matches!(t.option_type,  OptionType::Put) && t.strike < asset_price)).collect();

		otm_chain
	}
	pub fn atm(&self) -> OptionTick{
		let asset_price = self.asset_price().unwrap();
		let atm_chain = self.clone();
		// Get the OptionTick of the strike closest to the underlying asset price.
		//If put and call are available for the same strike, call is selected.
		let mut atm_tick = atm_chain.data[0].clone();
		for tick in atm_chain.data{
			if (tick.strike - asset_price).abs() < (atm_tick.strike - asset_price).abs(){
				atm_tick = tick.clone();
			}
		}
		atm_tick

	}

	pub fn call(&self) -> Self{
		let mut call_chain = self.clone();
		call_chain.data = call_chain.data.into_iter().filter(|t| matches!(t.option_type, OptionType::Call)).collect();
		call_chain
	}

	pub fn put(&self) -> Self{
		let mut put_chain = self.clone();
		put_chain.data = put_chain.data.into_iter().filter(|t| matches!(t.option_type, OptionType::Put)).collect();
		put_chain
	}

	pub fn sort_by_strike(&self) -> Self{
		let mut sorted_chain = self.clone();
		sorted_chain.data.sort_by(|a,b| a.strike.partial_cmp(&b.strike).unwrap());
		sorted_chain
	}

	pub fn call_25delta(&self) -> OptionTick{
		let call_chain = self.call();
		let delta_chain = call_chain.map(OptionTick::delta);

		// Get the index with delta closest to 0.25
		let mut index = 0;
		let mut delta = delta_chain.data[0];
		for i in 0..delta_chain.data.len(){
			if (delta_chain.data[i] - 0.25).abs() < (delta - 0.25).abs(){
				index = i;
				delta = delta_chain.data[i];
			}
		}

		call_chain.data[index].clone()
	}

	pub fn call_50delta(&self) -> OptionTick{
		let call_chain = self.call();
		let delta_chain = call_chain.map(OptionTick::delta);

		// Get the index with delta closest to 0.5
		let mut index = 0;
		let mut delta = delta_chain.data[0];
		for i in 0..delta_chain.data.len(){
			if (delta_chain.data[i] - 0.5).abs() < (delta - 0.5).abs(){
				index = i;
				delta = delta_chain.data[i];
			}
		}

		call_chain.data[index].clone()
	}

	pub fn put_25delta(&self) -> OptionTick{
		let put_chain = self.put();
		let delta_chain = put_chain.map(OptionTick::delta);

		// Get the index with delta closest to -0.25
		let mut index = 0;
		let mut delta = delta_chain.data[0];
		for i in 0..delta_chain.data.len(){
			if (delta_chain.data[i] + 0.25).abs() < (delta + 0.25).abs(){
				index = i;
				delta = delta_chain.data[i];
			}
		}

		put_chain.data[index].clone()
	}

	pub fn put_50delta(&self) -> OptionTick{
		let put_chain = self.put();
		let delta_chain = put_chain.map(OptionTick::delta);

		// Get the index with delta closest to -0.5
		let mut index = 0;
		let mut delta = delta_chain.data[0];
		for i in 0..delta_chain.data.len(){
			if (delta_chain.data[i] + 0.5).abs() < (delta + 0.5).abs(){
				index = i;
				delta = delta_chain.data[i];
			}
		}

		put_chain.data[index].clone()
	}

}

#[derive(Debug, Clone)]
pub struct OptionBoard<T>{
	pub data: Vec<OptionChain<T>>

}
