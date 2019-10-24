use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::identities::Zero;
use num_traits::cast::FromPrimitive;
use num_rational::Ratio;
use num_traits::identities::One;
// pub fn sortition(pi: [u8;32], hash: [u8;32], num_of_commitee : BigUint, this_user_own_coin : BigUint, total_coins : &BigUint)->([u8;32], [u8;32], u32){
//   let chance_ratio = (num_of_commitee / total_coins).to_f64().unwrap();

//   let j = get_vote(hash, this_user_own_coin, chance_ratio);
//   (hash, pi, j)
// }

fn combination(m: BigUint, n: u32) ->f64{
  let mut num : Ratio<BigUint>= Ratio::new(BigUint::one(), BigUint::one());
  let mut count : u32 = 0;
  let mut i = m.clone();
  while i > BigUint::zero(){
    if count == n {
      break;
    }
    let change = Ratio::new(i.clone() ,  (i.clone() + n - m.clone()));
    num = num *  change;
    count += 1;
    i = i - BigUint::from_i64(1).unwrap();
  }
  let (up, down) = num.into();
  up.to_f64().unwrap() / down.to_f64().unwrap()
}

fn binomial(k: u32, w:BigUint, p:f64) ->f64{
  let a = combination(w.clone(), k);
  let b = p.powi(k as i32);
  let power : BigUint = w -  BigUint::from_i64(k as i64).unwrap();
  
  let c = (1.0 - p).powi(power.to_i32().unwrap());
  a * b as f64 * c as f64
}

fn cumulative(j: u32, w: BigUint, p: f64) -> f64 {
  let mut sum: f64 = 0.0;
  for i in 0..= j{
    sum = sum + binomial(i, w.clone(), p);
  }
  sum
}

pub fn get_vote(hash_ref: &[u8;32], w: BigUint, p:f64) -> u32{
  let big_hash : BigUint = BigUint::from_bytes_le(hash_ref);
  let maxBuffer : BigUint = BigUint::from_bytes_le(&[std::u8::MAX;32]);
  //println!("hash_ref:{}", big_hash);
  //println!("max_ref:{}", maxBuffer);
  let tmp : Ratio<BigUint> = Ratio::new(big_hash , maxBuffer);
  let (up, down) = tmp.into();
  let value : f64 = up.to_f64().unwrap() / down.to_f64().unwrap();

  //println!("value:{}", value);
  
  let mut j:u32 = 0;
  let mut curr = cumulative(j, w.clone(), p);
  //println!("curr:{}", curr);
  while BigUint::from_u32(j).unwrap() <= w && value >= curr {
    j += 1;
    //println!("in while loop now. j = {}", j);
    let next = cumulative(j, w.clone(), p);
    //println!("before compare value:{}, next:{}", value, next);
    if value < next {
      return j;
    }
    curr = next;
  }
  return 0;
}


pub fn convert_u8_array_to_u8_32(slice: &[u8]) -> &[u8;32] {
    
  let ptr = slice.as_ptr() as *const [u8; 32];
  unsafe { &*ptr }
    
}

fn just_call_sortition(input: &str )->(){
  let hash = hex::decode(input).unwrap();
  
  if hash.len() != 32 {
    panic!("hash is not 32 bytes long");
  }

  let  hash_fixed = convert_u8_array_to_u8_32(&hash);
  println!("hash_fixed to byte:{:?}", hash_fixed);
  let w = BigUint::from_i64(100).unwrap();
  let p : f64 = 0.01;
  let j = get_vote(&hash_fixed, w, p);
  println!("input:{} , j:{}",input, j);
  ()
}

// #[test]
// fn test_5_strings(){
//   just_call_sortition("4ebd3943bcad3cc9aae2fb6a1e3cc22b054446dc8fcb779730425a0721dde55a");
//   just_call_sortition("69ef5c334aa1036fc190cb45b9fd2d2e44063f188e4a9d742fce9621f6caa3fb");
//   just_call_sortition("6c982d915dd5e2c5988d0321e70610f8249e554b6f50c5f67d72912ad0caa09a");
//   just_call_sortition("72f6800f928557ff870e401d2952bb53aab8f0c4ca93bf25c58f79778a782b1f");
//   just_call_sortition("6f092964cb2d1ead540526be744e2f1fa010d56e5451f68f5eb1ffc0fb14e4f6");
// }

// fn test_ratio(){
//   let a:i64 = 10000000000000;
//   let b:i64 = 33333330000000;
//     let tmp : Ratio<BigUint> = Ratio::new(BigUint::from_i64(a).unwrap() , BigUint::from_i64(b).unwrap());
//   let (up, down) = tmp.into();
//   let value : f64 = up.to_f64().unwrap() / down.to_f64().unwrap();

//   println!("value:{}", value);
// }


