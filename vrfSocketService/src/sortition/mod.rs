
use openssl::{
    bn::{BigNum, BigNumContext},
    ec::{EcGroup, EcPoint, PointConversionForm},
    error::ErrorStack,
    hash::{hash, MessageDigest},
    nid::Nid,
};

pub fn sortition()->(){
  println!("output: {:#?}", binomial(3, 100, 0.2));
}

fn combination(m: u32, n: u32) ->f64{
  let mut num : f64 = 1.0;
  let mut count : u32 = 0;
  let mut i = m;
  while i > 0{
    if count == n {
      break;
    }
    
    num = num * i as f64 / (i + n - m) as f64;
    count += 1;
    i -= 1;
  }
  num
}

fn binomial(k: u32, w:u32, p:f32) ->f64{
  let a = combination(w, k);
  let b = p.powi(k as i32);
  let c = (1.0 - p).powi(w as i32 - k as i32);
  a * b as f64 * c as f64
}

fn cumulative(j: u32, w: u32, p: f32) -> f64 {
  let mut sum: f64 = 0.0;
  for i in 0..= j{
    sum = sum + binomial(i, w, p);
  }
  sum
}

fn get_vote(hash: [u8;32], w: u32, p:f32) -> f32{
  std::f32::MAX
}
#[test]
fn just_call_sortition(){
  
  //sortition();

  let biggest = [255 as u8; 32];
  let b2 = [std::u8::MAX; 32];
  println!("{:?}", biggest);
  println!("{:?}",b2);
  
}