use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::*;

pub trait Binding<S> {
    fn get(&self) -> RefMut<'_, S>;
}

pub trait AnyState { }

#[derive(Clone)]
pub struct State<S> {
    value: Rc<RefCell<S>>,
}

impl<S> State<S> {
    pub fn new(value: S) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn set(&self, value: S) {
        *self.value.borrow_mut() = value;
    }
}

impl<S> AnyState for State<S> { }

impl<S> Binding<S> for State<S> {
    fn get(&self) -> RefMut<'_, S> {
        // Here we can indicate that a state change has
        // been made.
        self.value.borrow_mut()
    }
}

pub struct StateView<S: 'static, V: View> {
    default: S,
    func: Box<dyn Fn(State<S>) -> V>,
}

impl<S, V> View for StateView<S, V> where V: View, S: Clone {
    
    fn draw(&self, id: ViewID, cx: &mut Context) {

        // Look up the state in the context.
        let newstate = Box::new(State::new(self.default.clone()));
        let s = cx.state_map.entry(id).or_insert(newstate);

        if let Some(state) = s.downcast_ref::<State<S>>() {
            (*self.func)(state.clone()).draw(id.child(0), cx);
        } else {
            panic!("state has wrong type")
        }

    }

    fn process(&self, event: &Event, id: ViewID, cx: &mut Context) {

        // Look up the state in the context.
        let newstate = Box::new(State::new(self.default.clone()));
        let s = cx.state_map.entry(id).or_insert(newstate);

        if let Some(state) = s.downcast_ref::<State<S>>() {
            (*self.func)(state.clone()).process(event, id.child(0), cx);
        } else {
            panic!("state has wrong type")
        }
        
    }
}

pub fn state<S: Clone, V: View, F: Fn(State<S>) -> V + 'static>(initial: S, f: F) -> StateView<S, V> {
    StateView { default: initial, func: Box::new(f) }
}

