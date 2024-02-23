pub struct PokieConfig {
    pub sleep_duration: u64
}

pub fn init_pokie_config() -> PokieConfig {
    return PokieConfig {
        sleep_duration: 10
    }
}