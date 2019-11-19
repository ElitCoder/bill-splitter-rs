use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let mut bills = HashMap::new();
    bills.insert("Niklas", 80f64);
    bills.insert("Hampus", 200f64);
    bills.insert("Magnus", 400f64);
    bills.insert("Martin", 0f64);

    let total_sum: f64 = bills.values().sum();
    let total_sum = total_sum / bills.len() as f64;
    bills.values_mut().for_each(|v| *v -= total_sum);

    let (owed, owing): (HashMap<&str, f64>, HashMap<&str, f64>) =
        bills.iter().partition(|(_, &v)| v >= 0.0);

    let mut owed = Vec::from_iter(owed);
    owed.sort_by(|(_, v1), (_, v2)| v2.partial_cmp(v1).expect("Can't sort"));
    let mut owing = Vec::from_iter(owing);
    owing.sort_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).expect("Can't sort"));

    for (owing_name, owing_amount) in &mut owing {
        for (owed_name, owed_amount) in &mut owed {
            if *owing_amount == 0.0 || *owed_amount == 0.0 {
                continue;
            }

            if *owed_amount <= -*owing_amount {
                *owing_amount += *owed_amount;
                println!("{} sends {} {}", owing_name, owed_name, owed_amount);
                *owed_amount = 0.0;
            } else {
                *owed_amount += *owing_amount;
                println!("{} sends {} {}", owing_name, owed_name, -*owing_amount);
                *owing_amount = 0.0;
            }
        }
    }
}
