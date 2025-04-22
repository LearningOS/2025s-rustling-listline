/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    q1: Queue<T>, // 主队列
    q2: Queue<T>, // 辅助队列
}

impl<T> myStack<T> {
    // 创建一个新的栈
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    // 压入一个元素到栈中
    pub fn push(&mut self, elem: T) {
        // 将新元素加入辅助队列 q2
        self.q2.enqueue(elem);

        // 把 q1 中的元素逐个移到 q2 中
        while !self.q1.is_empty() {
            if let Ok(val) = self.q1.dequeue() {
                self.q2.enqueue(val);
            }
        }

        // 交换 q1 和 q2，这样 q1 中就是栈的元素，q2 为空
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    // 弹出栈顶元素
    pub fn pop(&mut self) -> Result<T, &str> {
        // 从 q1 中弹出栈顶元素
        self.q1.dequeue().map_err(|_| "Stack is empty")
    }

    // 查看栈是否为空
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}