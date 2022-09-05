use std::cell::RefCell;
use std::rc::Rc;

const NODE_VALUE_DEFAULT: f32 = 0f32;

// round to decimal digits
fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}

struct Node {
    op: Operation,
    value: f32,
    edges: Vec<Rc<RefCell<Node>>>,
    dependencies: Vec<Rc<RefCell<Node>>>,
    cache: Vec<(Vec<f32>, f32)>,
}

impl Node {
    fn new(op: Operation) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node {
            op: op.clone(),
            value: NODE_VALUE_DEFAULT,
            edges: Vec::new(),
            dependencies: Vec::new(),
            cache: Vec::new(),
        }));
        node
    }

    fn set(&mut self, value: f32) {
        self.value = value;
    }

    fn compute(&self) -> f32 {
        // chech cache
        for tuple in &self.cache {
            let mut is_cached = true;
            for (i, elem) in tuple.0.iter().enumerate() {
                if elem != &self.dependencies[i].borrow().value {
                    is_cached = false;
                    break;
                }
            }
            if is_cached {
                return tuple.1;
            }
        }
        if self.edges.len() == 0 {
            let mut vec = Vec::new();
            for element in &self.dependencies {
                vec.push(element.borrow().value);
            }
            return self.value;
        } else {
            match &self.op {
                Operation::Mul => {
                    return self.edges[0].borrow().compute() * self.edges[1].borrow().compute()
                }
                Operation::Add => {
                    return self.edges[0].borrow().compute() + self.edges[1].borrow().compute()
                }
                Operation::Sin => return self.edges[0].borrow_mut().compute().sin(),
                Operation::Pow(exp) => return self.edges[0].borrow_mut().compute().powf(*exp),
                _ => return self.value,
            }
        }
    }
}

#[derive(PartialEq, Clone)]
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
        match &input.borrow().op {
            Operation::Input(_) => node_borrowed.dependencies.push(input.clone()),
            _ => node_borrowed
                .dependencies
                .extend(input.borrow().dependencies.clone()),
        };
    }
    node
}

fn sin(input: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(Operation::Sin);
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input.clone());
        match &input.borrow().op {
            Operation::Input(_) => node_borrowed.dependencies.push(input.clone()),
            _ => node_borrowed
                .dependencies
                .extend(input.borrow().dependencies.clone()),
        };
    }
    node
}

fn mul(input_one: Rc<RefCell<Node>>, input_two: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(Operation::Mul);
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input_one.clone());
        node_borrowed.edges.push(input_two.clone());
        match &input_one.borrow().op {
            Operation::Input(_) => node_borrowed.dependencies.push(input_one.clone()),
            _ => node_borrowed
                .dependencies
                .extend(input_one.borrow().dependencies.clone()),
        };
        match &input_two.borrow().op {
            Operation::Input(_) => node_borrowed.dependencies.push(input_two.clone()),
            _ => node_borrowed
                .dependencies
                .extend(input_two.borrow().dependencies.clone()),
        };
    }
    node
}

fn add(input_one: Rc<RefCell<Node>>, input_two: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(Operation::Add);
    {
        let mut node_borrowed = node.borrow_mut();
        node_borrowed.edges.push(input_one.clone());
        node_borrowed.edges.push(input_two.clone());
        match &input_one.borrow().op {
            Operation::Input(_) => node_borrowed.dependencies.push(input_one.clone()),
            _ => node_borrowed
                .dependencies
                .extend(input_one.borrow().dependencies.clone()),
        };
        match &input_two.borrow().op {
            Operation::Input(_) => node_borrowed.dependencies.push(input_two.clone()),
            _ => node_borrowed
                .dependencies
                .extend(input_two.borrow().dependencies.clone()),
        };
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
        mul(x2.clone(), sin(add(x2.clone(), pow_f32(x3.clone(), 3f32)))),
    );
    x1.borrow_mut().set(1f32);
    x2.borrow_mut().set(2f32);
    x3.borrow_mut().set(3f32);
    let mut result = graph.borrow_mut().compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.32727);
    x1.borrow_mut().set(2f32);
    x2.borrow_mut().set(3f32);
    x3.borrow_mut().set(4f32);
    result = graph.borrow_mut().compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.56656);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn node_new_test() {
        let node = Node::new(Operation::Input("x1".to_string()));
        assert!(match &node.borrow().op {
            Operation::Input(name) => {
                if name == "x1" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        });
        assert!(node.borrow().value == NODE_VALUE_DEFAULT);
        assert!(node.borrow().edges.len() == 0);
    }

    #[test]
    fn node_set_test() {
        let node = Node::new(Operation::Input("x1".to_string()));
        node.borrow_mut().set(10f32);
        assert!(match &node.borrow().op {
            Operation::Input(name) => {
                if name == "x1" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        });
        assert!(node.borrow().value == 10f32);
        assert!(node.borrow().edges.len() == 0);
    }

    #[test]
    fn node_compute_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let x3 = create_input("x3");
        let x4 = create_input("x4");
        let x5 = create_input("x5");
        // graph variable is the output node of the graph:
        let graph = add(
            mul(sin(x1.clone()), pow_f32(x2.clone(), 3f32)),
            mul(add(x3.clone(), x4.clone()), x5.clone()),
        );
        x1.borrow_mut().set(1f32);
        x2.borrow_mut().set(2f32);
        x3.borrow_mut().set(3f32);
        x4.borrow_mut().set(4f32);
        x5.borrow_mut().set(5f32);

        let mut result = graph.borrow().compute();
        assert_eq!(round(result, 5), 41.73177);

        x1.borrow_mut().set(2f32);
        x2.borrow_mut().set(3f32);
        x3.borrow_mut().set(4f32);
        x4.borrow_mut().set(5f32);
        x5.borrow_mut().set(6f32);

        result = graph.borrow().compute();
        assert_eq!(round(result, 5), 78.55103);
    }

    #[test]
    fn create_input_test() {
        let input = create_input("ABCD");
        assert!(match &input.borrow().op {
            Operation::Input(name) => {
                if name == "ABCD" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        });
    }

    #[test]
    fn pow_f32_test() {
        let x1 = create_input("x1");
        let node = pow_f32(x1.clone(), 10f32);
        assert!(match &node.borrow().op {
            Operation::Pow(_exp) => true,
            _ => false,
        });
        assert!(node.borrow().value == NODE_VALUE_DEFAULT);
        assert!(node.borrow().edges.len() == 1);
    }

    #[test]
    fn add_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let node = add(x1.clone(), x2.clone());
        assert!(match &node.borrow().op {
            Operation::Add => true,
            _ => false,
        });
        assert!(node.borrow().value == NODE_VALUE_DEFAULT);
        assert!(node.borrow().edges.len() == 2);
    }

    #[test]
    fn mul_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let node = mul(x1.clone(), x2.clone());
        assert!(match &node.borrow().op {
            Operation::Mul => true,
            _ => false,
        });
        assert!(node.borrow().value == NODE_VALUE_DEFAULT);
        assert!(node.borrow().edges.len() == 2);
    }

    #[test]
    fn sin_test() {
        let x1 = create_input("x1");
        let node = sin(x1.clone());
        assert!(match &node.borrow().op {
            Operation::Sin => true,
            _ => false,
        });
        assert!(node.borrow().value == NODE_VALUE_DEFAULT);
        assert!(node.borrow().edges.len() == 1);
    }
}
