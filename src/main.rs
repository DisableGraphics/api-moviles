use warp::{http, Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

type Items = HashMap<String, i32>;

// ID, NAME, MONEY
type UserType = HashMap<u64, (String, u64)>;
// ID, user id, name, description, cost
type ObjectiveType = HashMap<u64, (u64, String, String, u64)>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserRegister {
	name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ObjectiveRegister {
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
}

impl Store {
    fn new() -> Self {
        Store {
            user_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn create_user(
	user: UserRegister,
	store: Store) -> Result<impl warp::Reply, warp::Rejection>
{
	let id = store.user_list.read().len();
	store.user_list.write().insert(id as u64, (user.name, 0));
	Ok(warp::reply::with_status(
		format!("{}", id),
		http::StatusCode::CREATED,
	))
}

async fn add_objective(
	obj: ObjectiveRegister,
	store: Store) -> Result<impl warp::Reply, warp::Rejection>
{
	let id = store.user_list.read().len();
	store.objective_list.write().insert(id as u64, (user.name, 0));
	Ok(warp::reply::with_status(
		format!("{}", id),
		http::StatusCode::CREATED,
	))
}

fn post_json_user() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json_user_register() -> impl Filter<Extract = (UserRegister,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    /*let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);


    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);*/

	let add_user = warp::post()
		.and(warp::path("create"))
		.and(warp::path::end())
		.and(post_json_user_register())
		.and(store_filter.clone())
		.and_then(create_user);


    let routes = add_user;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}