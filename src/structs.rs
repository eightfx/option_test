use typed_builder::TypedBuilder;

pub type FloatType = f64;

#[derive(Clone,Debug)]
pub enum OptionType{
	Put,
	Call
}

#[derive(Clone)]
pub enum OptionStyle{
	European,
	American
}

#[derive(Clone,Debug)]
pub enum OptionSide{
	Bid,
	Ask
}

#[derive(Clone,Debug)]
pub enum OptionValue{
	Price(FloatType),
	ImpliedVolatility(FloatType)
}

#[derive(Clone)]
pub struct AdditionalOptionData{
	pub open_interest: FloatType,
	pub volume: FloatType,
}

#[derive(Clone,Debug)]
#[derive(TypedBuilder)]
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
}


impl OptionTick{
	/// Normalize the difference between maturity and current time
	///T : 10-digit timestamp
	///t : 10-digit timestamp
	pub fn get_expiry(&self, t:&FloatType) -> FloatType{
		(self.expiry - t) / 31536000.0
	}


	/// Retrieve value from option_value
	/// 	Caution:
	/// Be careful when handling this function, as it extracts the value without distinguishing between premium and iv.
	pub fn get_value(&self) -> FloatType{
		match self.option_value{
			OptionValue::Price(p) => p,
			OptionValue::ImpliedVolatility(v) => v
		}
	}

}


#[derive(TypedBuilder,Clone)]
#[builder(field_defaults(default=FloatType::NAN, setter(!strip_option)))]
pub struct Option{
	#[builder(default=Vec::new())]
	pub data: Vec<OptionTick>,
	#[builder(default=OptionStyle::European)]
	pub option_style: OptionStyle,
	#[builder(default=0.001)]
	pub risk_free_rate: FloatType,
	#[builder(default=0.)]
	pub dividend_yield: FloatType,

	pub initial_price: FloatType,
}

impl Option{
	pub fn new(option_style: OptionStyle) -> Option{
		Option::builder().option_style(option_style).build()
	}
	pub fn push(&mut self, tick: OptionTick){
		self.data.push(tick);
	}
}


#[derive(Debug)]
pub struct TimeSeries<T>(pub Vec<T>);

	
impl<T> TimeSeries<T>
where T:Clone
{
	pub fn new() -> TimeSeries<T>{
		TimeSeries(Vec::new())
	}
	pub fn push(&mut self, value: T){
		self.0.push(value);
	}

	/// Given a function f: T -> U that converts data to indicator, give a function map: TimeSeries<T> -> TimeSeries<U> that converts time series data to time series indices
	pub fn map<U>(&self, f : impl Fn(&T) -> U) -> TimeSeries<U>{
		TimeSeries(self.0.iter().map(f).collect())
	}
	
}


