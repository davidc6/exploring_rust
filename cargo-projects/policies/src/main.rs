use std::{collections::HashMap, slice::Chunks};

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

#[derive(Debug, PartialEq)]
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

fn user_ids(active_policy_chunk: &[Policy]) -> Vec<&str> {
    // iterator over a slice, map to return person_id field only and collect into a Vector of string slices
    active_policy_chunk
        .iter()
        .map(|current_policy| current_policy.person_id.as_ref())
        .collect()
}

fn update_upsells(
    all_owned_vehicles: Vec<OwnedVehicle>,
    active_policy_user_vehicles: &mut HashMap<&str, Vec<String>>,
    upsells: &mut Vec<UpsellOpportunity>,
) {
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

fn find_potential_upsells(active_policies: Vec<Policy>) -> Vec<UpsellOpportunity> {
    let mut upsells: Vec<UpsellOpportunity> = vec![];
    let mut active_policy_user_vehicles = user_active_policies(&active_policies);
    let active_policies_chunks = active_policies.chunks(1);

    for active_policy_chunk in active_policies_chunks {
        let user_ids: Vec<&str> = user_ids(active_policy_chunk);
        let all_owned_vehicles = get_owned_vehicles(user_ids);

        update_upsells(
            all_owned_vehicles,
            &mut active_policy_user_vehicles,
            &mut upsells,
        )
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

    use crate::{
        find_potential_upsells, update_upsells, user_active_policies, user_ids, OwnedVehicle,
        Policy, UpsellOpportunity,
    };

    fn active_policies() -> Vec<Policy> {
        vec![
            Policy {
                person_id: "P1".to_owned(),
                vehicle_id: "V8".to_owned(),
            },
            Policy {
                person_id: "P2".to_owned(),
                vehicle_id: "V6".to_owned(),
            },
        ]
    }

    #[test]
    fn create_user_active_policies() {
        let active_policies = active_policies();

        let policies = user_active_policies(&active_policies);

        let expected_policies =
            HashMap::from([("P1", vec!["V8".to_owned()]), ("P2", vec!["V6".to_owned()])]);

        assert_eq!(policies, expected_policies);
    }

    #[test]
    fn create_user_ids() {
        let active_policies = active_policies();

        let expected_user_ids = vec!["P1", "P2"];
        let actual = user_ids(&active_policies);

        assert_eq!(actual, expected_user_ids);
    }

    #[test]
    fn update_upsells_inserts_new_upsell() {
        let vehicles = vec![
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
        ];

        let mut upsells: Vec<UpsellOpportunity> = vec![];
        let active_policies = active_policies();
        let mut user_active_policies = user_active_policies(&active_policies);

        update_upsells(vehicles, &mut user_active_policies, &mut upsells);
        assert!(upsells.len() == 2);
    }

    #[test]
    fn find_potential_upsells_finds_upsells() {
        let active_policies = active_policies();
        let actual_upsells = find_potential_upsells(active_policies);

        assert_eq!(
            actual_upsells,
            vec![
                UpsellOpportunity {
                    person_id: "P1".to_owned(),
                    vehicle_id: "V3".to_owned()
                },
                UpsellOpportunity {
                    person_id: "P2".to_owned(),
                    vehicle_id: "V11".to_owned()
                }
            ]
        )
    }
}
