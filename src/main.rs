#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, Sam Web rust rocket!"
}

fn main() {
    println!("Hello, world!");
    rocket::ignite().mount("/", routes![hello]).launch();
}
