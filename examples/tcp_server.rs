use std::sync::Arc;

use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::broadcast};


//수신자에따라 스레드생성 브로드캐스트 전송방식사용시 참고
#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("localhost:1234").await?;
    let (tx,_) =broadcast::channel(10);
    let shared_tx =Arc::new(tx);
    loop{
        //접속대기방식
        let (stream,_) =listener.accept().await?;
        let shared_tx = shared_tx.clone();
        let mut rx = shared_tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer)=tokio::io::split(stream);
            
            tokio::spawn(async move{
                loop{
                    let data:String = match rx.recv().await {
                        Ok(data)=>data,
                        Err(_)=>return 
                    };
                    if data =="/quit"{
                        break;
                    }
                    print!("{}",data);
                    match writer.write_all(data.as_bytes()).await {
                        Ok(_)=>{},
                        Err(err)=>{
                            println!("네트워크전송 에러{:?}",err);
                            return ;
                        }
                        
                    };
                }
            });
            let mut buf_reader =BufReader::new(reader);
            let mut username =String::new();

            buf_reader.read_line(&mut username).await;
            let username = username.trim();

            match shared_tx.send(format!("{} 님이 입장\n",username)){
                Ok(_)=>{},
                Err(_)=>{
                    return;
                }
            }
            loop{
                let mut message =String::new();
                buf_reader.read_line(&mut message).await;

                let mut message =String::from(message.trim());
                if message !="/quit"{
                    message=format!("{}:{}\n",username, message);
                }
                match shared_tx.send(message){
                    Ok(_)=>{},
                    Err(_)=>{break;}
                }
            }
            match shared_tx.send(format!("{} 퇴장\n",username)) {
                Ok(_)=>{},
                Err(_)=>{
                    return ;
                }
            }
        });
        
    }

    Ok(())
}
