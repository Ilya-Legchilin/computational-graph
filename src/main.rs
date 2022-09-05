// round to decimal digits
fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}

fn main() {
    // // x1, x2, x3 are input nodes of the computational graph:
    let mut x1 = create_input("x1");
    let mut x2 = create_input("x2");
    let mut x3 = create_input("x3");
    // graph variable is the output node of the graph:
    let graph = add(
        x1.clone(),
        mul(
            x2.clone(),
            sin(
                add(
                    x2.clone(),
                    pow_f32(
                        x3.clone(),
                        3f32
                    )
                )
            )
        )
    );
    x1.borrow_mut().set(1f32);
    x2.borrow_mut().set(2f32);
    x3.borrow_mut().set(3f32);
    println!("x1 = {}", x1.borrow().value.unwrap());
    println!("x2 = {}", x2.borrow().value.unwrap());
    println!("x3 = {}", x3.borrow().value.unwrap());
    // let mut result = graph.compute();
    // result = round(result, 5);
    // println!("Graph output = {}", result);
    // assert_eq!(round(result, 5), -0.32727);
    x1.borrow_mut().set(2f32);
    x2.borrow_mut().set(3f32);
    x3.borrow_mut().set(4f32);
    println!("x1 = {}", x1.borrow().value.unwrap());
    println!("x2 = {}", x2.borrow().value.unwrap());
    println!("x3 = {}", x3.borrow().value.unwrap());
    // result = graph.compute();
    // result = round(result, 5);
    // println!("Graph output = {}", result);
    // assert_eq!(round(result, 5), -0.56656);
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

struct Node {
    datum: Datum,
    value: f32,
    edges: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(datum: Datum) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            datum: datum,
            value: 0f32,
            edges: Vec::new(),
        }))
    }

    fn set(&mut self, value: f32) {
        self.value = value;
    }

    fn compute(&self) -> f32 {
        if self.edges.len() == 0 {
            return self.value;
        } else {
            match self.datum {
                Datum::Mul => return self.edges[0].borrow().value * self.edges[1].borrow().value,
                Datum::Add => return self.edges[0].borrow().value + self.edges[1].borrow().value,
                Datum::Sin => return self.edges[0].borrow().value.sin(),
                
            }

        }
    }

    // fn traverse<F>(&self, f: &F, seen: &mut HashSet<Datum>)
    //     where F: Fn(Datum)
    // {
    //     if seen.contains(&self.datum) {
    //         return;
    //     }
    //     f(self.datum);
    //     seen.insert(self.datum);
    //     for n in &self.edges {
    //         n.borrow().traverse(f, seen);
    //     }
    // }

    fn first(&self) -> Rc<RefCell<Node>> {
        self.edges[0].clone()
    }
}

// fn foo(node: &Node) {
//     println!("foo: {}", node.datum);
// }

// fn init() -> Rc<RefCell<Node>> {
//     let root = Node::new("A");

//     let b = Node::new("B");
//     let c = Node::new("C");
//     let d = Node::new("D");
//     let e = Node::new("E");
//     let f = Node::new("F");

//     {
//         let mut mut_root = root.borrow_mut();
//         mut_root.edges.push(b.clone());
//         mut_root.edges.push(c.clone());
//         mut_root.edges.push(d.clone());

//         let mut mut_c = c.borrow_mut();
//         mut_c.edges.push(e.clone());
//         mut_c.edges.push(f.clone());
//         mut_c.edges.push(root.clone());
//     }

//     root
// }

// pub fn main() {
//     let g = init();
//     let g = g.borrow();
//     g.traverse(&|d| println!("{}", d), &mut HashSet::new());
//     let f = g.first();
//     foo(&*f.borrow());
// }

#[derive(PartialEq)]
enum Datum {
    Input(String),
    Add,
    Mul,
    Sin,
    Pow(f32),
}

fn create_input(name: &'static str) -> Rc<RefCell<Node>> {
    Node::new(Datum::Input(name.to_string()))
}

fn pow_f32(input: Rc<RefCell<Node>>, exp: f32) -> Rc<RefCell<Node>> {
    let node = Node::new(Datum::Pow(exp));
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input.clone());
    }
    node
}

fn sin(input: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(Datum::Sin);
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input.clone());
    }
    node
}

fn mul(input_one: Rc<RefCell<Node>>, input_two: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(Datum::Mul);
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input_one.clone());
        node_borrowed.edges.push(input_two.clone());
    }
    node
}

fn add(input_one: Rc<RefCell<Node>>, input_two: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(Datum::Add);
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input_one.clone());
        node_borrowed.edges.push(input_two.clone());
    }
    node
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn create_input_test() {
        let input = create_input("ABCD");
        assert!(
            match &input.borrow().datum {
                Datum::Input(name) => { if name == "ABCD" { true } else { false } },
                _ => false,
            }
        );
    }
}
