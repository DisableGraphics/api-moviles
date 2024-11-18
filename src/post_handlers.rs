use warp::Filter;

use crate::structs::{Money, Objective, ObjectiveRegister, UserRegister};


pub fn post_json_user_register() -> impl Filter<Extract = (UserRegister,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn post_json_add_objective() -> impl Filter<Extract = (ObjectiveRegister,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 64).and(warp::body::json())
}

pub fn post_money() -> impl Filter<Extract = (Money,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 64).and(warp::body::json())
}

pub fn post_json_edit_objective() -> impl Filter<Extract = (Objective,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 64).and(warp::body::json())
}