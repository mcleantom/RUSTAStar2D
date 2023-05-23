extern crate rustastar2d;
use rustastar2d::node::Node;

fn main() {
    let n1 = Node::new(0, 0.0, 0);
    println!("Hello, world!");
    println!("n1.idx = {}", n1.idx);
    let n2 = Node::new(1, 1.0, 1);
    let n2_greater_than_n1 = n2 > n1;
    println!("n2_greater_than_n1 = {}", n2_greater_than_n1);
}
