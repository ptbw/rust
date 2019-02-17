/* 1
fn is_equal<T:std::cmp::PartialEq>(a: T, b: T) -> bool {
  a==b
}
*/

fn is_equal<T>(a: T, b: T) -> bool {
    a == b
}

/* 2
struct Person {
  name: String,
}

impl Person {
  pub fn say_hello(&self) {
    println!("{} says hello!", self.name);
  }
}

*/


fn main() {
  
    // 1
    if is_equal(10, 11) {
        println!("equal");
    } else {
        println!("not equal");
    }
    
    /* 2 
    let p = Person { name: "Bibi".to_string() };
    p.say_hello();
    */
    
}
