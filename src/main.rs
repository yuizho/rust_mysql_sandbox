extern crate chrono;
extern crate mysql;

use chrono::prelude::*;
use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Product {
    id: i32,
    division: i32,
    created: NaiveDateTime,
    name: String,
}

fn main() {
    let pool = my::Pool::new("mysql://test:password@localhost:3306/test").unwrap();
    let products: Vec<Product> = pool
        .prep_exec(r"SELECT id, division, created, name FROM product", ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, division, created, name) = my::from_row(row);
                    Product {
                        id,
                        division,
                        created,
                        name,
                    }
                })
                .collect()
        })
        .unwrap();
    println!("{:?}", products);
}
