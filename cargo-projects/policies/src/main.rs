use std::collections::HashMap;

#[derive(Debug)]
struct Policy {
    person_id: String,
    vehicle_id: String,
}

#[derive(Debug)]
struct OwnedVehicle {
    person_id: String,
    vehicle_id: String,
}

struct UpsellOpportunity {
    person_id: String,
    vehicle_id: String,
}

fn get_owned_vehicles(person_ids: Vec<&str>) -> Vec<OwnedVehicle> {
    vec![
        OwnedVehicle {
            person_id: "P1".to_owned(),
            vehicle_id: "V3".to_owned(),
        },
        OwnedVehicle {
            person_id: "P1".to_owned(),
            vehicle_id: "V8".to_owned(),
        },
        OwnedVehicle {
            person_id: "P2".to_owned(),
            vehicle_id: "V6".to_owned(),
        },
        OwnedVehicle {
            person_id: "P2".to_owned(),
            vehicle_id: "V11".to_owned(),
        },
    ]
}

fn find_potential_upsells(policies: Vec<Policy>) -> Vec<OwnedVehicle> {
    let mut upsells: Vec<OwnedVehicle> = vec![];
    let mut user_vehicles: HashMap<&str, Vec<String>> = HashMap::new();

    // build a map of user ids and owned vehicles
    for policy in &policies {
        if let Some(vehicle) = user_vehicles.get_mut(&policy.person_id as &str) {
            vehicle.push(policy.vehicle_id.clone())
        } else {
            user_vehicles.insert(&policy.person_id, vec![policy.vehicle_id.clone()]);
        }
    }

    // chunk the vector
    let policy_chunk = policies.chunks(1);

    for policy in policy_chunk {
        // iterator over a slice, map to return person_id field only and collect into a Vector of string slices
        let user_ids: Vec<&str> = policy
            .iter()
            .map(|val| {
                // let id: &str = val.person_id.as_ref();
                // let owned_vehicles = user_vehicles.get_mut(id);

                // if let Some(vehicles) = owned_vehicles {
                //     vehicles.push(&val.vehicle_id);
                // } else {
                //     user_vehicles.insert(id, vec![&val.vehicle_id]);
                // }

                val.person_id.as_ref()
            })
            .collect();

        let all_owned_vehicles = get_owned_vehicles(user_ids);

        for vehicle in all_owned_vehicles {
            // let OwnedVehicle {
            //     person_id,
            //     vehicle_id,
            // } = vehicle;

            let user_policies = user_vehicles.get_mut(&vehicle.person_id as &str).unwrap();

            if !user_policies.contains(&vehicle.vehicle_id) {
                upsells.push(OwnedVehicle {
                    person_id: vehicle.person_id,
                    vehicle_id: vehicle.vehicle_id.clone(),
                });
                // let v_c = vehicle_id.clone();
                user_policies.push(vehicle.vehicle_id)
            }
        }
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
