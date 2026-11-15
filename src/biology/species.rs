pub struct Species {
    pub name: String,

    pub optimal_temperature: f32,
    pub temperature_resistance: f32,

    pub optimal_moisture: f32,
    pub moisture_resistance: f32,

    pub base_growth_rate: f32,

    // to introduce:
    // - nutrients
    // - enzyme production
    // - health
    // - reproductive state
    // - genetic variation
}
