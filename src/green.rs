use std::time::Duration;

struct FiboNachos {
    nacho_2: u64,
    nacho_1: u64,
}

#[tokio::main]
async fn main() {
    // fibo nachos
    let (s1, r1) = crossbeam::channel::bounded::<Option<FiboNachos>>(2);
    let (s2, r2) = crossbeam::channel::bounded::<Option<FiboNachos>>(2);
    let (s3, r3) = crossbeam::channel::bounded::<Option<FiboNachos>>(100);
    let s1_clone = s1.clone();
    let r1_clone = r1.clone();
    let s2_clone = s2.clone();
    let r2_clone = r2.clone();
    let s3_clone = s3.clone();

    tokio::task::spawn(async move {
        println!("Producing green Nacho thread with Ketchup");
        r1_clone.iter().take_while(|x| x.is_some()).for_each(|x| {
            if let Some(fibo_nachos) = x {
                std::thread::sleep(Duration::from_secs(1));
                let nacho_1 = fibo_nachos.nacho_1 + fibo_nachos.nacho_2;
                s2_clone.send(Some(FiboNachos { nacho_1, nacho_2: fibo_nachos.nacho_1 })).unwrap();
                s3_clone.send(Some(FiboNachos { nacho_1, nacho_2: fibo_nachos.nacho_1 })).unwrap();
            }
        });
    });

    let s3_clone_2 = s3.clone();
    tokio::task::spawn(async move {
        println!("Producing green Nacho thread with Chili");
        r2_clone.iter().take_while(|x| x.is_some()).for_each(|x| {
            if let Some(fibo_nachos) = x {
                std::thread::sleep(Duration::from_secs(1));
                let nacho_1 = fibo_nachos.nacho_1 + fibo_nachos.nacho_2;
                s1_clone.send(Some(FiboNachos { nacho_1, nacho_2: fibo_nachos.nacho_1 })).unwrap();
                s3_clone_2.send(Some(FiboNachos { nacho_1, nacho_2: fibo_nachos.nacho_1 })).unwrap();
            }
        });
    });

    println!("start");
    s1.send(Some(FiboNachos { nacho_1: 1, nacho_2: 1 })).unwrap();

    r3.iter().for_each(|x| {
        if let Some(FiboNachos { nacho_1, nacho_2 }) = x {
            println!("Producing fibonachos with flavor: Ketchup: {}, Chili: {}", nacho_1, nacho_2);
        }
    });
}