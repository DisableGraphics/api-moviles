use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserRegister {
	pub name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Money {
	pub user_id: u64,
	pub money: u64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ObjectiveRegister {
	pub user_id: u64,
	pub name: String,
	pub desc: String,
	pub cost: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Objective {
	pub id: u64,
	pub user_id: u64,
	pub name: String,
	pub desc: String,
	pub cost: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
	pub id: u64,
	pub name: String,
	pub money: u64
}
