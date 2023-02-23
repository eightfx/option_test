//! This module provides a convenient way to handle time-series data in Rust.
//! It allows the user to create and manipulate time-series data and perform arithmetic operations on them.
//! # Getting Started
//! import the module in your Rust code using:
//! ```rust
//! use time_series::TimeSeries;
//! ```
//! # Creating a new TimeSeries
//! To create a new TimeSeries, use the new() method:
//! ```rust
//! let mut ts: TimeSeries<f64> = TimeSeries::new();
//! ```
//! This creates a new TimeSeries that can hold floating-point numbers.
//! You can add elements to the TimeSeries using the push() method:
//! ```rust
//! ts.push(1.0);
//! ```
//! # Arithmetic operations
//! The TimeSeries type defines arithmetic operations for types that implement the Add, Sub, Mul, and Div traits. The following operations are available:
//!
//! - TimeSeries\<T\> @ TimeSeries\<T\>
//! - 	TimeSeries\<T\> @ &TimeSeries\<T\>
//! - &TimeSeries\<T\> @ TimeSeries\<T\>
//! - &TimeSeries\<T\> @ &TimeSeries\<T\>
//! However, @ refers to the four arithmetic operations +, -, *, /.
//! For example, to add two TimeSeries, use the + operator:
//! ```rust
//! let ts1: TimeSeries<f64> = TimeSeries::new();
//! let ts2: TimeSeries<f64> = TimeSeries::new();
//! let ts3 = &ts1 + &ts2;
//! let ts4 = ts1 + ts2;
//! ```
//! ### Attention.
//! I don't know why, but it seems that an error is detected by rust-analyzer regarding TimeSeries\<T\> @ TimeSeries\<T\>. It actually works, but may be a bit of a hindrance when coding.
//! # Mapping
//! You can apply a function to each element of a TimeSeries using the map() method. For example:
//! ```rust
//! let mut ts:TimeSeries<f64> = TimeSeries::new();
//! ts.push(1.);
//! ts.push(2.);
//! ts.push(3.);
//! let ts2 = ts.map(|x| x * 2.0);
//! dbg!(ts2);
//! ```
//! We can do more complicated things.
//! The map() function allows you to perform calculations on TimeSeries data by applying a function to each element in the series. This can be useful if you want to calculate a specific metric from your data, such as the Greeks from options data.
//!
//! If you have a function that takes a value of type T and returns a value of type U, you can use the map() function to apply that function to each element in a TimeSeries\<T\> and return a new TimeSeries\<U\>.
//! For example, suppose you have a TimeSeries of OptionChain\<OptionTick\> data and you want to calculate the Vega value for each option tick. You can use the map() function to first extract the ATM option ticks from each OptionChain, then calculate the implied volatility for each ATM option tick, and finally calculate the Vega for each option tick. Here is an example of how to do this:
//! ```rust
//! let mut ts: TimeSeries<OptionChain<OptionTick>> = TimeSeries::new();
//! ts.push(oc.clone());
//! ts.push(oc.clone());
//! ts.push(oc.clone());
//!
//! // Extract the ATM option tick from each OptionChain and calculate the Vega value
//! let vega_ts = ts.map(OptionChain::atm)
//!     .map(OptionTick::get_implied_volatility)
//!     .map(OptionTick::vega);
//! ```
//! Similarly, you can extract the 25delta call and put option ticks, calculate their implied volatility values, and then calculate the difference to obtain the put-call parity value. Here is an example of how to do this:
//! ```rust
//! // Extract the 25delta call and put option ticks and calculate their implied volatility values
//! let call_25delta_iv = ts.map(OptionChain::call_25delta)
//!     .map(OptionTick::get_implied_volatility)
//!     .map(OptionTick::iv);
//! let put_25delta_iv = ts.map(OptionChain::put_25delta)
//!     .map(OptionTick::get_implied_volatility)
//!     .map(OptionTick::iv);
//!
//! // Calculate the difference between the 25delta call and put implied volatility values to obtain put-call parity
//! let delta_iv_ts = &call_25delta_iv - &put_25delta_iv;
//! ```
//! In the above code, call_25delta_iv and put_25delta_iv are TimeSeries\<f64\> that contain the implied volatility values of the 25delta call and put option ticks, respectively. The delta_iv_ts is a TimeSeries\<f64\> that contains the put-call parity values.



use std::ops::*;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct TimeSeries<T>(pub Vec<T>);


impl<T> TimeSeries<T>
// where T:Clone
{
	pub fn new() -> TimeSeries<T>{
		TimeSeries(Vec::new())
	}
	pub fn push(&mut self, value: T){
		self.0.push(value);
	}

	/// Given a function f: T \-\> U that converts data to indicator, give a function map: TimeSeries\<T\> \-\> TimeSeries\<U\> that converts time series data to time series indices
	pub fn map<U>(&self, f : impl Fn(&T) -> U) -> TimeSeries<U>{
		TimeSeries(self.0.iter().map(f).collect())
	}
	
}

impl<T> TimeSeries<Result<T>>{
	pub fn unwrap(self) -> TimeSeries<T>{
		TimeSeries(self.0.into_iter().map(|x| x.unwrap()).collect())
	}
	
}

#[auto_impl_ops::auto_ops]
impl<T> Add<&TimeSeries<T>> for TimeSeries<T>
where for<'a> &'a T:Add<Output=T>
{
	type Output = TimeSeries<T>;
	fn add(self, other:&Self) -> Self::Output{
		TimeSeries(self.0.iter().zip(other.0.iter()).map(|(a,b)| a+b).collect())
	}

}

#[auto_impl_ops::auto_ops]
impl<T> Sub<&TimeSeries<T>> for TimeSeries<T>
where for<'a> &'a T:Sub<Output=T>
{
	type Output = TimeSeries<T>;
	fn sub(self, other:&Self) -> Self::Output{
		TimeSeries(self.0.iter().zip(other.0.iter()).map(|(a,b)| a-b).collect())
	}
	
}
#[auto_impl_ops::auto_ops]
impl<T> Mul<&TimeSeries<T>> for TimeSeries<T>
where for<'a> &'a T:Mul<Output=T>
{
	type Output = TimeSeries<T>;
	fn mul(self, other:&Self) -> Self::Output{
		TimeSeries(self.0.iter().zip(other.0.iter()).map(|(a,b)| a*b).collect())
	}

}

#[auto_impl_ops::auto_ops]
impl<T> Div<&TimeSeries<T>> for TimeSeries<T>
where for<'a> &'a T:Div<Output=T>
{
	type Output = TimeSeries<T>;
	fn div(self, other:&Self) -> Self::Output{
		TimeSeries(self.0.iter().zip(other.0.iter()).map(|(a,b)| a/b).collect())
	}

}




