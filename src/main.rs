pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    count: i32,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, count: 0 }
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn add(&mut self, value: T) {

        // создаем новую ноду
        let new_node = Box::new(Node {
            data: value,
            next: None,
        });

        self.count = self.count + 1;

        if self.head.is_none() {
            // Если очередь пуста
            self.head = Some(new_node);
        } else {
            // Находим последний элемент и добавляем новый узел
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(new_node);
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        self.head.take().map(|mut head| {
            self.head = head.next;
            head.data
        })
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
}



fn main() {
    let _node = Node :: new(5);
    let mut _que = Queue::new();
    _que.add(5);
    _que.add(5);
    _que.add(5);
    println!("{}", _que.count); 
    println!("Hello, world!");
}
