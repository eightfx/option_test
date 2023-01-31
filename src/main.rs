mod structs;
mod greeks;
use crate::structs::*;
use crate::greeks::*;

fn main() {
	dbg!(greeks::greeks::delta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::Europian, &0.001));

}
