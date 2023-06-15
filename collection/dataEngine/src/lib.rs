pub mod data_engine_mod
{


use std::{error::Error};
use mysql::{self, PooledConn, Pool};
use mysql::prelude::Queryable;
use mysql::params;
use mysql::prelude::TextQuery;

use std::sync::mpsc::*;
use std::thread;
use fastrand;


pub struct DataEngine
{
    pub strings : Vec<String>,
    pub seed : u64,
    pub statement : String,
    pub pool_con : Option<PooledConn>,
    pub sender : Sender<String>
}

fn conection_from_pool(pool : Option<Pool>) -> Option<PooledConn>{
    match pool {
        Some(pool) => {
            match pool.get_conn() {
                Ok(conn) => Some(conn),
                Err(err) => {
                    eprintln!("failed to connect to the database {err}");
                    None
                }
            }
        },
        None => None
    }
}

impl DataEngine
{
    pub fn new() -> DataEngine
    {    
    
        let url = "mysql://user:password@ip:port/SNAKE";
        let pool = match mysql::Pool::new(url){
            Ok(pool) => Some(pool),
            Err(err) => {
                eprintln!("Failed to create a connection pool {err}");
                None
            }
        };

        let conn_main = conection_from_pool(pool.clone());
        let conn_thread = conection_from_pool(pool);


        let (tx, rx) = channel();
        if let Some(mut connection) = conn_thread {
            thread::spawn(move || {
                loop {
                    match rx.recv() {
                        Ok(statment) => {
                            match connection.query_drop(&statment){
                                Ok(_) => eprintln!("Data sent succesfully"),
                                Err(err) => eprintln!("Failed to send data to the db {}", err)
                            }
                        },
                        Err(_) => {
                            println!("closing thread");
                            break;
                        }
                    };
                }
            });
        }

        let de = DataEngine{
            strings : vec![], 
            seed : 1, 
            statement : "".to_string(),
            pool_con : conn_main,
            sender : tx
        };
        return de;
    }

    pub fn create_statement(&mut self, flattened_world : String)
    {
        self.statement += &format!("INSERT INTO runs
        VALUES ({});", flattened_world);
    }

    pub fn send_run(&mut self) -> Result<(), Box<dyn Error>>
    {
        match self.sender.send(self.statement.clone()){
            Ok(_) => (),
            Err(err) => eprintln!("Failed to send to thread {}",err)
        };
        Ok(())     
    }

    pub fn get_seed(&mut self) -> Result<(), Box<dyn Error>>
    {
        println!("Retrieving random seed from the database...");
        self.statement = "".to_string();
        if let Some(connection) = &mut self.pool_con{
            self.seed = match connection.query_map("SELECT * FROM seeds ORDER BY RAND() LIMIT 1;", |i| -> u64 {i}){
                Ok(seeds) => {
                    match seeds.first() {
                        Some(seed) => *seed,
                        None => {
                            eprintln!("The seed query did not return any seeds");
                            fastrand::u64(..)
                        }
                    }
                    
                },
                Err(err) => {
                    eprintln!("The seed query failed {err}");
                    fastrand::u64(..)
                }
            };
            
            let seed_str = self.seed.to_string();
            println!("{}", seed_str);

            let del_statement = &format!("DELETE FROM seeds WHERE seed = {}",self.seed);

            match connection.query_drop(&del_statement){
                Ok(_) => eprintln!("Seed deleted succesfully"),
                Err(err) => eprintln!("Failed to delete seed from the db {}", err)
            }

        };
        
         
        

        

        Ok(())
    }
}
}

/*mod data_engine_mod;
use crate::data_engine_mod::data_engine_mod::DataEngine;



fn main()
{
    let mut de = DataEngine::new();
    
    if let Err(err) = de.get_seed()
    {
        println!("{}", err);
        // Here initialize gameEngine with de.seed
    }
    
    de.create_file();
    de.push(vec!["1".to_string(),
                 "2".to_string(),
                 "3".to_string()]);
    de.push(vec!["2".to_string(),
                 "4".to_string(),
                 "6".to_string()]);
    de.push(vec!["1".to_string(),
                 "2".to_string(),
                 "3".to_string()]);                 

    if let Err(err) = de.save_to_file()
    {
        println!("{}", err);
    }
} */