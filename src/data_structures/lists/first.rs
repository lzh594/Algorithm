//! # 最小可用的单向链表(栈)
// 避免内部细节公开，使用零开销抽象
pub struct List {
    head: Link,
}

#[derive(Clone)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    /// 构建 List 的新实例
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    /// 向 List 中插入新元素
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        // unimplemented!()：明确地说明目前的代码还没有实现，一旦代码执行到该的位置，就会发生一个 panic。
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会有无边界的递归发生
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::lists::first::List;

    #[test]
    fn test_basic() {
        let mut list = List::new();
        // check empty
        assert_eq!(list.pop(), None);
        // push
        list.push(1);
        list.push(2);
        list.push(3);
        // check pop
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn test_long() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }

}