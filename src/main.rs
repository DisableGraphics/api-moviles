use warp::Filter;

use std::error::Error;

mod helper;
mod handlers;
use handlers::{add_objective, create_user, edit_objective, get_objectives, logo, remove_objective, set_money};
mod structs;

mod store;
use store::Store;
mod post_handlers;
use post_handlers::{post_json_user_register, post_money, post_json_add_objective, post_json_edit_objective};

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
	let remove_objective = warp::delete()
		.and(warp::path!("objective" / u64))
		.and(store_filter.clone())
		.and_then(remove_objective);
	let edit_objective = warp::post()
		.and(warp::path!("edit_objective"))
		.and(warp::path::end())
		.and(post_json_edit_objective())
		.and(store_filter.clone())
		.and_then(edit_objective);

	let logo = warp::get()
		.and(warp::path!("logo"))
		.and_then(logo);

    let routes = add_user
		.or(money)
		.or(add_objective)
		.or(get_objectives)
		.or(remove_objective)
		.or(edit_objective)
		.or(logo);
	println!("Serving at 127.0.0.1:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
	Ok(())
}