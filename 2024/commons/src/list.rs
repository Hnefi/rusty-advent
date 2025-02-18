use std::mem::replace;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn move_all_elements_out(&mut self) {
        // move out of the head first, and then the node will be auto-dropped when the
        // function scope ends
        let mut dropped_head = replace(&mut self.head, None);
        while let Some(mut dropped_next) = dropped_head {
            dropped_head = replace(&mut dropped_next.next, None);
        }
    }

    fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn push_front(&mut self, elem: T) {
        let new = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new);
        self.len += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        // Return the elem in the head (moved out).
        // Set the current head to the old head's "next".
        if let Some(popped) = self.head.take() {
            assert!(self.head.is_none());
            self.head = popped.next;
            self.len -= 1;
            Some(popped.elem)
        } else {
            None
        }
    }

    fn front(&self) -> Option<&T> {
        // returns a reference to the list head element
        match self.head.as_ref() {
            Some(x) => Some(&x.elem),
            None => None,
        }
    }

    fn front_mut(&mut self) -> Option<&mut T> {
        // return an immutable reference to the list head
        match self.head.as_mut() {
            Some(x) => Some(&mut x.as_mut().elem),
            None => None,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn clear(&mut self) {
        // Iteratively move all elements out of "head", the same as the implementation of drop().
        self.move_all_elements_out();
        self.len = 0;
    }
}

impl<T> List<T>
where
    T: PartialEq,
{
    // Implementation for types that require PartialEq on the list generic
    fn contains(&self, e: &T) -> bool {
        let mut cur = &self.head;
        while let Some(node) = cur {
            if node.elem == *e {
                return true;
            }
            cur = &node.next;
        }
        false
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        self.move_all_elements_out();
    }
}

#[cfg(test)]
mod test {

    use super::List;
    #[test]
    fn test_new_basic() {
        let _list: List<i32> = List::new();
        let list: List<List<i32>> = List::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_front() {
        let mut list: List<i32> = List::new();
        list.push_front(1);
        assert_eq!(list.head.as_ref().unwrap().elem, 1);
        list.push_front(2);
        assert_eq!(list.head.as_ref().unwrap().elem, 2);
        assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().elem, 1);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_pop_front() {
        let mut list: List<i32> = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front().unwrap(), 3);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front().unwrap(), 2);
        assert_eq!(list.pop_front().unwrap(), 1);
        assert!(list.pop_front().is_none());
        assert!(list.pop_front().is_none());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_front_reference() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.front(), None);
        list.push_front(1);
        assert_eq!(list.front(), Some(&1));
    }

    #[test]
    fn test_front_mut_reference() {
        let mut list: List<i32> = List::new();
        // check calling front_mut gives None
        assert_eq!(list.front_mut(), None);
        list.push_front(1);
        assert_eq!(list.front_mut(), Some(&mut 1));
        *list.front_mut().unwrap() = 2;
        assert_eq!(list.front_mut(), Some(&mut 2));
    }

    #[test]
    fn test_clear_and_is_empty() {
        let mut list: List<String> = List::new();
        assert!(list.is_empty());
        list.push_front("test".to_string());
        list.push_front("clear".to_string());
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
        assert_eq!(list.front(), Some(&"clear".to_string()));
        list.clear();
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_contains() {
        let mut list: List<String> = List::new();
        list.push_front("test".to_string());
        list.push_front("clear".to_string());
        assert!(list.contains(&"test".to_string()));
        assert!(!list.contains(&"fail".to_string()));
        list.push_front("secondary".to_string());
        assert!(list.contains(&"secondary".to_string()));
    }
}
