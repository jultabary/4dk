#[macro_use] extern crate rocket;

use rocket::State;

struct ValueToInject {
    value: bool
}
unsafe impl Send for ValueToInject { }
unsafe impl Sync for ValueToInject { }

#[get("/")]
fn index(value: &State<ValueToInject>) -> String {
    let response = String::from("Hello, world! ") +  &value.value.to_string();
    return response;
}

#[rocket::main]
async fn main() {
    let value = ValueToInject { value: true };
    let server = rocket::build()
        .manage(value)
        .mount("/", routes![index]).launch().await;
}