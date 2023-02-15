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

// #[derive(FieldNamesAsArray)]
// pub struct greeks_list {
// 	pub delta:FloatType,
// 	pub gamma:FloatType,
// 	pub theta:FloatType,
// 	pub vega:FloatType,
// 	pub rho:FloatType,
// 	pub epsilon:FloatType,
// 	pub vanna:FloatType,
// 	pub charm:FloatType,
// 	pub vomma:FloatType,
// 	pub veta:FloatType,
// 	pub speed:FloatType,
// 	pub zomma:FloatType,
// 	pub color:FloatType,
// 	pub ultima:FloatType,
// 	pub dual_delta:FloatType,
// 	pub dual_gamma:FloatType,
// }
// macro_rules! greeks_list_trait{
// 	($pub:vis struct $name:ident {
// 		$($fpub:vis $field:ident: $type:ty,)*
// 	}) => {
// 		pub trait Greeks{
// 			$(
// 				fn $field(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;
// 			)*
// 		}

// 	}
// }

// greeks_list_trait!(greeks_list);
greeks_trait!(delta, gamma, theta, rho, vega, epsilon, vanna, charm, vomma, veta, speed, zomma, color, ultima, dual_delta, dual_gamma);
greeks_impl!(delta, gamma, theta, rho, vega, epsilon, vanna, charm, vomma, veta, speed, zomma, color, ultima, dual_delta, dual_gamma);

