use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Spot {
    pub name: String,
    pub id: String
}

fn generate_spot_list() -> HashMap<String, Spot> {
    let hardcoded = [
        ["Nahant", "5a1ef708aa1aea001b27be37"]
    ];

    let mut spots: HashMap<String, Spot> = HashMap::new();

    for spot in hardcoded {
        spots.insert(
            String::from(spot[0]),
            Spot{ name: String::from(spot[0]), id: String::from(spot[1]) }
        );
    };

    return spots;
}

pub fn get_spot(name: &str) -> Option<Spot> {
    let spot_map = generate_spot_list();

    let spot = spot_map.get(name);
    match spot {
        Some(s) => Some(s.clone()),
        None => None
    }
}