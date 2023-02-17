use typed_builder::TypedBuilder;

pub type FloatType = f64;
pub enum OptionType{
	Put,
	Call
}

pub enum OptionStyle{
	European,
	American
}


#[builder(field_defaults(default=FloatType::NAN, setter(!strip_option)))]
pub struct OptionTick{
	pub strike: FloatType,
	pub expiry: FloatType,
	pub open_interest: FloatType,
	pub implied_volatility: FloatType,
	pub premium: FloatType,
	pub initial_price: FloatType,
	#[builder(default=0.001)]
	pub risk_free_rate: FloatType,

	#[builder(default=OptionType::Call)]
	pub option_type: OptionType,
	#[builder(default=OptionStyle::European)]
	pub option_style: OptionStyle,
}

pub struct Option{
	pub data: Vec<Tick>,
	pub option_style: OptionStyle,
	pub risk_free_rate: FloatType,
	pub initial_price: FloatType,
}

impl Option{
	pub fn new(option_style: OptionStyle) -> Option{
		Option{
			data: Vec::new(),
			option_style: option_style,
			risk_free_rate: 0.001,
			initial_price: 0.0,
		}
	}
	pub fn add_tick(&mut self, tick: Tick){
		self.data.push(tick);
	}
}

impl Tick{
	/// Normalize the difference between maturity and current time
	///T : 10-digit timestamp
	///t : 10-digit timestamp
	pub fn get_expiry(&self, t:&FloatType) -> FloatType{
		(self.expiry - t) / 31536000.0
	}
}


