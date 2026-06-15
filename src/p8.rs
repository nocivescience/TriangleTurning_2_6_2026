use std::time::Duration;

fn main() {
    for _frame in 1..60 {
        println!(
            "{:02}", _frame
        );
        std::thread::sleep(Duration::from_millis(16));
    }
}