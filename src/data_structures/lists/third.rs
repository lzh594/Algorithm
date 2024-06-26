//! # 不可变的、共享所有权的持久化链表
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}


type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    /// 构建 List 的新实例
    pub fn new() -> Self {
        List { head: None }
    }
    /// 在原链表头加入新结点，并返回新链表
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            }))
        }
    }
    /// 现有链表的首个元素移除，并返回剩余的链表
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| { node.next.clone() })
        }
    }
    /// 返回首个元素的引用(peek)
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| { &node.elem })
    }
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            // 判断当前的 Rc 是否只有一个强引用，若是，则返回 Rc 持有的值，否则返回一个错误。
            if let Ok(mut node) = Rc::try_unwrap(node){
                head = node.next.take();
            }else {
                break;
            }

        };
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // self.0.pop()
        todo!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
    #[test]
    fn test_iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

}
