use probability::prelude::*;
use crate::*;

#[cfg_attr(doc, katexit::katexit)]
/// This is the trait for calculating European Greeks.
pub trait EuropianGreeks{
	/// Returns the d1 
	/// # Formula
	/// $$
	/// 	d_1 = \frac{\log{(\frac{S_t}{T})} + (r - g + \frac{\sigma^2}{2})(T-t)}{\sigma \sqrt{T-t}}
	/// $$
	fn d1(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) ->FloatType;
	/// Returns the d1 
	/// # Formula
	/// $$
	/// 	d_2 = \frac{\log{(\frac{S_t}{T})} + (r - g - \frac{\sigma^2}{2})(T-t)}{\sigma \sqrt{T-t}}
	/// $$
	fn d2(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) ->FloatType;
	/// Returns the delta of the option
	/// # Formula
	/// $$
	/// 	\Delta_c = N(d_1)
	/// $$
	/// $$
	/// 	\Delta_p = N(d_1) - 1
	/// $$
	fn delta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;

	/// Returns the gamma of the option
	/// # Formula
	/// $$
	/// 	\Gamma = \frac{1}{\sigma S_t \sqrt{T-t}} \frac{1}{\sqrt{2\pi}} e^{-\frac{d_1^2}{2}}
	/// $$
	fn gamma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;

	/// Returns the theta of the option
	/// # Formula
	/// $$
	/// 	\Theta_c = -r K e^{-r(T-t)} N(d_2) - \frac{\sigma S_t}{2 \sqrt{T-t}} \frac{1}{\sqrt{2\pi}} e^{-\frac{d_1^2}{2}}
	/// $$
	/// $$
	/// 	\Theta_p = r K e^{-r(T-t)} (N(-d_2)) - \frac{\sigma S_t}{2 \sqrt{T-t}} \frac{1}{\sqrt{2\pi}} e^{-\frac{d_1^2}{2}}
	/// $$
	fn theta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;

	/// Returns the rho of the option
	/// # Formula
	/// $$
	/// 	\rho_c = (T-t) K e^{-r(T-t)} N(d_2)
	/// $$
	/// $$
	/// 	\rho_p = -(T-t) K e^{-r(T-t)} N(-d_2)
	/// $$
	fn rho(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;

	/// Returns the vega of the option
	/// # Formula
	/// $$
	/// 	\kappa = S_t \sqrt{T-t} \frac{1}{\sqrt{2\pi}} e^{-\frac{d_1^2}{2}}
	/// $$
	fn vega(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType;


	fn get_expiry(ts_expiration:&FloatType, ts_now:&FloatType) -> FloatType{
		(ts_expiration - ts_now) / 31536000.
	}
}

impl EuropianGreeks for greeks::greeks{
	fn d1(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {

		let t = Self::get_expiry(ts_expiration, ts_now);
		((asset_price / strike).log(std::f64::consts::E) + (risk_free_rate + 0.5 * implied_volatility * implied_volatility) * t) / (implied_volatility * t.sqrt()) 

	}
	fn d2(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {

		let t = Self::get_expiry(ts_expiration, ts_now);
		((asset_price / strike).log(std::f64::consts::E) + (risk_free_rate - 0.5 * implied_volatility * implied_volatility) * t) / (implied_volatility * t.sqrt())

	}
	
	fn delta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		
		let d1 = <Self as EuropianGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, option_style, risk_free_rate);
		let g = Gaussian::new(0.0, 1.0);
		match option_type{
			OptionType::Call => g.distribution(d1),
			OptionType::Put => g.distribution(d1) - 1.0,
		}
	}

	fn gamma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {

		let d1 = <Self as EuropianGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, option_style, risk_free_rate);
		let t = Self::get_expiry(ts_expiration, ts_now);

		(-0.5 * d1*d1).exp() / (implied_volatility * asset_price * t.sqrt() *  (2. * FloatType::from(std::f64::consts::PI)).sqrt())
	}

	fn theta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {

		let d1 = <Self as EuropianGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, option_style, risk_free_rate);
		let d2 = <Self as EuropianGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, option_style, risk_free_rate);

		let t = Self::get_expiry(ts_expiration, ts_now);
		let g = Gaussian::new(0.0, 1.0);
		match option_type{
			OptionType::Call =>
				- risk_free_rate * strike * (-risk_free_rate * t).exp() * g.distribution(d2) - implied_volatility * asset_price
				* (-0.5 * (d1 * d1)).exp() / (2. * t.sqrt() * ((2. * FloatType::from(std::f64::consts::PI)).sqrt())),
			OptionType::Put =>
				risk_free_rate * strike * (-risk_free_rate * t).exp() * (g.distribution(-d2)) - implied_volatility * asset_price
				* (-0.5 * (d1 * d1)).exp() / (2. * t.sqrt() * ((2. * FloatType::from(std::f64::consts::PI)).sqrt())),
		}

	}

	fn rho(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		let d2 = <Self as EuropianGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, option_style, risk_free_rate);


		let t = Self::get_expiry(ts_expiration, ts_now);

		let g = Gaussian::new(0.0, 1.0);
		match option_type{
			OptionType::Call => t*strike*(-risk_free_rate*t).exp()*g.distribution(d2),
			OptionType::Put => - t*strike*(-risk_free_rate*t).exp()*g.distribution(-d2)

		}

	}

	fn vega(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, option_style:&OptionStyle, risk_free_rate:&FloatType) -> FloatType {
		let d1 = <Self as EuropianGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, option_style, risk_free_rate);


		let t = Self::get_expiry(ts_expiration, ts_now);
		asset_price * t.sqrt() * (-0.5 * (d1 * d1)).exp() / ((2. * FloatType::from(std::f64::consts::PI)).sqrt())
	}
	
}


#[cfg(test)]
mod tests{
	use crate::greeks::*;

	#[test]
	fn greeks_call(){
	
		assert_float_relative_eq!(greeks::delta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::Europian, &0.001), 0.867, 0.01);
		assert_float_relative_eq!(greeks::gamma(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::Europian, &0.001), 0.0007483, 0.0001);
		assert_float_relative_eq!(greeks::theta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::Europian, &0.001), -374.163, 0.01);
		assert_float_relative_eq!(greeks::rho(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::Europian, &0.001),0.818, 0.01);
		assert_float_relative_eq!(greeks::vega(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::Europian, &0.001),6.151, 0.01);
	}

	#[test]
	fn greeks_put(){
		assert_float_relative_eq!(greeks::delta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::Europian, &0.001), -0.132666, 0.01);
		assert_float_relative_eq!(greeks::gamma(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::Europian, &0.001), 0.0007483, 0.0001);
		assert_float_relative_eq!(greeks::theta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::Europian, &0.001), -373.9, 0.01);
		assert_float_relative_eq!(greeks::rho(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::Europian, &0.001), -19.7285, 0.01);
		assert_float_relative_eq!(greeks::vega(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::Europian, &0.001),6.151, 0.01);
		
	}
	
} 
