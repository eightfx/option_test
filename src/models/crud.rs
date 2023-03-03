use super::extract_common_info::*;
use super::structs::{FloatType, OptionBase, OptionBoard, OptionChain, OptionTick, StrikeBoard};

/// This trait automatically builds OptionChain, OptionBoard, StrikeBoard, etc. by simply entering an OptionTick.
pub trait CRUD {
    type DataType;
    fn new() -> Self;
    fn upsert(&mut self, tick: OptionTick);
    fn delete(&mut self, tick: OptionTick);
    fn push(&mut self, data: Self::DataType);
}

impl CRUD for StrikeBoard {
    type DataType = OptionTick;
    fn new() -> Self {
        Self(Vec::new())
    }
    fn upsert(&mut self, tick: OptionTick) {
        if tick.get_value() < FloatType::EPSILON {
            return self.delete(tick);
        }
        let mut ticks = self.0.clone();
        let mut index = 0;
        let mut found = false;
        for (i, t) in ticks.iter().enumerate() {
            if t.option_value == tick.option_value && t.side == tick.side {
                index = i;
                found = true;
                break;
            }
        }
        if found {
            ticks[index] = tick;
        } else {
            ticks.push(tick);
        }
        self.0 = ticks;
    }

    fn delete(&mut self, tick: OptionTick) {
        let mut ticks = self.0.clone();
        let mut index = 0;
        let mut found = false;
        for (i, t) in ticks.iter().enumerate() {
            if t.option_value == tick.option_value && t.side == tick.side {
                index = i;
                found = true;
                break;
            }
        }
        if found {
            ticks.remove(index);
        }
        self.0 = ticks;
    }

    fn push(&mut self, data: OptionTick) {
        self.0.push(data);
    }
}

impl CRUD for OptionChain<OptionTick> {
    type DataType = OptionTick;
    fn new() -> Self {
        Self(Vec::new())
    }
    fn upsert(&mut self, tick: OptionTick) {
        if tick.get_value() < FloatType::EPSILON {
            return self.delete(tick);
        }
        let mut ticks = self.0.clone();
        let mut index = 0;
        let mut found = false;
        for (i, t) in ticks.iter().enumerate() {
            if t.strike == tick.strike && t.option_type == tick.option_type {
                index = i;
                found = true;
                break;
            }
        }
        if found {
            ticks[index] = tick;
        } else {
            ticks.push(tick);
        }
        self.0 = ticks;
    }

    fn delete(&mut self, tick: OptionTick) {
        let mut ticks = self.0.clone();
        let mut index = 0;
        let mut found = false;
        for (i, t) in ticks.iter().enumerate() {
			if t.strike == tick.strike && t.option_type == tick.option_type {
				index = i;
				found = true;
                break;
            }
        }
        if found {
            ticks.remove(index);
        }
        self.0 = ticks;
    }

    fn push(&mut self, data: OptionTick) {
        self.0.push(data);
    }
}

impl CRUD for OptionChain<StrikeBoard> {
    type DataType = StrikeBoard;
    fn new() -> Self {
        Self(Vec::new())
    }
    fn upsert(&mut self, tick: OptionTick) {
        let mut strike_boards = self.0.clone();
        let mut index = 0;
        let mut found = false;
        for (i, sb) in strike_boards.iter().enumerate() {
            if sb.strike().unwrap() == tick.strike && sb.option_type().unwrap() == tick.option_type
            {
                index = i;
                found = true;
                break;
            }
        }
        if found {
            strike_boards[index].upsert(tick);
        } else {
            let mut sb = StrikeBoard::new();
            sb.push(tick);
            strike_boards.push(sb);
        }
        self.0 = strike_boards;
    }
    fn delete(&mut self, tick: OptionTick) {
        let mut strike_boards = self.0.clone();
        let mut index = 0;
        let mut found = false;
        for (i, sb) in strike_boards.iter().enumerate() {
            if sb.strike().unwrap() == tick.strike && sb.option_type().unwrap() == tick.option_type
            {
                index = i;
                found = true;
                break;
            }
        }
        if found {
            strike_boards[index].delete(tick);
            if strike_boards[index].0.is_empty() {
                strike_boards.remove(index);
            }
        }
        self.0 = strike_boards;
    }
    fn push(&mut self, data: Self::DataType) {
        self.0.push(data);
    }
}
impl<T> CRUD for OptionBoard<T>
where
    T: OptionBase + ExtractCommonInfo,
    OptionChain<T>: CRUD,
{
    type DataType = OptionChain<T>;
    fn new() -> Self {
        Self(Vec::new())
    }
    fn upsert(&mut self, tick: OptionTick) {
        let mut option_chains = self.0.clone();
        let mut index = 0;
        let mut found = false;
        for (i, oc) in option_chains.iter().enumerate() {
            if oc.maturity().unwrap() == tick.maturity {
                index = i;
                found = true;
                break;
            }
        }
        if found {
            option_chains[index].upsert(tick);
        } else {
            let mut oc = OptionChain::new();
            oc.upsert(tick);
            option_chains.push(oc);
        }
        self.0 = option_chains;
    }

    fn delete(&mut self, tick: OptionTick) {
        let mut option_chains = self.0.clone();
        let mut index = 0;
        let mut found = false;
        for (i, oc) in option_chains.iter().enumerate() {
            if oc.maturity().unwrap() == tick.maturity {
                index = i;
                found = true;
                break;
            }
        }
        if found {
            option_chains[index].delete(tick);
            if option_chains[index].0.is_empty() {
                option_chains.remove(index);
            }
        }
        self.0 = option_chains;
    }
    fn push(&mut self, data: Self::DataType) {
        self.0.push(data);
    }
}
