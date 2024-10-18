use dotenv::dotenv;
use mysql::*;
use mysql::prelude::*;
use std::env;

#[derive(Debug)]
struct Test {
    analyzeResultId: String,
    regDT: String
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = Pool::new(database_url).expect("Failed to create pool");
    let mut conn = pool.get_conn().expect("Failed to get connection");

    let results: Vec<Test> = conn.query_map(
        "SELECT analyzeResultId, regDT FROM tb_analyzeResult",
        |(analyzeResultId, regDT)| {
            Test {
                analyzeResultId,  
                regDT, 
            }
        },
    ).expect("Query failed");

    for result in results {
        println!("{:?}", result);
    }
}

