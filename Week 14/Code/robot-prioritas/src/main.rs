use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Eq, PartialEq)]
struct Task {
    priority: usize,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority) // Higher priority comes first
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut task_queue = BinaryHeap::new();

    task_queue.push(Task {
        priority: 2,
        description: String::from("Inspect machinery"),
    });
    task_queue.push(Task {
        priority: 1,
        description: String::from("Refill fuel"),
    });
    task_queue.push(Task {
        priority: 3,
        description: String::from("Respond to emergency signal"),
    });

    println!("Robot Task Scheduler:");
    while let Some(task) = task_queue.pop() {
        println!("Executing task with priority {}: {}", task.priority, task.description);
    }
}
