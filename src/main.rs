
use hello_macro::HelloMacro;
use hello_macro_derive::{HelloMacro, sql};

// #[macro_define_crate::my_first_attribute_proc_macro2("mazi")]
// fn add(a: i32, b: i32) { println!("{} + {} = {}", a, b, a + b); }

#[macro_define_crate::my_first_attribute_proc_macro("mazi")]
fn my_test() {
   assert_eq!(1, 2); 
}

macro_rules! vvec {
    ($($x: expr), *) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Mazi;

fn main() {
    // add(1, 3);
    let v: Vec<i32> = vvec![1, 2, 3];
    println!("{:?}", v);
    Pancakes::hello_macro();
    Mazi::hello_macro();
    sql!("SELECT * FROM posts WHERE id = 1");
}
