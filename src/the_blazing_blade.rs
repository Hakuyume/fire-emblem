pub use crate::gba::Rng;

stats!(
    Stats,
    hp: "HP",
    str_mag: "力/魔力" ,
    skill: "技",
    spd: "速さ",
    luck: "幸運",
    def: "守備" ,
    res: "魔防",
);

pub struct Unit {
    pub name: &'static str,
    pub growth_rate: Stats<u8>,
}

pub mod units {
    use super::{Stats, Unit};

    pub const REBECCA: Unit = Unit {
        name: "レベッカ",
        growth_rate: Stats {
            hp: 60,
            str_mag: 40,
            skill: 50,
            spd: 60,
            luck: 50,
            def: 15,
            res: 30,
        },
    };
}
