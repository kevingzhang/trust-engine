
use std::env;
use hyper::service::service_fn;
use hyper::{header, Body, Request, Response};
use vrf::openssl::{CipherSuite, ECVRF};
use vrf::VRF;
use std::{fs, io};
use std::path::{PathBuf};
const PHRASE: &'static str = "It's a Unix system. I know this.";

fn hello(
    req: Request<Body>,
) -> impl futures::Future<Item = Response<Body>, Error = io::Error> + Send {
    println!("servicing new request {:?}", req);
    let uri = req.uri();
     match uri.path_and_query(){
        Some(path_query)=>{
            println!("reqested path {:#?}", path_query.path());
            println!("reqested query {:#?}", path_query.query().unwrap_or("none"));
        }
        _=>{
            println!("uri: {:#?}", uri);
        }
    }
    
    let res = match req.uri().to_string().as_ref(){
            "/ping" =>{
                "Pong"
            },
            _ => PHRASE,
    };
    vrf();
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


fn vrf() {
    let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
    // Inputs: Secret Key, Public Key (derived) & Message
    let secret_key =
        hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721").unwrap();
    let public_key = vrf.derive_public_key(&secret_key).unwrap();
    let message: &[u8] = b"sample";

    // VRF proof and hash output
    let pi = vrf.prove(&secret_key, &message).unwrap();
    let hash = vrf.proof_to_hash(&pi).unwrap();
    println!("Generated VRF proof: {}", hex::encode(&pi));

    // VRF proof verification (returns VRF hash output)
    let beta = vrf.verify(&public_key, &pi, &message);

    match beta {
        Ok(beta) => {
            println!("VRF proof is valid!\nHash output: {}", hex::encode(&beta));
            assert_eq!(hash, beta);
        }
        Err(e) => {
            println!("VRF proof is not valid: {}", e);
        }
    }
}