use tokio::{task, time::Instant};


#[tokio::main]
async fn main() {
    seq();

    parallel().await
}

fn seq() {
    let start = Instant::now();
    let numbers: Vec<i64> = (1..=1000).collect();

    let mut result: Vec<f64> = vec![];

    for number in numbers {
        result.push((number as f64).sqrt());
    }

    let duration = start.elapsed();
    println!("Sequential Execution Time: {:?}", duration);
}


async fn parallel() {
    let start = Instant::now();
    let numbers: Vec<i64> = (1..=1000).collect();

    let mut result: Vec<f64> = vec![];
    let mut handles = Vec::new();

    for number in numbers {
        handles.push(task::spawn(async move { (number as f64).sqrt() }));
    }
    
    for handle in handles {
        result.push(handle.await.unwrap());
    }

    let duration = start.elapsed();
    println!("Parallel Execution Time: {:?}", duration);
}