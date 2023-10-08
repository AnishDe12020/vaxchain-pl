pub const VAX_TOKEN_MINT: &str = "2JU4847ngmiGjuZ6m2pt3unq41GVt6WRw6wnJhVPe2oD";
pub const VAX_TOKEN_DECIMALS: u16 = 9;
pub const STAKE_PER_PIECE: u16 = 100;
pub const COST_PER_REFRIGERATION_UNIT: u16 = 10;

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
