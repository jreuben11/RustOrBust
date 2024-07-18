use rocket::*;

#[get("/")]
async fn index() -> &'static str {
    "Hello, bakeries!"
}

#[launch] // The "main" function of the program
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
