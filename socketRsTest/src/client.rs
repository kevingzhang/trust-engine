

use std::io::{self, Write};
use std::env;
use std::path::PathBuf;
use futures::Future;
use futures::Stream;
use hyper::{rt, Client};
use hyperlocal::{UnixConnector, Uri};

fn main() {
    
    let sock_file_name = get_sock_file ();
    let client = Client::builder()
        .keep_alive(false)
        .build::<_, ::hyper::Body>(UnixConnector::new());
    let url = Uri::new(sock_file_name, "/").into();

    let work = client
        .get(url)
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