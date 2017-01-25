use std::rc::Rc;
use std::cell::{RefCell};

#[derive(Debug, Clone)]
struct RcThing {
    name: String,
    pointer: RefCell<Option<Rc<RcThing>>>,
}

impl RcThing {
    fn new_empty(name: &str) -> Self {
        println!("{:?} created!", name);
        RcThing {
            name: name.into(),
            pointer: RefCell::new(None),
        }

    }
}

impl Drop for RcThing {
    fn drop(&mut self) {
        println!("{:?} dropped!", self.name);
    }
}

fn main() {
    let firstone = Rc::new(RcThing::new_empty("First one"));
    let secondone = Rc::new(RcThing::new_empty("Second one"));
    let _ = Rc::new(RcThing::new_empty("Third one"));
    let help = secondone.clone();
    *firstone.pointer.borrow_mut() = Some(help);
    *secondone.pointer.borrow_mut() = Some(firstone);
}
