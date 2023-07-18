//! [`List<T>`]
//!

use std::fmt::Debug;

/// contain any T for list.
#[derive(Debug)]
pub enum List<T> {
    /// Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(T, Box<List<T>>),
    /// Nil：末结点，表明链表结束
    Nil,
}

impl<T: Debug> List<T> {
    // 创建一个空的 List 实例
    pub fn new() -> Self {
        List::Nil
    }

    /// 在其头部插入新元素，并返回该 List
    pub fn prepend(self, elem: T) -> List<T> {
        List::Cons(elem, Box::new(self))
    }

    /// 在其尾部插入新元素，并返回该 List
    pub fn append(self, elem: T) -> List<T> {
        match self {
            List::Cons(_, _) => List::Cons(elem, Box::new(self)),
            List::Nil => self.prepend(elem),
        }
    }

    // 返回 List 的长度
    pub fn len(&self) -> u32 {
        // 取出 List 的值,根据值类型做出不同的处理
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 因此使用一个对 tail 的引用
            List::Cons(_, ref tail) => 1 + tail.len(),
            // （递归的）基准情形（base case）：一个长度为 0 的空列表
            List::Nil => 0,
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    pub fn stringify(&self) -> String {
        match *self {
            List::Cons(ref head, ref tail) => format!("{:?},{}", *head, tail.stringify()),
            List::Nil => format!("Nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 创建一个空链表
        let list = List::<u32>::new();
        assert!(match list {
            List::Cons(_, _) => false,
            List::Nil => true,
        });

        let list = list.prepend(1);
        let list = list.prepend(5);
        // 长度正确
        assert_eq!(list.len(), 2, "list is {:?}", list);
        // 插入正确
        assert_eq!(list.stringify(), "5,1,Nil", "expect {}.", list.stringify());
    }
}
