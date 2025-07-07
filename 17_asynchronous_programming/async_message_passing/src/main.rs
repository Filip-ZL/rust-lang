use std::time::Duration;
// basics
// fn main() {
//     trpl::run(async {
//         let (tx, mut rx) = trpl::channel();

//         let val = String::from("hi");
//         tx.send(val).unwrap();

//         let received = rx.recv().await.unwrap();

//         println!("Got: {received}");
//     })
// }

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        // let tx2 = tx.clone();                                                // also uncomment to demonstrate
                                                                                // multiple receivers!
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // Uncomment the code below to demonstrate multiple producers!
        // let tx_fut2 = async move {
        //     let vals = vec![
        //         String::from("more"),
        //         String::from("messages"),
        //         String::from("for"),
        //         String::from("you"),
        //     ];
        //     for val in vals {
        //         tx2.send(val).unwrap();
        //         trpl::sleep(Duration::from_millis(1500)).await;
        //     }
        // };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join3(tx_fut, tx_fut2, rx_fut).await;
    })
}
