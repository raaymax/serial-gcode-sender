use cloud_storage::{Client,ListRequest };
use futures::StreamExt;
use std::boxed::Box;
use std::io::BufReader;
use crate::vec_reader::VecReader;

pub struct Storage;

impl Storage {
    pub async fn list() -> Result<(), String> {
        let client = Client::default();
        let mut stream = Box::pin(client.object().list("codecat_laser", ListRequest::default()).await.map_err(|err|err.to_string())?);
        while let Some(Ok(data)) = stream.next().await {
            println!("{:?}", data.items);
            for item in data.items {
                println!("{}", item.name);
            }
        }
        Ok(())
    }

    pub async fn read(path: String) -> BufReader<VecReader> {
        let client = Client::default();
        let stream = client.object().download_streamed("codecat_laser", path.as_str()).await.unwrap();
        let buf = stream.map(|x| x.unwrap()).collect::<Vec<u8>>().await;
        BufReader::new(VecReader(buf))
    }
    
}
