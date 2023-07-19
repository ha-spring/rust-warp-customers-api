use std::convert::Infallible;
use wrap::{self, http::StatusCode};

use crate::models::Customer;
use crate::db::Db;

pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible> {
  let customers = db.lock().await;
  let customers: Vec<Customer> = customers.clone();
  Ok(wrap::reply::json(&customers))
}

pub async fn create_customer(new_customer: Customer, db: Db) -> Result<impl warp::Reply, Infallible> {
  let mut customers = db.lock().await;

  for customer in customers.iter() {
    if customer.guid == new_customer.guid {
      return Ok(StatusCode::BAD_REQUEST);
    }
  }

  customers.push(new_customer);

  Ok(StatusCode::CREATED)
}


