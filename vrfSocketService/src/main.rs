mod sortition;
use std::env;
use hyper::service::service_fn;
use url::Url;
use hyper::{header, Body, Request, Response, Method};
use vrf::openssl::{CipherSuite, ECVRF};
use vrf::VRF;
use std::{fs, io};
use std::path::{PathBuf};
const PHRASE: &'static str = "It's a Unix system. I know this.";
use serde_json;
use std::fmt;
use secp256k1::{Secp256k1, Message};
use secp256k1::rand::OsRng;

use dirs;

fn vrf_services(
    req: Request<Body>,
) -> impl futures::Future<Item = Response<Body>, Error = io::Error> + Send {
    
    println!("servicing new request {:?}", req);
    let uri = req.uri();
    // match uri.path_and_query(){
    //     Some(path_query)=>{
    //         println!("reqested path {:#?}", path_query.path());
    //         println!("reqested query {:#?}", path_query.query().unwrap_or("none"));
    //     }
    //     _=>{
    //         println!("uri: {:#?}", uri);
    //     }
    // }
    
    let res = match (req.method(), req.uri().path()){
            (&Method::GET, "/ping") =>{
                "Pong".to_string()
            },
            (&Method::GET, "/get_rand_secret") =>{
                let ret = get_rand_secret().to_string();
                //println!("Returns: {}", ret);
                ret
            },
            (&Method::GET, "/get_vrf_proof") =>{
                let uri_string = format!("http://unix{}", req.uri());
                let request_url = Url::parse(&uri_string).unwrap();
                let params = request_url.query_pairs();
                let mut public_key = String::new();
                let mut secret_key = String::new();
                let mut message = String::new(); 
                for param in params{
                    //println!("Key-Value:{} - {}", param.0, param.1);
                    match param.0.to_string().as_ref(){
                        "p"=>public_key = param.1.to_string(),
                        "s"=>secret_key = param.1.to_string(),
                        "m"=>message = param.1.to_string(),
                        _=>()
                    }
                };
                let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
                let recal_public_key = vrf.derive_public_key(&hex::decode(&secret_key).unwrap()).unwrap(); 
                assert_eq!(hex::encode(recal_public_key), public_key);
                println!("vrf object is created");
                let p1 = &hex::decode(&secret_key).unwrap();
                let p2 = &message.as_bytes();
                let pi = vrf.prove(p1, p2).unwrap();
                let hash = vrf.proof_to_hash(&pi).unwrap();
                let ret = serde_json::json!({
                    "pi": hex::encode(pi).to_string(),
                    "hash":hex::encode(hash).to_string()
                });
                ret.to_string()
            },
            (&Method::GET, "/get_vrf_verified") =>{
                let uri_string = format!("http://unix{}", req.uri());
                let request_url = Url::parse(&uri_string).unwrap();
                let params = request_url.query_pairs();
                let mut public_key = String::new();
                let mut pi = String::new();
                let mut message = String::new(); 
                for param in params{
                    println!("Key-Value:{} - {}", param.0, param.1);
                    match param.0.to_string().as_ref(){
                        "p"=>public_key = param.1.to_string(),
                        "pi"=>pi = param.1.to_string(),
                        "m"=>message = param.1.to_string(),
                        _=>()
                    }
                };
                let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
                let pk_ref = &hex::decode(&public_key).unwrap();
                let pi_ref = &hex::decode(&pi).unwrap();
                //println!("pk_ref - pi_ref: {:#?}-{:#?}", pk_ref, pi_ref);
                sortition::sortition();
                match vrf.verify(pk_ref, pi_ref, &message.as_bytes()){
                    Ok(beta) => {
                        println!("VRF proof is valid!\nHash output: {}", hex::encode(&beta));
                        serde_json::json!({
                            "result":true
                        }).to_string()
                    }
                    Err(e) => {
                        println!("VRF proof is not valid: {}", e);
                        serde_json::json!({
                            "result":false
                        }).to_string()
                    }
                }
                
            },
            _ => {
                println!("did not find any match for Method:{} and path:{}", &(req.method()), &(req.uri().path()));
                req.uri().to_string()
            },
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

    let svr = hyperlocal::server::Server::bind(&sock_file_name, || service_fn(vrf_services))?;

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

fn get_sock_file () -> String {
    match env::var("SOCKETFILE"){
        Ok(f)=>f,
        _=>{
             
            let mut buf = PathBuf::from(dirs::home_dir().unwrap()).join("sockets").join("files");
            println!("buf {:#?}", buf);
           
            buf.set_file_name("vrf_service.sock");
            println!("buf {:#?}", buf);
            buf.as_path().to_str().unwrap().to_string()
        }
    }
}


fn get_rand_secret()-> serde_json::Value{

    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);
    // let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
    // //let secret_key = rand::thread_rng().gen::<[u8; 32]>();
    // let secret_key =
    //     hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721").unwrap();
    
    // let public_key = vrf.derive_public_key(&secret_key).unwrap();
    serde_json::json!(
        {
            "secret_key": format!("{}", secret_key),//hex::encode(secret_key as [u8;32]),
            "public_key": format!("{}", public_key)//hex::encode(public_key as [u8;32])
        }
    )
}


fn main() {
    if let Err(err) = run() {
        eprintln!("error starting server: {}", err)
    }
}
