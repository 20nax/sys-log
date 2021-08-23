use postgres::{Client, NoTls};
use std::time;
mod controller;
mod service;

fn main() {
    let client = Client::connect("host=localhost user=postgres password=password", NoTls);
    let con = match client {
        Ok(client) => client,
        Err(_) => panic!("Aled"),
    };
    
    let res = controller::db::cpu_write(con);
    let con = match res {
        Ok(res) => {
            println!("Sucess: cpu logs");
            res
        }
        Err(_) => panic!("Failed: cpu logs"),
    };

    let res = controller::db::cpu_write(con);
    let con = match res {
        Ok(_) => {
            println!("Sucess: cpu logs");
            res
        }
        Err(_) => panic!("Failed: cpu logs"),
    }
}
