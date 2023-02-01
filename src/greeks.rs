mod american_greeks;
mod european_greeks;
use crate::*;

#[warn(non_camel_case_types)]
pub struct greeks{}

macro_rules! greeks_trait {
	($($func_name:ident),*) => {
		pub trait Greeks{
			$(
				fn $func_name(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

			)*
		}
	};
}

macro_rules! greeks_impl {
	($($func_name:ident),*) => {
		impl Greeks for greeks{
			$(
				fn $func_name(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
					match option_style{
						OptionStyle::European => <greeks as european_greeks::EuropeanGreeks>::$func_name(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type,  risk_free_rate, dividend_yield),
						OptionStyle::American => <greeks as american_greeks::AmericanGreeks>::$func_name(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type,  risk_free_rate, dividend_yield),
					}
				}

			)*
		}
	};
}

greeks_trait!(delta, gamma, theta, rho, vega, epsilon, vanna, charm, vomma, veta, speed, zomma, color, ultima, dual_delta, dual_gamma);
greeks_impl!(delta, gamma, theta, rho, vega, epsilon, vanna, charm, vomma, veta, speed, zomma, color, ultima, dual_delta, dual_gamma);

