struct Policy {
    person_id: String,
    vehicle_id: String,
}

#[derive(Debug)]
struct OwnedVehicles {
    person_id: String,
    vehicle_id: String,
}

struct UpsellOpportunity {
    person_id: String,
    vehicle_id: String,
}

fn get_owned_vehicles(person_ids: &[Policy]) -> Vec<OwnedVehicles> {
    let owned_vehicles: Vec<OwnedVehicles> = vec![
        OwnedVehicles {
            person_id: "P1".to_owned(),
            vehicle_id: "V3".to_owned(),
        },
        OwnedVehicles {
            person_id: "P1".to_owned(),
            vehicle_id: "V8".to_owned(),
        },
        OwnedVehicles {
            person_id: "P2".to_owned(),
            vehicle_id: "V6".to_owned(),
        },
    ];

    owned_vehicles
}

fn find_potential_upsells(policies: Vec<Policy>) -> Vec<OwnedVehicles> {
    let iter = policies.chunks(1);

    let mut upsells: Vec<OwnedVehicles> = vec![];

    for item in iter {
        upsells.append(&mut get_owned_vehicles(item));
    }

    upsells
}

fn main() {
    let policies = vec![
        Policy {
            person_id: "P1".to_owned(),
            vehicle_id: "V8".to_owned(),
        },
        Policy {
            person_id: "P2".to_owned(),
            vehicle_id: "V6".to_owned(),
        },
    ];

    let upsells = find_potential_upsells(policies);

    println!("{:?}", upsells);
}
