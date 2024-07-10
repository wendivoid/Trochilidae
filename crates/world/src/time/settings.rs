use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct TimeSettings {
    pub seconds_per_hour: u8,
    pub hours_per_day: u8,
    pub days_per_year: u16
}

impl Default for TimeSettings {
    fn default() -> TimeSettings {
        TimeSettings {
            seconds_per_hour: 1,
            hours_per_day: 10,
            days_per_year: 365
        }
    }
}