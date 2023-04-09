use std::{
    sync::mpsc::{Receiver, Sender, TryRecvError},
};

// Given a string to execute, return the executed value

pub struct Backend {
    rx_controller: Receiver<String>,
    tx_controller: Sender<String>,
    last_value: f64,
}

impl Backend {
    pub fn init(tx: Sender<String>, rx: Receiver<String>) -> Backend {
        Backend {
            rx_controller: rx,
            tx_controller: tx,
            last_value: 0.0,
        }
    }

    pub fn receive(&mut self) {
        loop {
            match self.rx_controller.try_recv()
            {
                Ok(message) => {
                    let data = self.calculate(message).to_string();
                    self.tx_controller.send(String::from("backend ") + &data).unwrap();
                },
                Err(TryRecvError::Disconnected) => break,
                _ => {},
            }
        }
    }

    pub fn send(&self, message: &str) {
        self.tx_controller.send(message.to_owned()).unwrap();
    }

    fn calculate(&mut self, message: String) -> f64{
        match message.parse::<f64>()
        {
            Ok(number) => {
                self.last_value = number;
                return self.last_value;
            },
            Err(_) => return self.change_value(message),
        };
    }

    fn change_value(&mut self, message: String) -> f64
    {
        if message.contains("+")
        {
            let number = message.strip_prefix("+").unwrap().trim();
            self.last_value += number.parse::<f64>().unwrap();
        }
        else if message.contains("-")
        {
            let number = message.strip_prefix("-").unwrap().trim();
            self.last_value -= number.parse::<f64>().unwrap();
        }
        else if message.contains("*")
        {
            let number = message.strip_prefix("*").unwrap().trim();
            self.last_value *= number.parse::<f64>().unwrap();
        }
        else if message.contains("/")
        {
            let number = message.strip_prefix("/").unwrap().trim();
            self.last_value /= number.parse::<f64>().unwrap();
        }
        else if message.contains("%")
        {
            let number = message.strip_prefix("%").unwrap().trim();
            self.last_value %= number.parse::<f64>().unwrap();
        }
        return self.last_value;
    }
}
