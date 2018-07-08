fn main() {
  // explicitly defining vector by type
  let mut v: Vec<i32> = Vec::new();
  // implicitly defining vector with macro provided by rust
  let v2 = vec![1, 2, 3];
  // adding new values to vector
  v.push(1);
  v.push(2);
  v.push(3);
  v2.get(4);

  // accessing a vector value
  let third: &i32 = &v[2];
  let third2: Option<&i32> = v.get(2);
  println!("[+] third element of v: {:?}", third);
  println!("[+] third element of v: {:?}", third2);

  // accessing a non-existant index
  let v3 = vec![1, 2, 3, 4, 5];
  
  //let nonex = &v3[100];
  //println!("[+] nonex: {:?}", nonex);
  //  thread 'main' panicked at 'index out of bounds: the len is 5 but the 
  //  index is 100',

  // returns None
  let nonex2 = v3.get(100);
  println!("[+] nonex2: {:?}", nonex2);

  // iterating a vector
  let iv = vec![1, 2, 3, 4];
  println!("[*] iterating over vector {:?}", iv);
  for v in &iv {
    println!("  [*] value: {:?}", v);
  }
  // iterating over, and mutating, a vector
  let mut mv = vec![1, 2, 3, 4];
  println!("[*] squaring vector {:?}", mv);
  for v in &mut mv {
    *v += *v;
    println!("  [*] new value {:?}", v);
  }

  // we can utilize enums with vectors in order to hold items of different
  // types in a vector
  #[derive(Debug)]
  enum DifferentTypes {
    Int(i32),
    Float(f64),
    Text(String),
  }
  let combined = vec![
    DifferentTypes::Int(2),
    DifferentTypes::Float(3.0),
    DifferentTypes::Text(String::from("blue")),
  ];
  println!("[*] vector with different types by using enums:");
  println!("  {:?}", combined);
}






