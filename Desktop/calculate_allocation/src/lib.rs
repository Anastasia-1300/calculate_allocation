use std::collections::HashMap;
pub struct PositionInput {
    pub asset_type: &'static str ,
    pub quantity: f64,
    pub price: f64,
}
pub fn calculate_allocation(positions: &[PositionInput]) -> Result<HashMap<&'static str, f64>, String> {
    let mut total_share = 0.0;
    let mut total_bond = 0.0;
    let mut total_etf = 0.0;
    let mut total_currency = 0.0;
    let mut total_other = 0.0;
    let mut p = 0.0;
    for i in positions {
        let value = i.quantity * i.price;
        p += value;
    match i.asset_type {
        "share" => total_share += value,
        "bond" => total_bond += value,
        "etf" => total_etf += value,
        "currency" => total_currency += value,
        _ => total_other += value,
    }
}
    if p <= 1e-9 {
        return Err("No positions".to_string());
    }
    let mut allocation = HashMap::new();
    allocation.insert("share", total_share / p);
    allocation.insert("bond", total_bond / p);
    allocation.insert("etf", total_etf / p);
    allocation.insert("currency", total_currency / p);
    allocation.insert("other", total_other / p);
    Ok(allocation)
}
  //fn main() {
//         let positions = vec![
//             PositionInput {
//             asset_type: "share".to_string(),
//             quantity: 2.0,
//             price: 321.93,
//         },
//         PositionInput {
//             asset_type: "bond".to_string(),
//             quantity: 4.0,
//             price: 671.93,
//         },
//         PositionInput {
//             asset_type: "etf".to_string(),
//             quantity: 1.0,
//             price: 2764.0,
//         },
//         PositionInput {
//             asset_type: "currency".to_string(),
//             quantity: 0.0,
//             price: 0.0,
//         },
//         PositionInput {
//             asset_type: "other".to_string(),
//             quantity: 57.0,
//             price: 17.9,
//         },
//         ];
//     match calculate_allocation(&positions) {
//         Ok(allocation) => {
//             println!("{:?}", allocation);
//         },
//          Err(_) => todo!(),
//     }  
// }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_all() {
        let positions = vec![
            PositionInput {
            asset_type: "share",
            quantity: 2.0,
            price: 321.93,
        },
        PositionInput {
            asset_type: "bond",
            quantity: 4.0,
            price: 671.93,
        },
        PositionInput {
            asset_type: "etf",
            quantity: 1.0,
            price: 2764.0,
        },
        PositionInput {
            asset_type: "currency",
            quantity: 12.0,
            price: 56.0,
        },
        PositionInput {
            asset_type: "other",
            quantity: 57.0,
            price: 17.9,
        },
        ];
        let result = calculate_allocation(&positions);
        assert!((result.unwrap().values().sum::<f64>() - 1.0).abs() < 1e-6);
    }
     #[test]
    fn test_zero() {
        let zero = vec![
            PositionInput {
            asset_type: "share",
            quantity: 0.0,
            price: 0.0,
        },
        PositionInput {
            asset_type: "bond",
            quantity: 0.0,
            price: 0.0,
        },
        PositionInput {
            asset_type: "etf",
            quantity: 0.0,
            price: 0.0,
        },
        PositionInput {
            asset_type: "currency",
            quantity: 0.0,
            price: 0.0,
        },
        PositionInput {
            asset_type: "other",
            quantity: 0.0,
            price: 0.0,
        },
        ];
        let result = calculate_allocation(&zero);
        assert!(result.is_err());
    }
}
