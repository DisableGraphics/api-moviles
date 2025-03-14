use std::{collections::HashMap, io::Read};

use crate::{store::Store, structs::{Money, Objective, ObjectiveRegister, UserRegister}};
use warp::{http, reply::Response};

fn get_last_free<K>(list: &HashMap<u64, K>) -> usize {
	let mut prev: usize = 0;
	for elem in list {
		if *elem.0 > (prev as u64) {
			prev = *elem.0 as usize;
		}
	}

	prev + 1
}

pub async fn create_user(
	user: UserRegister,
	store: Store) -> Result<impl warp::Reply, warp::Rejection>
{
	let id = store.user_list.read().len();
	store.user_list.write().insert(id as u64, (user.name, 0));
	let _ = store.save_users();
	println!("Creating user {id}");
	Ok(warp::reply::with_status(
		format!("{}", id),
		http::StatusCode::OK,
	))
}

pub async fn add_objective(
	obj: ObjectiveRegister,
	store: Store) -> Result<impl warp::Reply, warp::Rejection>
{
	let id = get_last_free(&store.objective_list.read());
	let user_exists = store.user_list.read().contains_key(&obj.user_id);

	if user_exists {
		store.objective_list.write().insert(id as u64, (obj.user_id, obj.name, obj.desc, obj.cost));
		let _ = store.save_objectives();
		println!("Created objective {id}");
		Ok(warp::reply::with_status(
			format!("{}", id),
			http::StatusCode::OK,
		))
	} else {
		Ok(warp::reply::with_status(
			"User does not exist".to_string(), 
		http::StatusCode::NOT_FOUND))
	}
}

pub async fn edit_objective (
	obj: Objective,
	store: Store) -> Result<impl warp::Reply, warp::Rejection>
{
	let user_exists = store.user_list.read().contains_key(&obj.user_id);
	if user_exists {
		store.objective_list.write().insert(obj.id as u64, (obj.user_id, obj.name, obj.desc, obj.cost));
		let _ = store.save_objectives();
		println!("Edited objective");
		Ok(warp::reply::with_status("Ok", http::StatusCode::OK))
	} else {
		Ok(warp::reply::with_status(
			"User does not exist", 
		http::StatusCode::NOT_FOUND))
	}
}

pub async fn get_objectives(
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

	println!("Got objs: {:?}", ret);

	Ok(warp::reply::json(
		&ret
	))
}

pub async fn remove_objective(
	obj: u64,
	store: Store
	)  -> Result<impl warp::Reply, warp::Rejection> {
	{
		let mut write = store.objective_list.write();
		write.remove(&obj);
	}
	println!("Removed objective");
	let _ = store.save_objectives();
	Ok(warp::reply())
}

pub async fn set_money(
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
	println!("Set money");
	let _ = store.save_users();
	Ok(warp::reply())
}

pub async fn logo() -> Result<impl warp::Reply, warp::Rejection> {
	let img = std::fs::File::open("logo.png").unwrap();
	let bytes: Vec<_> = img.bytes().filter_map(|x| { 
		if x.is_ok() {
			Some(x.unwrap())
		} else {
			None
		}
	}).collect();
	let response = Response::new(bytes.into());
	
	Ok(response)
}