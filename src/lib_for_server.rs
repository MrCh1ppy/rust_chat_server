use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use crate::lib::TextMessage;

pub fn run(local_host:&'static str,msg_size:usize,sleep_millis:u64) {
    //绑定地址
    let listener = TcpListener::bind(local_host).expect("无法绑定socket");
    //设置为无阻塞模式
    listener.set_nonblocking(true).expect("设置为无阻塞失败");
    //添加用户队列
    let mut clients = vec![];
    //定义一个String隧道用于传递消息
    let (c_sender, c_receiver) = mpsc::channel::<String>();
    println!("初始化完成");
    loop {
        //accept返回一个stream与socket地址的结果集
        //OK函数
        if let Ok((mut stream,address))=listener.accept(){
            println!("client {},connected", address);
            //为客户端提供一个channel的使用机会
            //将客户端的流压入客户端队列
            let sender = c_sender.clone();
            clients.push(stream.try_clone().expect("客户端压入队列失败"));
            //生成一个属于客户端的线程
            thread::spawn(move || loop {
                let mut msg_buffer = vec![0; msg_size];
                match stream.read_exact(&mut msg_buffer) {
                    Ok(_) => {
                        //获取获得的消息
                        let msg = msg_buffer
                            //用迭代器遍历
                            .into_iter()
                            //获取不是空格的信息
                            //用while调用函数,符合则加入,不符合则中断,忽略后面的
                            .take_while(|&x| x != 0)
                            //将其加入vec中
                            .collect::<Vec<_>>();
                        let msg:String = String::from_utf8(msg).expect("无法转换");
                        let text_msg=msg.parse::<TextMessage>().expect("消息体转换失败");
                        //{:?}使用debug模式输出
                        println!("{}:{:?}", address, msg);
                        //将消息发出
                        sender.send(text_msg.to_string()).expect("fail to send message");
                    }
                    //如果错误会让进程阻塞,那么返回一个()单元,即void
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    //否则关闭链接
                    Err(_) => {
                        println!("close connection with:{}", address);
                        break;
                    }
                }
                //睡眠0.1s
                thread::sleep(::std::time::Duration::from_millis(sleep_millis))
            });
        }
        //获取msg值并取名为msg
        //如果发现管道的缓冲区有东西,那么进行以下操作
        if let Ok(msg)=c_receiver.try_recv(){
            println!("msg[{:?}] is received",msg);
            clients=clients.into_iter().filter_map(|mut client|{
                //将消息转换为字节
                let mut buffer=msg.clone().into_bytes();
                //将buffer的size变成msg的size
                buffer.resize(msg_size, 0);
                //获取客户端,写入缓冲区所有消息的内容
                //那个map的意思似乎是回退,变回最开始的样式
                //write_all啥都没返回
                client.write_all(&buffer).map(|_|client).ok()
            }).collect::<Vec<_>>();//刷新clients
        }
        thread::sleep(::std::time::Duration::from_millis(sleep_millis));
    }
}