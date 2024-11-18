use parking_lot::RwLock;
use std::fs::File;
use std::sync::Arc;
use std::collections::HashMap;
use std::io::Write;
use std::error::Error;
use crate::helper::read_lines;

const OBJ_FILE: &str = "objectives.txt";
const USER_FILE: &str = "user.txt";

// ID, NAME, MONEY
pub type UserType = HashMap<u64, (String, u64)>;
// ID, user id, name, description, cost
pub type ObjectiveType = HashMap<u64, (u64, String, String, u64)>;

#[derive(Clone)]
pub struct Store {
	pub user_list: Arc<RwLock<UserType>>,
	pub objective_list: Arc<RwLock<ObjectiveType>>,
}

impl Store {
    pub fn new() -> Result<Self, Box<dyn Error>> {
		let user_list = Self::load_users().unwrap_or(HashMap::new());
		let objectives_list = Self::load_objectives().unwrap_or(HashMap::new());
        Ok(Self {
            user_list: Arc::new(RwLock::new(user_list)),
			objective_list: Arc::new(RwLock::new(objectives_list)),
        })
    }
	pub fn save_users(&self) -> Result<(), Box<dyn Error>> {
		let users = self.user_list.read();
		let mut file = File::create(USER_FILE)?;
		for user in users.iter() {
			let bytes: Vec<u8> = format!("{}·{}·{}\n", user.0, user.1.0, user.1.1).bytes().collect();
			file.write(&bytes)?;
		}
		Ok(())
	}
	fn load_users() -> Result<UserType, Box<dyn Error>>{
		let lines = read_lines(USER_FILE)?;
		let mut ret = HashMap::new();
		for line in lines {
			if let Ok(line) = line {
				let parts: Vec<&str> = line.split('·').collect();
				let id: u64 = parts[0].parse()?;
				let name: String = parts[1].to_owned();
				let money: u64 = parts[2].parse()?;
				ret.insert(id, (name, money));
			}
		}
		Ok(ret)
	}

	pub fn save_objectives(&self) -> Result<(), Box<dyn Error>> {
		let users = self.objective_list.read();
		let mut file = File::create(OBJ_FILE)?;
		for objective in users.iter() {
			let bytes: Vec<u8> = format!("{}·{}·{}·{}·{}\n", objective.0, objective.1.0, objective.1.1,
				objective.1.2, objective.1.3).bytes().collect();
			file.write(&bytes)?;
		}
		Ok(())
	}
	fn load_objectives() -> Result<ObjectiveType, Box<dyn Error>>{
		let lines = read_lines(OBJ_FILE)?;
		let mut ret = HashMap::new();
		for line in lines {
			if let Ok(line) = line {
				let parts: Vec<&str> = line.split('·').collect();
				let id: u64 = parts[0].parse()?;
				let user_id = parts[1].parse()?;
				let name: String = parts[2].to_owned();
				let desc: String = parts[3].to_owned();
				let cost: u64 = parts[4].parse()?;
				ret.insert(id, (user_id, name, desc, cost));
			}
		}
		Ok(ret)
	}
}