#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

#[get("/")]
fn index() -> &'static str {
  "Hello, Sam rust rocket!"
}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
