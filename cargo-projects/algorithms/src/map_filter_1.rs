use std::collections::HashMap;

macro_rules! define_struct_with_common_fields {
    // Pattern derives
    (derive($($derive:tt)+), $name:ident) => {
        #[derive($($derive)+)]
        struct $name {
            user_id: String,
            product_id: String,
        }
    };

    // Pattern no derives
    ($name:ident) => {
        struct $name {
            user_id: String,
            product_id: String,
        }
    };
}

define_struct_with_common_fields!(UserOwnedProduct);
define_struct_with_common_fields!(Insurance);
define_struct_with_common_fields!(derive(Debug, PartialEq), Offer);

fn owned_products(user_ids: Vec<String>) -> Vec<UserOwnedProduct> {
    let product = UserOwnedProduct {
        user_id: "Human1".to_owned(),
        product_id: "SKU3".to_owned(),
    };
    vec![product]
}

fn offers(insurance: Vec<Insurance>) -> Vec<Offer> {
    // Build a HashMap of User: []
    // then split into 100 each
    // --->

    let mut user_products: HashMap<String, Vec<String>> = HashMap::new();

    let mut chunks = vec![vec![]];
    let mut current_chunk: Vec<String> = vec![];
    let max = 1;

    for entry in insurance {
        let Insurance {
            user_id,
            product_id,
        } = entry;
        user_products
            .entry(user_id.clone())
            .or_default()
            .push(product_id);

        if current_chunk.len() == max {
            chunks.push(current_chunk);
            current_chunk = Vec::with_capacity(max);
        } else {
            current_chunk.push(user_id);
        }
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    // TODO: iterate over the

    for chnks in chunks {
        // slow function
        owned_products(chnks);
    }

    // assume that SKU3 is on offer too
    let offer = Offer {
        user_id: "Human1".to_owned(),
        product_id: "SKU1".to_owned(),
    };
    let offers = vec![offer];
    offers
}

#[cfg(test)]
mod offer_test {
    use super::{offers, Insurance};
    use crate::map_filter_1::Offer;

    #[test]
    fn map_filter_works() {
        // ---- Arrange
        let insurance1 = Insurance {
            user_id: "Human1".to_owned(),
            product_id: "SKU1".to_owned(),
        };
        let insurance2 = Insurance {
            user_id: "Human1".to_owned(),
            product_id: "SKU2".to_owned(),
        };

        // ---- Act
        let actual = offers(vec![insurance1, insurance2]);

        // ---- Assert
        let expected = vec![Offer {
            user_id: "Human1".to_owned(),
            product_id: "SKU3".to_owned(),
        }];
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn some_test() {}
}
