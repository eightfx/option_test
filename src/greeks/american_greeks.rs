use probability::prelude::*;
use crate::*;

pub trait AmericanGreeks{


	fn d1(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;
	fn d2(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;
	fn delta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;
	fn gamma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;
	fn theta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;
	fn rho(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;
	fn vega(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;
}
impl AmericanGreeks for greeks::greeks{
	fn d1(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		0.0
	}
	fn d2(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		0.0
	}

	fn delta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		0.0
	}
	fn gamma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		0.0
	}
	fn theta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		0.0
	}
	fn rho(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		0.0
	}
	fn vega(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		0.0
	}
}
