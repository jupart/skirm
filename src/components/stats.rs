use specs::VecStorage;

#[derive(Component)]
#[component(VecStorage)]
pub struct StatsComp {
    pub health: u8,
    pub max_health: u8,
    pub strength: u8,
    pub aim: u8,
    pub move_per_turn: u8,
}

impl StatsComp {
    pub fn default() -> Self {
        Self {
            health: 100,
            max_health: 100,
            strength: 5,
            aim: 5,
            move_per_turn: 7,
        }
    }
}
