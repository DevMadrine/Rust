fn main() {
  let m1 = String::from("hello");
  let m2 = String::from("world");
  greet(&m1, &m2);
   let x = true;
   ownership(x);
//  let s = String::from("hello");
//   let s2 = add_suffix(s);
//   println!("{}", s2);

  // main1();
  aliasing();
}


fn ownership(y:bool){
    if y{
        println!("Ownership is true");
    }

}

// fn main1() {
//   let s = String::from("hello");
//   let s2;
//   let b = false;
//   if b {
//     s2 = s;
//   }
//   println!("{}", s);
// }

fn add_suffix(mut s: String) -> String {
  s.push_str(" world");
  s
}

// fn main1() {
//   let s = String::from("hello");
//   let s2;
//   let b = false;
//   if b {
//     s2 = s;
//   }
//   println!("{}", s);
// }

//Referencing and Borrowing

fn greet(g1:&String, g2:&String){
  println!("{} {}", g1, g2);
}


fn aliasing(){
 let mut s = String::from("hello");
 let t = &mut s;
 t.push_str(" world");
 s = "hello world";
 println!("{}", s);
}
