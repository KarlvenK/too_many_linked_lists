use std::mem;
// A:
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }

//B:
// pub enum List {
//     Empty,
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }

//C: compile error
// struct Node {
//     elem: i32,
//     next: List,
// }
// pub enum List {
//     Empty,
//     More(Box<Node>),
// }

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            //next: self.head, //error
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // match self.head { //error
        //     Link::Empty => None,
        //     Link::More(ref node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = cur_link {
            cur_link = mem::replace(&mut node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_1() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
