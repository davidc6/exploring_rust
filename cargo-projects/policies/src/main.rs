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

#[derive(Debug)]
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

fn user_active_policies(active_policies: &Vec<Policy>) -> HashMap<&str, Vec<String>> {
    let mut active_policy_user_vehicles: HashMap<&str, Vec<String>> = HashMap::new();

    for active_policy in active_policies {
        let Policy {
            person_id,
            vehicle_id,
        } = active_policy;

        if let Some(vehicle) = active_policy_user_vehicles.get_mut(&person_id as &str) {
            vehicle.push(vehicle_id.clone())
        } else {
            active_policy_user_vehicles.insert(person_id, vec![vehicle_id.to_owned()]);
        }
    }

    active_policy_user_vehicles
}

fn find_potential_upsells(active_policies: Vec<Policy>) -> Vec<UpsellOpportunity> {
    let mut upsells: Vec<UpsellOpportunity> = vec![];
    let mut active_policy_user_vehicles = user_active_policies(&active_policies);

    let active_policies_chunk = active_policies.chunks(1);

    for active_policy_chunk in active_policies_chunk {
        // iterator over a slice, map to return person_id field only and collect into a Vector of string slices
        let user_ids: Vec<&str> = active_policy_chunk
            .iter()
            .map(|current_policy| current_policy.person_id.as_ref())
            .collect();

        let all_owned_vehicles = get_owned_vehicles(user_ids);

        for vehicle in all_owned_vehicles {
            let OwnedVehicle {
                person_id,
                vehicle_id,
            } = vehicle;

            let user_policies = active_policy_user_vehicles
                .get_mut(&person_id as &str)
                .unwrap();

            if !user_policies.contains(&vehicle_id) {
                upsells.push(UpsellOpportunity {
                    person_id,
                    vehicle_id: vehicle_id.clone(),
                });
                user_policies.push(vehicle_id);
            }
        }
    }

    upsells
}

fn main() {
    let active_policies = vec![
        Policy {
            person_id: "P1".to_owned(),
            vehicle_id: "V8".to_owned(),
        },
        Policy {
            person_id: "P2".to_owned(),
            vehicle_id: "V6".to_owned(),
        },
    ];

    let upsells = find_potential_upsells(active_policies);

    println!("{:?}", upsells);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{user_active_policies, Policy};

    #[test]
    fn create_user_active_policies() {
        let active_policies = vec![
            Policy {
                person_id: "P1".to_owned(),
                vehicle_id: "V8".to_owned(),
            },
            Policy {
                person_id: "P2".to_owned(),
                vehicle_id: "V6".to_owned(),
            },
        ];

        let policies = user_active_policies(&active_policies);

        let mut expected_policies = HashMap::new();
        expected_policies.insert("P1", vec!["V8".to_owned()]);
        expected_policies.insert("P2", vec!["V6".to_owned()]);

        assert_eq!(policies, expected_policies);
    }
}
