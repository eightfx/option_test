use probability::prelude::*;

type FloatType = f64;
enum OptionType{
	Put,
	Call
}

enum OptionStyle{
	Europian,
	American
}
struct Tick{
	strike: FloatType,
	option_type: OptionType,
	expiry: FloatType,
	open_interest: FloatType,
	implied_volatility: FloatType,
}

struct Option{
	data: Vec<Tick>,
	option_style: OptionStyle,
	risk_free_rate: FloatType,
	initial_price: FloatType,
}

impl Option{
	fn new(option_style: OptionStyle) -> Option{
		Option{
			data: Vec::new(),
			option_style: option_style,
			risk_free_rate: 0.001,
			initial_price: 0.0,
		}
	}
	fn add_tick(&mut self, tick: Tick){
		self.data.push(tick);
	}
}

impl Tick{
	/// Normalize the difference between maturity and current time
	///T : 10-digit timestamp
	///t : 10-digit timestamp
	fn get_expiry(&self, t:FloatType) -> FloatType{
		(self.expiry - t) / 31536000.0
	}
}

trait EuropianGreeks{
	fn get_d(tick:&Tick, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType) -> (FloatType, FloatType);
	fn delta(tick:&Tick, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType) -> FloatType;
}
impl EuropianGreeks for Tick{
	fn get_d(tick:&Tick, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType) -> (FloatType, FloatType) {
		let t = tick.get_expiry(t);
		let d1:FloatType;
		let d2:FloatType;
		d1 = ((&initial_price / &tick.strike).log10() + (&risk_free_rate + 0.5 * &tick.implied_volatility * &tick.implied_volatility) * &t) / (&tick.implied_volatility * &t.sqrt()) ;

		d2 = d1 - &tick.implied_volatility * &t.sqrt();

		if d1.is_nan() || d2.is_nan(){
			(0.0, 0.0)
		}else{
			(d1, d2)
		}
	}
	
	fn delta(tick:&Tick, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType) -> FloatType {
		let (d1, _) = <Tick as EuropianGreeks>::get_d(tick, risk_free_rate, initial_price, t);
		let g = Gaussian::new(0.0, 1.0);
		g.distribution(d1)
	}
}
trait AmericanGreeks{

	fn get_d(tick:&Tick, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType) -> (FloatType, FloatType);
	fn delta(tick:&Tick, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType) -> FloatType;
}
impl AmericanGreeks for Tick{
	fn get_d(tick:&Tick, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType) -> (FloatType, FloatType) {
		(0.0, 0.0)
	}
	fn delta(tick:&Tick, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType) -> FloatType {
		0.0
	}
}
trait Greeks{
	fn get_d(&self, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType, option_style:OptionStyle) -> (FloatType, FloatType);
	fn delta(&self, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType, option_style:OptionStyle) -> FloatType;
	
}

impl Greeks for Tick{
	fn get_d(&self, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType, option_style:OptionStyle) -> (FloatType, FloatType) {
		match option_style{
			OptionStyle::Europian => <Tick as EuropianGreeks>::get_d(self, risk_free_rate, initial_price, t),
			OptionStyle::American => <Tick as AmericanGreeks>::get_d(self, risk_free_rate, initial_price, t),
		}
	}

	fn delta(&self, risk_free_rate:FloatType, initial_price:FloatType, t:FloatType, option_style:OptionStyle) -> FloatType {
		match option_style{
			OptionStyle::Europian => <Tick as EuropianGreeks>::delta(self, risk_free_rate, initial_price, t),
			OptionStyle::American => <Tick as AmericanGreeks>::delta(self, risk_free_rate, initial_price, t),
		}
}
}


fn main() {
	let tick = Tick{
		strike: 100.0,
		option_type: OptionType::Call,
		expiry: 1609459200.0,
		open_interest: 0.0,
		implied_volatility: 0.2,
	};
	dbg!(tick.delta(0.001, 100.0, 1609439200.0, OptionStyle::Europian));

}
