use std::{
    sync::mpsc::{Receiver, Sender},
    process::exit,
};

pub struct Frontend {
    rx_controller: Receiver<String>,
    tx_controller: Sender<String>,
}

impl Frontend {
    pub fn init(tx: Sender<String>, rx: Receiver<String>) -> Frontend {
        Frontend {
            rx_controller: rx,
            tx_controller: tx,
        }
    }

    pub fn receive(&mut self) {
        loop {
            let mut data = String::new();
            let mut _len = std::io::stdin().read_line(&mut data).unwrap();

            if data.eq("\n")
            {
                exit(0);
            }

            self.tx_controller.send(String::from("frontend ") + data.strip_suffix("\n").unwrap()).unwrap();

            loop {
                match &self.rx_controller.try_recv() {
                    Ok(message) => {
                        println!("current value: {message}");
                        break;
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn send(&self, message: &str) {
        self.tx_controller.send(message.to_owned()).unwrap();
    }
}
