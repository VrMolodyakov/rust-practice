use std::fs;
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn get_random_number() -> i32{
    rand::thread_rng().gen_range(0..=1000)
}

fn get_random_string() -> String{
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()

}

#[derive(Debug)]
pub struct Message{
    content:String,
    id:i32
}

impl Message {
    pub fn new(s : String,id:i32) -> Message{
        Message { 
            content:s,
            id 
        }
    }
}

async fn msg_stream(sender : mpsc::Sender<Message>) {
    loop{
        tokio::time::sleep(Duration::from_secs(1)).await;
        let m = Message::new(get_random_string(),get_random_number());
        println!("message = {:?}",m);
        if let Err(e) = sender.send(m).await{
            println!("channel was closed,{}",e);
            break
        }
    }
}

async fn read_stream(mut receiver : mpsc::Receiver<Message>){
    let (tx, mut rx) = oneshot::channel::<()>();
    loop{
        tokio::select! {
            Err(_) = tokio::time::timeout(Duration::from_secs(3),& mut rx) => {
                println!("time has elapsed");
                break
            }
            message = receiver.recv() =>{
                println!("was receiver message = {:?} ",message)
            }
        }
    }
    println!("END OF STREAM");

}

#[tokio::main]
async fn main() {
    
    let (tx,rx) = mpsc::channel::<Message>(8);
    tokio::join!(msg_stream(tx),read_stream(rx));
    println!("end of programm");
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    // let (tx,mut rx) = oneshot::channel::<()>();
    // loop{
    //     tokio::select! {
    //         Err(_) = tokio::time::timeout(Duration::from_secs(3),& mut rx) => {
    //             println!("time has elapsed");
    //         }
    //     }
    // }
}

