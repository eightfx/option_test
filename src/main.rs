
enum OptionType{
	Put,
	Call
}

enum OptionStyle{
	Europian,
	American
}
struct Tick{
	strike: f64,
	option_type: OptionType,
	expiry: f64,
	price: f64,
	open_interest: f64,
}

struct Option{
	data: Vec<Tick>,
	option_style: OptionStyle,
}

impl Option{
	fn new(option_style: OptionStyle) -> Option{
		Option{
			data: Vec::new(),
			option_style: option_style,
		}
	}
	fn add_tick(&mut self, tick: Tick){
		self.data.push(tick);
	}
}

trait EuropianGreeks{
	fn delta(option:&Option) -> f64;
}
impl EuropianGreeks for Option{
	fn delta(option:&Option) -> f64{
		0.0
	}
}
trait AmericanGreeks{
	fn delta(option:&Option) -> f64;
}
impl AmericanGreeks for Option{
	fn delta(option:&Option) -> f64{
		1.0
	}
}
trait Greeks{
	fn delta(&self) -> f64;
}

impl Greeks for Option{
	fn delta(&self) -> f64 {
		match self.option_style{
			OptionStyle::Europian => <Option as EuropianGreeks>::delta(self),
			OptionStyle::American => <Option as AmericanGreeks>::delta(self),
		}
	}
}

fn main() {
	let mut option = Option::new(OptionStyle::American);
	option.add_tick(Tick{
		strike: 100.0,
		option_type: OptionType::Call,
		expiry: 1.0,
		price: 10.0,
		open_interest: 100.0,
	});

	dbg!(option.delta());
}
