use std::pin::Pin;
use std::time::{Duration, Instant};
use std::thread;
use trpl::Either;
// fn main() {
//     trpl::run(async {
//         let (tx, mut rx) = trpl::channel();
//         let tx2 = tx.clone();                                                
                                                                                
//         let tx1_fut = async move {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];
//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let tx2_fut = async move {
//             let vals = vec![
//                 String::from("more"),
//                 String::from("messages"),
//                 String::from("for"),
//                 String::from("you"),
//             ];
//             for val in vals {
//                 tx2.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(1500)).await;
//             }
//         };

//         let rx_fut = async {
//             while let Some(value) = rx.recv().await {
//                 println!("received '{value}'");
//             }
//         };

//         let futures: Vec<Pin<Box<dyn Future<Output=()>>>> = 
//             vec![Box::pin(tx1_fut), Box::pin(tx2_fut), Box::pin(rx_fut)];
//         trpl::join_all(futures).await;
//     })
// }

// fn main() {
//     trpl::run(async {
//         let slow = async {
//             println!("'slow' started.");
//             trpl::sleep(Duration::from_millis(100)).await;
//             println!("'slow' finished.");
//         };

//         let fast = async {
//             println!("'fast' started.");
//             trpl::sleep(Duration::from_millis(50)).await;
//             println!("'fast' finished");
//         };

//         trpl::race(slow, fast).await;
//     })
// }

// yielding control to the Runtime
// fn slow(name: &str, ms: u64) {
//     thread::sleep(Duration::from_millis(ms));
//     println!("'{name}' ran for {ms}ms");
// }
// fn main() {
//     trpl::run(async {
//         let one_ms = Duration::from_millis(1);

//         let a = async {
//             println!("'a' started.");
//             slow("a", 30);
//             trpl::yield_now().await;
//             slow("a", 10);
//             trpl::yield_now().await;
//             slow("a", 20);
//             trpl::yield_now().await;
//             println!("'a' finished");
//         };
//             let b = async {
//             println!("'b' started.");
//             slow("b", 75);
//             trpl::yield_now().await;
//             slow("b", 10);
//             trpl::yield_now().await;
//             slow("b", 15);
//             trpl::yield_now().await;
//             slow("b", 350);
//             trpl::yield_now().await;
//             println!("'b' finished");
//         };

//         trpl::race(a, b).await;
//     })
// }

// Simple benchmark
// fn main() {
//     trpl::run(async {
//         let one_ns = Duration::from_nanos(1);
//         let start = Instant::now();
//         async {
//             for _ in 1..1000 {
//                 trpl::sleep(one_ns).await;
//             }
//         }
//         .await;
//         let time = Instant::now() - start;
//         println!(
//             "'sleep' version finished after {} seconds",
//             time.as_secs_f32()
//         );

//         let start = Instant::now();
//         async {
//             for _ in 1..1000{
//                 trpl::yield_now().await;
//             }
//         }
//         .await;
//         let time = Instant::now() - start;
//         println!(
//             "'yield' version finished after {} seconds.",
//             time.as_secs_f32()
//         );

//     })
// }

fn main(){
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "I finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    })
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration>{
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time)
    }
}