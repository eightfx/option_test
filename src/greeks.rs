mod american_greeks;
mod europian_greeks;
use crate::*;

pub struct greeks{}

macro_rules! greeks_trait {
	($($func_name:ident),*) => {
		pub trait Greeks{
			$(
				fn $func_name(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;

			)*
		}
	};
}

macro_rules! greeks_impl {
	($($func_name:ident),*) => {
		impl Greeks for greeks{
			$(
				fn $func_name(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
					match option_style{
						OptionStyle::Europian => <greeks as europian_greeks::EuropianGreeks>::$func_name(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, option_style, risk_free_rate),
						OptionStyle::American => <greeks as american_greeks::AmericanGreeks>::$func_name(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, option_style, risk_free_rate),
					}
				}

			)*
		}
	};
}

greeks_trait!(d1, d2, delta, gamma, theta, rho, vega);
greeks_impl!(d1,d2,delta, gamma, theta, rho, vega);

