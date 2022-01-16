use cloud_storage::{Client,ListRequest, Object };
use cloud_storage::object::ObjectList;
use color_eyre::Report;
use futures::executor::block_on;
use futures::StreamExt;
use futures::Stream;
use std::pin::Pin;
use std::boxed::Box;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Storage;

impl Storage {
    pub async fn list() -> Result<(), Report> {
        let client = Client::default();
        let mut stream = Box::pin(client.object().list("codecat_laser", ListRequest::default()).await?);
        while let Some(Ok(data)) = stream.next().await {
            println!("{:?}", data.items);
            for item in data.items {
                println!("{}", item.name);
            }
        }
        Ok(())
    }

    pub async fn read() -> Result<(), Report> {
        let client = Client::default();
        let mut stream = client.object().download_streamed("codecat_laser", "fire.gcode").await?;
        //let mut file = BufWriter::new(File::create("fire.gcode").unwrap());
        let mut buf = Vec::new();
        while let Some(Ok(byte)) = stream.next().await {
            match byte.into() {
                '\n' => {
                    println!("{}", std::str::from_utf8(buf.as_slice()).unwrap());
                    buf.clear();
                }
                _ => buf.push(byte),
            }
        }
        Ok(())
    }
    
}
