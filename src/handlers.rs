use std::convert::Infallible;
use wrap::{self, http::StatusCode};

use crate::db::Db;
use crate::models::Customer;

pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await;
    let customers: Vec<Customer> = customers.clone();
    Ok(wrap::reply::json(&customers))
}

pub async fn create_customer(
    new_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for customer in customers.iter() {
        if customer.guid == new_customer.guid {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    customers.push(new_customer);

    Ok(StatusCode::CREATED)
}

pub async fn get_customer(guid: String, db: Db) -> Result<Box<dyn wrap::Result>, Infallible> {
    let customers = db.lock().await;

    for customer in customers.iter() {
        if customer.guid == guid {
            return Ok(Box::new(wrap::reply::json(customer)));
        }
    }

    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn update_customer(
    guid: String,
    updated_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for customer in customers.iter() {
        if customer.guid == guid {
            *customer = update_customer;
            return Ok(StatusCode::OK);
        }
    }

    return Ok(StatusCode::NOT_FOUND);
}

pub async fn delete_customer(guid: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    let customer_count = customers.len();

    customers.retain(|customer| customer.guid != guid);

    let deleted = customers.len() != customer_count;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
