struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    fn swap(self) -> Self {
        Self {
            first: self.second,
            second: self.first,
        }
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Task {
    title: String,
    status: String,
    details: String,
}

impl Summary for Task {
    fn summarize(&self) -> String {
        format!("{} ({})", self.title, self.status)
    }
}

fn longest_description<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let tasks = vec![
        Task {
            title: "Review chapter 10".into(),
            status: "done".into(),
            details: "Learn generics, traits, and lifetimes.".into(),
        },
        Task {
            title: "Build iterator demo".into(),
            status: "in progress".into(),
            details: "Use iterators and closures to process task data.".into(),
        },
        Task {
            title: "Write summary".into(),
            status: "todo".into(),
            details: "Collect and display only unfinished tasks.".into(),
        },
    ];

    let pending_titles: Vec<String> = tasks
        .iter()
        .filter(|task| task.status != "done")
        .map(|task| task.title.clone())
        .collect();

    println!("Pending tasks: {pending_titles:?}");

    let summaries: Vec<String> = tasks.iter().map(|task| task.summarize()).collect();
    println!("Task summaries:");
    for line in summaries {
        println!("- {line}");
    }

    let first_detail = &tasks[0].details;
    let third_detail = &tasks[2].details;
    let longer = longest_description(first_detail, third_detail);
    println!("Longer description: {longer}");

    let pair = Pair::new(100, 200);
    let swapped = pair.swap();
    println!(
        "Original pair first=100 second=200, swapped first={} second={}",
        swapped.first, swapped.second
    );

    let numbers = [1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|&n| n * n).collect();
    println!("Squared numbers: {squared:?}");

    // move closure using captured value
    let prefix = String::from("Note:");
    let describe_pending = move |title: &str| format!("{prefix} {title}");
    let notes: Vec<String> = pending_titles
        .iter()
        .map(|title| describe_pending(title))
        .collect();
    println!("Notes for pending tasks: {notes:?}");
}
