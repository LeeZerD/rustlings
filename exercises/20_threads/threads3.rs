use std::sync::mpsc;
use std::sync::Arc;  // Utilisé pour permettre le partage de données entre plusieurs threads de manière sécurisée
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {

    let qc = Arc::new(q);  // Cration d'un Arc autour de Queue

    let qc1 = Arc::clone(&qc);  // clonage Arc pour le premier thread
    let qc2 = Arc::clone(&qc);  // clonage arc pour le deuxième threa

    // Clonage de l'émetteur pour le premier thread
    let tx1 = tx.clone();  

    // Création du premier thread pour envoyer les valeurs de la première moitié
    thread::spawn(move || {
        for val in &qc1.first_half {  // Utilise qc1 pour accéder à la première moitié
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();  // Envoie la valeur via le Sender cloné
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Création du deuxième thread pour envoyer les valeurs de la deuxième moitié
    thread::spawn(move || {
        for val in &qc2.second_half {  // Utilise qc2 pour accéder à la deuxième moitié
            println!("sending {:?}", val);
            tx.send(*val).unwrap();  // Envoie la valeur via le Sender original
            thread::sleep(Duration::from_secs(1));
        }
    });
}

#[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}

