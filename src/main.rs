use core::num;


#[derive(Debug)]
struct node_com {
    id:String,
    ledger:Vec<String>,
    pool:Vec<String>
}
#[derive(Debug)]
struct minor {
    id:String
}
#[derive(Debug)]
struct system_user {
    id:String
}
#[derive(Debug)]
struct sys_list {
    node_com_list:Vec<node_com>,
    minor_list:Vec<minor>,
    system_user_list:Vec<system_user>
}
fn init_system (node_num:&u32,minor_num:&u32,system_user_num:&u32)->sys_list{
    let mut node_list:Vec<node_com> = Vec::new();
    let mut minor_list:Vec<minor> = Vec::new();
    let mut system_user_list:Vec<system_user> =Vec::new();
    for _i in 0..*node_num {
        node_list.push(node_com{id:String::from("test"),ledger:vec![],pool:vec![]})
    }
    for _i in 0..*minor_num {
        minor_list.push(minor{id:String::from("test")});
    }
    for _i in 0..*system_user_num {
        system_user_list.push(system_user{id:String::from("test")})
    }
    sys_list{node_com_list:node_list,minor_list,system_user_list}
}
fn main() {
    let node_num:u32 = 10;
    let minor_num:u32 = 30;
    let system_user_num:u32 = 5;
    let mut main_sys = init_system(&node_num,&minor_num,&system_user_num);
    
}
