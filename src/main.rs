use warp::{http, Filter};
use parking_lot::RwLock;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::{collections::HashMap, error::Error};
use std::sync::Arc;
use serde::{Serialize, Deserialize};

// ID, NAME, MONEY
type UserType = HashMap<u64, (String, u64)>;
// ID, user id, name, description, cost
type ObjectiveType = HashMap<u64, (u64, String, String, u64)>;

const OBJ_FILE: &str = "objectives.txt";
const USER_FILE: &str = "user.txt";

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserRegister {
	name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Money {
	user_id: u64,
	money: u64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ObjectiveRegister {
	user_id: u64,
	name: String,
	desc: String,
	cost: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Objective {
	id: u64,
	user_id: u64,
	name: String,
	desc: String,
	cost: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct User {
	id: u64,
	name: String,
	money: u64
}

#[derive(Clone)]
struct Store {
	user_list: Arc<RwLock<UserType>>,
	objective_list: Arc<RwLock<ObjectiveType>>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Store {
    fn new() -> Result<Self, Box<dyn Error>> {
		let user_list = Self::load_users().unwrap_or(HashMap::new());
		let objectives_list = Self::load_objectives().unwrap_or(HashMap::new());
        Ok(Self {
            user_list: Arc::new(RwLock::new(user_list)),
			objective_list: Arc::new(RwLock::new(objectives_list)),
        })
    }
	fn save_users(&self) -> Result<(), Box<dyn Error>> {
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

	fn save_objectives(&self) -> Result<(), Box<dyn Error>> {
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

async fn create_user(
	user: UserRegister,
	store: Store) -> Result<impl warp::Reply, warp::Rejection>
{
	let id = store.user_list.read().len();
	store.user_list.write().insert(id as u64, (user.name, 0));
	let _ = store.save_users();
	Ok(warp::reply::with_status(
		format!("{}", id),
		http::StatusCode::CREATED,
	))
}

async fn add_objective(
	obj: ObjectiveRegister,
	store: Store) -> Result<impl warp::Reply, warp::Rejection>
{
	let id = store.objective_list.read().len();
	let user_exists = store.user_list.read().contains_key(&obj.user_id);

	if user_exists {
		store.objective_list.write().insert(id as u64, (obj.user_id, obj.name, obj.desc, obj.cost));
		let _ = store.save_objectives();
		Ok(warp::reply::with_status(
			format!("{}", id),
			http::StatusCode::CREATED,
		))
	} else {
		Ok(warp::reply::with_status(
			"User does not exist".to_string(), 
		http::StatusCode::NOT_FOUND))
	}
}

async fn get_objectives(
	obj: u64,
	store: Store
	)  -> Result<impl warp::Reply, warp::Rejection>
{
	let read = store.objective_list.read();
	let mut ret = Vec::new();
	for i in read.iter() {
		if i.1.0 == obj { 
			let obj = Objective{
				id: *i.0,
				user_id: i.1.0,
				name: i.1.1.clone(),
				desc: i.1.2.clone(),
				cost: i.1.3
			};
			ret.push(obj);
		}
	}

	Ok(warp::reply::json(
		&ret
	))
}

async fn remove_objective(
	obj: u64,
	store: Store
	)  -> Result<impl warp::Reply, warp::Rejection> {
	{
		let mut write = store.objective_list.write();
		write.remove(&obj);
	}
	let _ = store.save_objectives();
	Ok(warp::reply())
}

async fn set_money(
	money: Money,
	store: Store
	)  -> Result<impl warp::Reply, warp::Rejection> {
	{
		let mut write = store.user_list.write();
		let user = {
			write.get(&money.user_id).and_then(|e|{Some(e.to_owned())})
		};
		if let Some(user) = user {
			write.insert(money.user_id, (user.0.clone(), money.money));
		}
	}
	let _ = store.save_users();
	Ok(warp::reply())
}

fn post_json_user_register() -> impl Filter<Extract = (UserRegister,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json_add_objective() -> impl Filter<Extract = (ObjectiveRegister,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 64).and(warp::body::json())
}

fn post_money() -> impl Filter<Extract = (Money,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 64).and(warp::body::json())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let store = Store::new()?;
    let store_filter = warp::any().map(move || store.clone());

	let add_user = warp::post()
		.and(warp::path("create"))
		.and(warp::path::end())
		.and(post_json_user_register())
		.and(store_filter.clone())
		.and_then(create_user);

	let money = warp::post()
		.and(warp::path("money"))
		.and(warp::path::end())
		.and(post_money())
		.and(store_filter.clone())
		.and_then(set_money);

	let add_objective = warp::post()
		.and(warp::path("objective"))
		.and(warp::path::end())
		.and(post_json_add_objective())
		.and(store_filter.clone())
		.and_then(add_objective);
	let get_objectives = warp::get()
		.and(warp::path!("objective" / u64))
		.and(store_filter.clone())
		.and_then(get_objectives);
	let remove_objectives = warp::delete()
		.and(warp::path!("objective" / u64))
		.and(store_filter.clone())
		.and_then(remove_objective);
	

    let routes = add_user
		.or(money)
		.or(add_objective)
		.or(get_objectives)
		.or(remove_objectives);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
	Ok(())
}