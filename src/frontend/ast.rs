#[derive(std::fmt::Debug)]
pub struct Node {
    op: Op,
    children: Vec<Node>,
    val: String
}

impl Node {
    pub fn binary(op: Op, left: Node, right: Node) -> Node {
        let children = vec![left, right];
        Node { 
            op,
            children,
            val: String::from("") 
        } 
    }

    pub fn unary(op: Op, operand: Node) -> Node {
        let children = vec![operand];
        Node {
            op,
            children,
            val: String::from("") 
        } 
    }

    pub fn standalone(val: &str) -> Node {
        let children = vec![];
        Node {
            op: Op::Literal,
            children,
            val: String::from(val) 
        } 
    }
}

#[derive(std::fmt::Debug)]
pub enum Op {
    Add, Sub, Mul, Div,
    Mod, StrConcat,
    Neg, Literal
}