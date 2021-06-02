use std::collections::HashMap;
use std::io;

// O(n) opposed to binary search O(nlog(n)) or loops O(2^n)
fn what_flavours(flavours: &Vec<u32>, money: &u32) -> [u32; 2] {
    let mut cost_to_flavour: HashMap<&u32, usize> = HashMap::new();
    for (flavour_id, cost) in flavours.iter().enumerate() {
        if cost_to_flavour.contains_key(&cost) && cost * 2 == *money {
            let mut same_cost: [u32; 2] = [
                *cost_to_flavour.get(&cost).unwrap() as u32,
                (flavour_id + 1) as u32,
            ];
            same_cost.sort();
            return same_cost;
        }

        cost_to_flavour.insert(cost, flavour_id + 1);
    }

    let mut sonny: u32 = 1;
    let mut johnny: u32 = money - sonny;
    let mut flavour_ids: [u32; 2] = [0; 2];

    while sonny <= money / 2 {
        let flavour_1 = cost_to_flavour.get(&sonny);
        let flavour_2 = cost_to_flavour.get(&johnny);

        if flavour_1.is_some() && flavour_2.is_some() {
            flavour_ids[0] = *flavour_1.unwrap() as u32;
            flavour_ids[1] = *flavour_2.unwrap() as u32;
            break;
        }

        sonny += 1;
        johnny -= 1;
    }
    flavour_ids.sort();
    flavour_ids
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_ice_cream_flavours_1() {
        let money: u32 = 4;
        let flavours: Vec<u32> = vec![1, 4, 5, 3, 2];
        let actual: [u32; 2] = what_flavours(&flavours, &money);
        assert_eq!(actual, [1, 4]);
    }

    #[test]
    fn should_return_ice_cream_flavours_2() {
        let money: u32 = 4;
        let flavours: Vec<u32> = vec![2, 2, 4, 3];
        let actual: [u32; 2] = what_flavours(&flavours, &money);
        assert_eq!(actual, [1, 2]);
    }

    #[test]
    fn should_return_ice_cream_flavours_3() {
        let money: u32 = 5;
        let flavours: Vec<u32> = vec![2, 1, 3, 5, 6];
        let actual: [u32; 2] = what_flavours(&flavours, &money);
        assert_eq!(actual, [1, 3]);
    }

    #[test]
    fn should_return_ice_cream_flavours_4() {
        let money: u32 = 542;
        let flavours: Vec<u32> = vec![863, 221, 2, 863, 321];
        let actual: [u32; 2] = what_flavours(&flavours, &money);
        assert_eq!(actual, [2, 5]);
    }
}
