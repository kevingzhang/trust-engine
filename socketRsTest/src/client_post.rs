

use std::io::{self, Write};
use std::env;
use std::path::PathBuf;
use futures::Future;
use futures::Stream;
use hyper::{rt, Client, Body, Request};
use hyperlocal::{UnixConnector, Uri};
use url::form_urlencoded;
fn main() {
    
    let sock_file_name = get_sock_file ();
    println!("sock_file_name {}", sock_file_name);
    let client = Client::builder()
        .keep_alive(false)
        .build::<_, ::hyper::Body>(UnixConnector::new());
    
    let url = Uri::new(sock_file_name, &path_with_query("ping")).into();
    println!("url is {:#?}", url);
    
    let body = form_urlencoded::Serializer::new(String::new())
      .append_pair("When", "Now")
      .append_pair("Where", "The Moon")
      .finish();
    let work = client
        .post(url)
        .body(&body[..])
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            res.into_body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        });

    rt::run(work);
}

fn get_sock_file () -> String {
    match env::var("SOCKETFILE"){
        Ok(f)=>f,
        _=>{
            let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            println!("cargo_manifest_dir is {}", cargo_manifest_dir);
            let mut buf = PathBuf::from(cargo_manifest_dir);
            buf.set_file_name("rust.sock");
            buf.as_path().to_str().unwrap().to_string()
        }
    }
}

fn path_with_query(pass_in : &str)->(String){
    let mut map = std::collections::HashMap::new();
    map.insert("FirstName", "Kevin");
    map.insert("LastName" , "Zhang");
    map.insert("Occupation" , "ProgrammingApe");
    "/".to_string() + pass_in + "?" + &build_query_url_part(map)
}
fn build_query_url_part(input : std::collections::HashMap<&str, &str>) -> String {
    let mut builder = form_urlencoded::Serializer::new(String::new());
    for (k, v) in input{
        builder.append_pair(&k, &v);
    }
    builder.finish()
        // .append_pair("foo", "bar & baz")
        // .append_pair("saison", "Été+hiver")
        // .finish();
}
