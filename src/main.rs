#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Zażółć gęślą jaźń!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
