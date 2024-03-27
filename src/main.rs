use dll::dll::List;
use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // ASSERT
    let mut list = List::new();

    // ACT
    list.push_back("first".to_owned());
    list.push_back("second".to_owned());
    list.push_back("third".to_owned());

    println!("{:?}", list);
    
    // ASSERT
    assert_eq!(list.pop_back().unwrap(), "third");
    println!("{:?}", list);
   
}
