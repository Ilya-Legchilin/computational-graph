use std::rc::Rc;
use std::cell::RefCell;

// round to decimal digits
fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}

struct Node {
    op: Operation,
    value: f32,
    edges: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(op: Operation) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            op: op,
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
            match &self.op {
                Operation::Mul => return self.edges[0].borrow().compute() * self.edges[1].borrow().compute(),
                Operation::Add => return self.edges[0].borrow().compute() + self.edges[1].borrow().compute(),
                Operation::Sin => return self.edges[0].borrow().compute().sin(),
                Operation::Pow(exp) => return self.edges[0].borrow().compute().powf(*exp),
                _ => return self.value,
            }
        }
    }
}

#[derive(PartialEq)]
enum Operation {
    Input(String),
    Add,
    Mul,
    Sin,
    Pow(f32),
}

fn create_input(name: &'static str) -> Rc<RefCell<Node>> {
    Node::new(Operation::Input(name.to_string()))
}

fn pow_f32(input: Rc<RefCell<Node>>, exp: f32) -> Rc<RefCell<Node>> {
    let node = Node::new(Operation::Pow(exp));
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input.clone());
    }
    node
}

fn sin(input: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(Operation::Sin);
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input.clone());
    }
    node
}

fn mul(input_one: Rc<RefCell<Node>>, input_two: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(Operation::Mul);
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input_one.clone());
        node_borrowed.edges.push(input_two.clone());
    }
    node
}

fn add(input_one: Rc<RefCell<Node>>, input_two: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(Operation::Add);
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input_one.clone());
        node_borrowed.edges.push(input_two.clone());
    }
    node
}

fn main() {
    // // x1, x2, x3 are input nodes of the computational graph:
    let x1 = create_input("x1");
    let x2 = create_input("x2");
    let x3 = create_input("x3");
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
    println!("x1 = {}", x1.borrow().value);
    println!("x2 = {}", x2.borrow().value);
    println!("x3 = {}", x3.borrow().value);
    let mut result = graph.borrow().compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.32727);
    x1.borrow_mut().set(2f32);
    x2.borrow_mut().set(3f32);
    x3.borrow_mut().set(4f32);
    println!("x1 = {}", x1.borrow().value);
    println!("x2 = {}", x2.borrow().value);
    println!("x3 = {}", x3.borrow().value);
    result = graph.borrow().compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.56656);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn create_input_test() {
        let input = create_input("ABCD");
        assert!(
            match &input.borrow().op {
                Operation::Input(name) => { if name == "ABCD" { true } else { false } },
                _ => false,
            }
        );
    }
}
