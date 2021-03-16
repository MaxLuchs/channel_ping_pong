use std::time::Duration;
use std::io::{stdin};

#[derive(Clone, Copy, Debug)]
struct Message {
    idx: u64,
    text: &'static str,
}

fn main() {
    let (s1, r1) = crossbeam::channel::bounded::<Option<Message>>(2);
    let (s2, r2) = crossbeam::channel::bounded::<Option<Message>>(2);

    let s1_clone = s1.clone();
    let s2_clone = s2.clone();
    let handle_1 = std::thread::spawn(move || {
        r1.iter().take_while(|x| x.is_some()).for_each(move |x| {
            if let Some(msg) = x {
                println!("{} {}", msg.idx, msg.text);
                std::thread::sleep(Duration::from_secs(1));
                s2.send(Some(Message { idx: msg.idx + 1, text: "pong" }));
            }
        });
    });

    let handle_2 = std::thread::spawn(move || {
        r2.iter().take_while(|x| x.is_some()).for_each(|x| {
            if let Some(msg) = x {
                println!("{} {}", msg.idx, msg.text);
                std::thread::sleep(Duration::from_secs(1));
                s1_clone.send(Some(Message { idx: msg.idx + 1, text: "ping" }));
            }
        });
    });

    s1.send(Some(Message { idx: 0, text: "ping" }));

    let mut stdin = stdin();
    let mut input = String::new();
    'ui: loop {
        std::thread::sleep(Duration::from_millis(100));
        stdin.read_line(&mut input);
        if input.trim() == "q" {
            println!("{}", "ping pong finished");
            s1.send(None);
            s2_clone.send(None);
            break 'ui;
        }
    }
    vec![handle_1, handle_2].into_iter().for_each(|h| { h.join(); });
}

