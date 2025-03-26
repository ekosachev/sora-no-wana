use bevy::prelude::{Query, ResMut};

use crate::game::components::{
    common::ID,
    galaxy_map::{GalaxyMap, StarData},
    spatial::Position,
    star::{CStarClass, CStarType, Star},
};

pub fn update_galaxy_map(
    mut map: ResMut<GalaxyMap>,
    query: Query<(&Star, &Position, &CStarType, &CStarClass, &ID)>,
) {
    let mut stars = map.stars.lock().unwrap();
    for (_, position, star_type, star_class, id) in query.iter() {
        stars.push(StarData {
            id: id.clone(),
            position: position.0,
            star_type: star_type.0,
            star_class: star_class.0,
        });
    }
}
