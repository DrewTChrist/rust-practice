use std::mem;

#[derive(Debug, PartialEq)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
    fn new_boxed(data: T) -> Option<Box<Self>> {
        Some(Box::new(Self::new(data)))
    }
}

#[derive(Debug)]
pub struct List<T> {
    root: Option<Box<Node<T>>>,
    len: usize,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug> List<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }
    pub fn _pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.root, None) {
            None => None,
            Some(node) => {
                self.len -= 1;
                self.root = node.next;
                Some(node.data)
            }
        }
    }
    pub fn _push(&mut self, data: T) {
        let mut new_node = Node::new(data);
        new_node.next = mem::replace(&mut self.root, None);
        self.root = Some(Box::new(new_node));
        self.len += 1;
    }
    pub fn append(&mut self, data: T) {
        let new_node = Node::new_boxed(data);
        match self.root {
            None => {
                self.root = new_node;
                self.len += 1;
            }
            Some(ref mut root) => {
                let mut tmp: &mut Node<T> = root;
                loop {
                    tmp = match tmp.next {
                        Some(ref mut next) => next,
                        None => break,
                    };
                }
                tmp.next = new_node;
                self.len += 1;
            }
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref root) => {
                if index >= self.len() {
                    return None;
                }
                let mut tmp: &Node<T> = root;
                let mut count: usize = 0;
                while count < index {
                    tmp = match tmp.next {
                        Some(ref next) => {
                            count += 1;
                            next
                        }
                        None => break,
                    };
                }
                Some(&tmp.data)
            }
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        if let Some(mut next) = self.root.take() {
            while let Some(n) = next.next.take() {
                next = n;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn append1() {
        let mut l = List::new();
        for i in 0..10 {
            l.append(i as i32);
        }
        assert_eq!(l.len(), 10);
    }

    #[test]
    fn get1() {
        let mut l = List::new();
        for i in 0..5 {
            l.append(i as i32);
        }
        assert_eq!(l.get(4), Some(&4_i32));
    }

    #[test]
    fn get2() {
        let mut l = List::new();
        l.append(String::from("hello"));
        l.append(String::from("there"));
        l.append(String::from("foo"));
        l.append(String::from("bar"));
        assert_eq!(l.get(2), Some(&String::from("foo")));
    }
}
