use crate::StorageData;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use crate::Player;
use core::slice::IterMut;

// Custom serializer for `[u64; 4]` as an array of strings.
pub fn bigint_array_serializer<S>(array: &Vec<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(array.len()))?;
    for &element in array {
        seq.serialize_element(&element.to_string())?;
    }
    seq.end()
}

#[derive(Clone, Serialize)]
pub struct PlayerData {
    #[serde(serialize_with = "bigint_array_serializer")]
    pub inventory: Vec<u64>,
    pub balance: u64,
    pub placed: u64,
    pub previous: u64,
    pub power: u64,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            inventory: vec![],
            balance: 0,
            placed: 0,
            previous: 0,
            power: 0,
        }
    }
}

impl StorageData for PlayerData {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let objects_size = *u64data.next().unwrap();
        let mut inventory = Vec::with_capacity(objects_size as usize);
        for _ in 0..objects_size {
            inventory.push(*u64data.next().unwrap());
        }
        PlayerData {
            inventory,
            balance: (*u64data.next().unwrap()),
            placed: (*u64data.next().unwrap()),
            previous: (*u64data.next().unwrap()),
            power: (*u64data.next().unwrap())
        }
    }
    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.inventory.len() as u64);
        for c in self.inventory.iter() {
            data.push(*c as u64);
        }
        data.push(self.balance);
        data.push(self.placed);
        data.push(self.previous);
        data.push(self.power);
    }
}

pub trait Settle {
    fn settle_rewards(&mut self, rewards: u64);
}


pub type CombatPlayer = Player<PlayerData>;

impl Settle for CombatPlayer {
    fn settle_rewards(&mut self, rewards: u64) {
        self.data.balance += rewards;
        self.store();
    }
}
