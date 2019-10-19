
use std::env;
use hyper::service::service_fn;
use hyper::{header, Body, Request, Response};
use std::{fs, io};
use std::path::{PathBuf};
const PHRASE: &'static str = "It's a Unix system. I know this.";

fn hello(
    req: Request<Body>,
) -> impl futures::Future<Item = Response<Body>, Error = io::Error> + Send {
    println!("servicing new request {:?}", req);
    let uri = req.uri();
    let path_query = uri.path_and_query().unwrap();
    println!("reqested path {:#?}", path_query.path());
    println!("reqested query {:#?}", path_query.query().unwrap_or("none"));
    let res = match req.uri().to_string().as_ref(){
            "/ping" =>{
                "Pong"
            },
            _ => PHRASE,
    };
    futures::future::ok(
        Response::builder()
            .header(header::CONTENT_TYPE, "text/plain")
            .header(header::CONTENT_LENGTH, res.len() as u64)
            .body(res.into())
            .expect("failed to create response"),
    )
}

fn run() -> io::Result<()> {
    
    let sock_file_name = get_sock_file();
    if let Err(err) = fs::remove_file(&sock_file_name) {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err);
        }
    }

    let svr = hyperlocal::server::Server::bind(&sock_file_name, || service_fn(hello))?;

    {
        let path = svr.local_addr().as_pathname().unwrap();
        println!(
            "Listening on unix://{path} with 1 thread.",
            path = path.to_string_lossy(),
        );
    }

    svr.run()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error starting server: {}", err)
    }
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