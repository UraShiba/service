use ::std::env;
use service::{
    function::{self},
    models::UserInfo,
};

fn help() {
    println!(
        "PLEASE SET ARGUMENT \n
        cargo run   <arg1>      <arg2>  <arg3>
                    insert 
                    delete       <id>
                    query        <id> 
                    update_pass  <id>   <pass>
        "
    )
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let func = &args[1];
            match func.as_str() {
                "insert" => function::insert::insert(UserInfo {
                    user_id: 2,
                    user_name: "maejima".to_string(),
                    login_name: "maejima".to_string(),
                    pass: "maemae".to_string(),
                }),
                _ => {
                    help();
                }
            }
        }
        3 => {
            let func = &args[1];
            let _query_id: i32 = args[2].parse().unwrap();
            match func.as_str() {
                "delete" => {
                    function::delete::delete(_query_id);
                }
                "query" => {
                    function::query::query(_query_id);
                }
                _ => {
                    help();
                }
            }
        }
        4 => {
            let func = &args[1];
            let _query_id: i32 = args[2].parse().unwrap();
            let _pass: String = args[3].parse().unwrap();
            match func.as_str() {
                "update_pass" => function::update::update_pass(_query_id, _pass),
                _ => {
                    help();
                }
            }
        }
        _ => {
            help();
        }
    }
    println!("Databaese result");
    function::query::query_all();
}
