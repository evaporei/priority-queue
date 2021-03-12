use std::collections::BTreeMap;

trait PriorityQueue<T> {
    fn push(&mut self, item: T, priority: u8);
    fn pop(&mut self) -> Option<T>;
}

#[derive(Debug)]
struct Queue<T> {
    items: BTreeMap<u8, Vec<T>>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }
}

impl<T> PriorityQueue<T> for Queue<T> {
    fn push(&mut self, item: T, priority: u8) {
        let priority_vec = self.items.entry(priority).or_insert(vec![]);

        (*priority_vec).push(item);
    }
    fn pop(&mut self) -> Option<T> {
        let key = self.items.keys().rev().next().cloned();

        key.map(|priority| {
            let list = self.items.get_mut(&priority).unwrap();
            let item = list.pop().unwrap();

            if list.len() == 0 {
                self.items.remove(&priority);
            }

            item
        })
    }
}

fn main() {
    let mut queue = Queue::new();

    queue.push("Jannis".to_string(), 1);
    queue.push("Leo".to_string(), 1);
    queue.push("David".to_string(), 1);

    queue.push("Otávio".to_string(), 2);
    queue.push("Nathália".to_string(), 2);
    queue.push("Isa".to_string(), 2);

    queue.push("Panda".to_string(), 5);

    queue.push("Cat".to_string(), 6);
    queue.push("Dog".to_string(), 6);

    println!("{:#?}", queue);

    for _ in 0..100 {
        let item = queue.pop();
        println!("{:#?}", item);
    }
}
