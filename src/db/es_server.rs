use std::collections::HashMap;

//use rs_es::Client;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::utils::es_client::{self, ESClient};

pub async fn es_client() {
    /* let mut client = Client::init("http://172.18.0.1:9200").unwrap();
    //let z = client.refresh().send().unwrap();
    let x: rs_es::operations::get::GetResult<Source> = client.get("test", "01").with_doc_type("_doc").send().unwrap();
    //println!("{:?}", x.id); */
    let res = Client::new().get("http://172.18.0.1:9200/test/_doc/01").send().await.unwrap();
    let x = res.text().await.unwrap();
    println!("{}", x);
}
