use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct NodeSingle {
    pub val: i32,
    pub next: Option<Box<NodeSingle>>,
}

#[derive(Debug)]
pub struct NodeDouble {
    pub val: i32,
    pub pre: Option<Box<NodeDouble>>,
    pub next: Option<Box<NodeDouble>>,
}

#[derive(Debug)]
pub struct NodeSingleRef {
    pub val: i32,
    pub next: Option<Rc<RefCell<NodeSingleRef>>>,
}

#[derive(Debug)]
pub struct NodeDoubleRef {
    pub val: i32,
    pub pre: Option<Rc<RefCell<NodeDoubleRef>>>,
    pub next: Option<Rc<RefCell<NodeDoubleRef>>>,
}
