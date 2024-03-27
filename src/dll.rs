use std::rc::{Rc, Weak};
use std::cell::RefCell;

type StrongLink<T> = Rc<RefCell<Node<T>>>;
type WeakLink<T> = Weak<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T: Clone>
{
    data: T,
    next: Option<StrongLink<T>>,
    prev: Option<WeakLink<T>>
}

impl<T: Clone> Node<T>
{
    pub fn new(data: T) -> Node<T>
    {
        Node{ data, next: None, prev: None}
    }
}

#[derive(Debug)]
pub struct List<T: Clone> 
{
    head: Option<StrongLink<T>>,
    tail: Option<StrongLink<T>>,
    size: i32
}

impl<T: Clone> List<T>
{
    pub fn new() -> List<T>
    {
        List {head: None, tail: None, size: 0}
    }

    /// Retrieves the size of the list
    pub fn size(&self) -> i32
    {
        self.size
    }

    /// Pushes a value to the back of the list, returns
    /// the new size of the list
    pub fn push_back(&mut self, data: T) -> i32
    {
        let new_node = Rc::new(RefCell::new(Node::new(data)));
        if let Some(tail) = self.tail.take() {
           
                new_node.borrow_mut().prev = Some(Rc::downgrade(&tail));
                tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            
        } else {// empty list
            self.tail = Some(new_node.clone());
            self.head = Some(new_node.clone());
        }
        self.size += 1;

        self.size
    }

    /// adds data to the front of the list returning
    /// the new size of the list
    pub fn push_front(&mut self, data: T) -> i32
    {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        // take current head if it exists
        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            new_node.borrow_mut().next = Some(head);
            self.head = Some(new_node);
        } else {
            self.tail = Some(new_node.clone());
            self.head = Some(new_node.clone());
        }
        self.size += 1;
        self.size
    }

    /// returns the first element in the list, properly updating
    /// the links
    pub fn pop_front(&mut self) -> Option<T>
    {
        if let Some(head) = self.head.take()
        {
            // get rid of the prev link for new head
            if let Some(next) = &mut head.borrow_mut().next {
                next.borrow_mut().prev = None;
                self.head = Some(next.clone());
            }
            self.size -= 1;
            // return a clone of the data
            return head.borrow().data.clone().into()
        }

        None
    }

    /// returns the last element in the list, properly updating the links
    pub fn pop_back(&mut self) -> Option<T> {
        let tail_node = self.tail.take();
        match tail_node {
            None => None,
            Some(tail) => {
                let prev_link = tail.borrow_mut().prev.take();
                if prev_link.is_none() {
                    self.head.take();
                }
                
                if let Some(prev) = prev_link.and_then(|weak| weak.upgrade()) {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                }
                
                self.size -= 1;
                tail.borrow().data.clone().into()
            }
        }
    }
    

    /// Gets a clone of the last value in the list without
    /// modifying the actual data of the list
    /// ** performs a clone of the data ***
    pub fn peek_back(&self) -> Option<T>
    {
        // if the list is not empty then tail should have a value
        if let Some(tail) = &self.tail {
            
            Some(tail.borrow().data.clone())
            
        } else { // list is emtpy
            None
        }
    }

    /// Gets a clone of the first value in the list without
    /// modifying the actual data of the list
    /// ** performs a clone of the data ***
    pub fn peek_front(&self) -> Option<T>
    {
        // if the list is not empty then tail should have a value
        if let Some(head) = &self.head {
            Some(head.borrow().data.clone())
        } else { // list is emtpy
            None
        }
    }
}


