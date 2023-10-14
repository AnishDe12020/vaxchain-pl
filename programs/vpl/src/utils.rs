use crate::constants::COST_PER_REFRIGERATION_UNIT;

pub fn calculate_refrigeration_cost(days: u16, temp: u16) -> u16 {
    if temp > 283 {
        return days * COST_PER_REFRIGERATION_UNIT * 1;
    } else if temp > 273 {
        return days * COST_PER_REFRIGERATION_UNIT * 10;
    } else if temp > 263 {
        return days * COST_PER_REFRIGERATION_UNIT * 100;
    } else {
        return days * COST_PER_REFRIGERATION_UNIT * 1000;
    }
}

pub fn calculate_days(start_date: i64, end_date: i64) -> u8 {
    let days = (((end_date - start_date) / 86400) as f64).ceil() as u8;

    if days < 1 {
        return 1;
    } else {
        return days;
    }
}
