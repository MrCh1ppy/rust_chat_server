use std::io;

mod lib_for_server;
mod lib_for_client;
mod lib;

const LOCAL_HOST: &str = "127.0.0.1:7878";
const MESSAGE_SIZE: usize = 128;
const SLEEP_MILLIS: u64 =100;

fn main() {
    loop{
        println!("开始界面\n请键入命令:\nserver:开启服务器\nclient:开启客户端\nexit:退出");
        let mut order=String::new();
        io::stdin().read_line(&mut order).expect("非法字符串");

        match order.trim() {
            "exit"=>break,
            //运行服务器模式
            "server" => lib_for_server::run(LOCAL_HOST,MESSAGE_SIZE,SLEEP_MILLIS),
            //运行客户端模式
            "client"=> {
                println!("键入用户名:");
                let username=get_username();
                lib_for_client::run(LOCAL_HOST,MESSAGE_SIZE,SLEEP_MILLIS,username.as_str());
            },
            _ => println!("命令非法")
        }
    }
}

fn get_username() -> String{
    let mut username=String::new();
    io::stdin().read_line(&mut username).expect("fail to read");
    username.trim().to_string()
}
