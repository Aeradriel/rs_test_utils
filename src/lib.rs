#[cfg(feature = "postgres")]
extern crate diesel;
#[cfg(feature = "dotenv")]
extern crate dotenv;
#[cfg(feature = "lazy_static")]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "rocket_support")]
extern crate rocket;

#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "rocket_support")]
pub mod rocket_support;
