use std::time::Duration;

struct Message {
    idx: u64,
    text: &'static str,
}

fn main() {
    let (s1, r1) = crossbeam::channel::bounded::<Message>(2);
    let (s2, r2) = crossbeam::channel::bounded::<Message>(2);

    let handle_1 = std::thread::spawn(move || {
        r1.iter().for_each(move |msg| {
            println!("{} {}", msg.idx, msg.text);
            std::thread::sleep(Duration::from_secs(1));
            s2.send(Message { idx: msg.idx + 1, text: "pong" });
        });
    });

    let s1_clone = s1.clone();
    let handle_2 = std::thread::spawn(move || {
        r2.iter().for_each(|msg| {
            println!("{} {}", msg.idx, msg.text);
            std::thread::sleep(Duration::from_secs(1));
            s1_clone.send(Message { idx: msg.idx + 1, text: "ping" });
        });
    });

    s1.send(Message { idx: 0, text: "ping" });

    vec![handle_1, handle_2].into_iter().for_each(|h| { h.join(); });
}
