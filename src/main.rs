use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Debug, Clone)]
struct Node {
    kind: Kind,
    count: u32,
    has: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>
}

#[derive(Debug, PartialEq, Clone)]
enum Kind {
    Root,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Node {

    fn new(kind: Kind) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node {
            kind,
            count: 0,
            has: Rc::new(RefCell::new(Vec::new()))
        }));
        node
    }

    fn root() -> Rc<RefCell<Node>> {
        Node::new(Kind::Root)
    }

    fn check_node_kind(node: &Node, digit: u8) -> bool {

        if node.kind == Kind::Zero && digit == 0 {
            return true;
        }

        if node.kind == Kind::One && digit == 1 {
            return true;
        }

        if node.kind == Kind::Two && digit == 2 {
            return true;
        }

        if node.kind == Kind::Three && digit == 3 {
            return true;
        }

        if node.kind == Kind::Four && digit == 4 {
            return true;
        }

        if node.kind == Kind::Five && digit == 5 {
            return true;
        }

        if node.kind == Kind::Six && digit == 6 {
            return true;
        }

        if node.kind == Kind::Seven && digit == 7 {
            return true;
        }

        if node.kind == Kind::Eight && digit == 8 {
            return true;
        }

        if node.kind == Kind::Nine && digit == 9 {
            return true;
        }

        false

    }

    fn get_node_kind(digit: u8) -> Kind {

        if digit == 0 {
            return Kind::Zero;
        }

        if digit == 1 {
            return Kind::One;
        }

        if digit == 2 {
            return Kind::Two;
        }

        if digit == 3 {
            return Kind::Three;
        }

        if digit == 4 {
            return Kind::Four;
        }

        if digit == 5 {
            return Kind::Five;
        }

        if digit == 6 {
            return Kind::Six;
        }

        if digit == 7 {
            return Kind::Seven;
        }

        if digit == 8 {
            return Kind::Eight;
        }

        if digit == 9 {
            return Kind::Nine;
        }

        return Kind::Root;

    }

    fn check_has(parent_node: &Rc<RefCell<Node>> , handle: u8) -> Result<(bool, usize), u8> {

        let parent_node = parent_node.deref().borrow();
        let has_list = parent_node.has.deref().borrow();

        let mut has_status: bool;
        let handle_index: usize;

        for (index, child_node) in has_list.iter().enumerate() {

            let node = child_node.deref().borrow();
            let n = node.deref();
            has_status = Node::check_node_kind(n, handle);

            if has_status == true {

                handle_index = index;
                return Ok((has_status, handle_index));

            }

        }

        return Err(handle);

    }

}

fn main() {

    let l0 = Node::root();

    // l0 (level-0) reference count is incremented so that when updating the parent_node,
    // root node wont go out of scope

    let mut parent_node = l0.clone();

    let contact_num = "912234343";
    let buffer = contact_num.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut handle_check: Result<(bool, usize), u8>;

    for handle in buffer {

        // handle is digit
        // Check if the handle is present in "has" Vec

        handle_check = Node::check_has(&parent_node, handle as u8);

        match handle_check {

            // If present
            Ok((_, index)) => {

                // -> update the parent_node with the has node

                let old_parent_node = parent_node.clone();
                parent_node = old_parent_node
                    .deref()
                    .borrow()
                    .has
                    .deref()
                    .borrow()
                    .get(index)
                    .unwrap()
                    .clone();

                // -> Inc parent_node count

                parent_node.borrow_mut().count += 1;

            }

            // If absent 
            Err(handle) => {

                // -> create new child_node 
                let node_kind = Node::get_node_kind(handle);
                let child_node = Node::new(node_kind);

                // -> add the new child_node to the has

                parent_node.deref()
                    .borrow()
                    .has
                    .deref()
                    .borrow_mut()
                    .push(child_node.clone());
                
                // -> update the parent_node with the child_node

                parent_node = child_node;

                // -> Inc parent_node count

                parent_node.borrow_mut().count += 1;

            }
            
        }

    }

    println!("{:#?}", l0);

}
