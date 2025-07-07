use std::time::Duration;

// fn main() {
//     trpl::run(async {
//         let handle = trpl::spawn_task(async {
//             for i in 1..10 {
//                 println!("Hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         });

//         for i in 1..5 {
//             println!("Hi number {i} from the second task!");
//             trpl::sleep(Duration::from_millis(500)).await;
//         }
//         // Similar as with the threads. We can use the join to await the
//         // handle task to finish.
//         handle.await.unwrap();        
//     })
// }

// Joining futures
fn main() {
    trpl::run(async {

        let fut1 = async {
            for i in 1..10{
                println!("Hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        // fut1.await;                                                          // Connected with 2nd task
        let fut2 =  async {
            for i in 1..5 {
                println!("Hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        // fut2.await;                                                          // Connected with 2nd task
        // fut1.await;                                                          // Connected with 3rd task

        // trpl::join(fut1, fut2).await;                                        // uncomment for default state
    })
}