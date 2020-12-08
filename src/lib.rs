pub mod chain;

use std::collections::HashSet;
use crate::chain::AddressPool;
use std::fmt::{Debug, Formatter};
use std::fmt;

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Clone)]
pub struct Address {
    add: Vec<u8>,
    weight: u32,
}

impl Debug for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Address")
            .field("address", &hex::encode(self.add.clone()))
            .field("weight", &self.weight)
            .finish()
    }
}

pub struct Race;

impl Race {
    pub fn top<T: AddressPool>(n: u16, pool: &T) -> HashSet<Address> {
        let mut ret = HashSet::new();
        let mut pool = pool.get_pool();
        pool.reverse();
        for i in 0..n {
            ret.insert(pool[i as usize].clone());
        }
        ret
    }

    pub fn select<T: AddressPool>(n: u16, input: &T) -> HashSet<Address> {
        let seeds = input.get_seeds(n).unwrap();
        let mut pool = vec![];
        for address in input.get_pool().iter() {
            for _ in 0..address.weight {
                pool.push(address.clone());
            }
        }

        let mut selected: HashSet<Address> = HashSet::new();
        let pool_size = pool.len() as u32;
        for mut nonce in seeds {
            loop {
                let address = pool.get((nonce % pool_size) as usize).unwrap();
                if !selected.contains(address) {
                    selected.insert(address.clone());
                    break;
                }
                nonce += 1;
            }
        }
        selected
    }
}

#[test]
fn test_select() {
    let path = std::path::Path::new("/Users/fikgol/workspaces/stcmint-fight/starcoindb");
    let chain = BlockSnapshot::load_from_db(path, 1607090400, 1607392800).unwrap();
    let luckies = Race::select(2, &chain);
    let winners = Race::top(2, &chain);
    println!("{:?}", luckies);
    println!("{:?}", winners);
}
