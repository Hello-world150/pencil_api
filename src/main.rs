#[macro_use]
extern crate rocket;

use rand::SeedableRng;
use rand::rngs::StdRng;
use rocket::State;
use rocket::serde::json::Json;
use std::sync::Mutex;

use pencil_api::models::data::Data;
use pencil_api::models::error::ErrorMessage;
use pencil_api::models::hitokoto::*;

#[get("/hitokoto")]
fn random_hitokoto(
    data: &State<Mutex<Data>>,
    rng: &State<Mutex<StdRng>>,
) -> Json<Result<Hitokoto, ErrorMessage>> {
    let mut rng = rng.lock().unwrap();
    let data_lock = data.lock().unwrap();
    match get_random_hitokoto(&*data_lock, &mut *rng) {
        Some(hitokoto) => Json(Ok(hitokoto)),
        None => Json(Err(ErrorMessage("No hitokoto found"))),
    }
}

#[post("/hitokoto", data = "<new_hitokoto>")]
fn create_hitokoto(
    new_hitokoto: Json<NewHitokoto>,
    data: &State<Mutex<Data>>,
) -> Json<Result<Hitokoto, ErrorMessage>> {
    // Create a new Hitokoto from the incoming JSON
    let hitokoto = Hitokoto::from(new_hitokoto.into_inner());

    // Get a mutable reference to the data
    match data.lock() {
        Ok(mut data_lock) => {
            data_lock.hitokotos.push(hitokoto.clone());
            Json(Ok(hitokoto))
        }
        Err(_) => Json(Err(ErrorMessage("Failed to lock data"))),
    }
}
#[launch]
fn rocket() -> _ {
    let data = Mutex::new(pencil_api::models::data::load_data());
    let rng = Mutex::new(StdRng::seed_from_u64(0)); // Seed the RNG
    rocket::build()
        .manage(data)
        .manage(rng)
        .mount("/", routes![random_hitokoto, create_hitokoto])
}
