use std::cmp::Ordering;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use std::{io, thread};

use chrono::Utc;

use crate::content_dealer;
use crate::text_msg;
use crate::text_msg::TextMessage;

pub fn run(local_host: &'static str, msg_size: usize, sleep_millis: u64, username: &str) {
    let mut client = TcpStream::connect(local_host).expect("连接失败");
    let local_address_text = client.local_addr().expect("address 解析失败").to_string();
    client.set_nonblocking(true).expect("无法设置为非阻塞");
    //建立管道
    let (sender, receiver) = mpsc::channel::<String>();
    //不能放入闭包内,否则local_address就进入闭包了
    let check = local_address_text.clone();
    thread::spawn(move || loop {
        let mut buffer = vec![0; msg_size];
        //read_exact:读尽可能多的数据来填充缓冲区
        match client.read_exact(&mut buffer) {
            Ok(_) => {
                //读取消息
                let msg = buffer
                    .into_iter()
                    .take_while(|&cur| cur != 0)
                    .collect::<Vec<_>>();
                let msg = String::from_utf8(msg).unwrap();
                //使用fromstr的trait来通过String解析对象
                //这里是接收方
                match msg.parse::<text_msg::TextMessage>() {
                    Ok(text_message) => match &check.cmp(&text_message.from().to_string()) {
                        Ordering::Equal => {}
                        _ => {
                            println!(
                                "{}({}): {}",
                                text_message.username(),
                                text_message.m_date(),
                                text_message.content()
                            )
                        }
                    },
                    Err(_) => println!("解析失败"),
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection error");
                break;
            }
        }
        match receiver.try_recv() {
            Ok(msg) => {
                //将缓冲区的信息变成字节形式
                let mut buffer = msg.clone().into_bytes();
                //扩展缓冲区,需要的话扩展,并用value填充,类似于realloc()
                buffer.resize(msg_size, 0);
                client.write_all(&buffer).expect("fail to get message");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        thread::sleep(Duration::from_millis(sleep_millis));
    });
    println!("已进入聊天室");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("fail to read");
        let msg = buffer.trim().to_string();
        if &msg == "quit" || &msg == "exit" {
            break;
        }
        //创建消息对象并发送
        //为了密封性使用构造函数构造
        let t_msg = TextMessage::new(
            //to_string相当于复制值
            local_address_text.to_string(),
            local_host.to_string(),
            content_dealer::run(msg.trim()).to_string(),
            Utc::now().format("%T").to_string(),
            username.to_string(),
        );
        if sender.send(t_msg.to_string()).is_err() {
            break;
        }
    }
    println!("bye");
}
