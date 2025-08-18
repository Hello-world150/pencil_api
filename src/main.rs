#[macro_use]
extern crate rocket;

use rand::SeedableRng;
use rand::rngs::StdRng;
use rocket::State;
use rocket::serde::json::Json;
use std::sync::Mutex;

use pencil_api::models::anthology::Anthology;
use pencil_api::models::data::Data;
use pencil_api::models::error::ErrorMessage;
use pencil_api::models::hitokoto::*;

/// Get a random hitokoto
// TODO: query parameters support
#[get("/hitokoto")]
fn random_hitokoto(
    data: &State<Mutex<Data>>,
    rng: &State<Mutex<StdRng>>,
) -> Json<Result<Hitokoto, ErrorMessage>> {
    let mut rng = rng.lock().unwrap();
    let data_lock = data.lock().unwrap();
    match get_random_hitokoto(&data_lock, &mut *rng) {
        Some(hitokoto) => Json(Ok(hitokoto)),
        None => Json(Err(ErrorMessage("No hitokoto found"))),
    }
}

/// Add a new hitokoto
#[post("/hitokoto", data = "<new_hitokoto_json>")]
fn create_hitokoto(
    new_hitokoto_json: Json<NewHitokoto>,
    data: &State<Mutex<Data>>,
) -> Result<Json<Hitokoto>, Json<ErrorMessage>> {
    // TODO: Json error processing
    let new_hitokoto = new_hitokoto_json.into_inner();
    // Create a new Hitokoto from the incoming JSON
    let hitokoto = Hitokoto::from(new_hitokoto);

    // Get a mutable reference to the data
    match data.lock() {
        Ok(mut data_lock) => {
            data_lock.hitokotos.push(hitokoto.clone());
            Ok(Json(hitokoto))
        }
        Err(_) => Err(Json(ErrorMessage("Failed to lock data"))),
    }
}

/// Add a new anthology
#[post("/anthology", data = "<new_anthology_json>")]
fn create_anthlogy(
    new_anthology_json: Json<Anthology>,
    data: &State<Mutex<Data>>,
) -> Result<Json<Anthology>, Json<ErrorMessage>> {
    let new_anthology = new_anthology_json.into_inner();
    let anthology = Anthology::from(new_anthology);

    match data.lock() {
        Ok(mut data_lock) => {
            data_lock.anthologies.push(anthology.clone());
            Ok(Json(anthology))
        }
        Err(_) => Err(Json(ErrorMessage("Server Error"))),
    }
}
#[launch]
fn rocket() -> _ {
    let data = Mutex::new(pencil_api::models::data::load_data());
    let rng = Mutex::new(StdRng::seed_from_u64(0)); // Seed the RNG
    rocket::build().manage(data).manage(rng).mount(
        "/",
        routes![random_hitokoto, create_hitokoto, create_anthlogy],
    )
}
