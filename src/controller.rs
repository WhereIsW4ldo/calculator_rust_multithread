use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

// be used as a controller between the backend and frontend

use crate::{backend, frontend};

pub struct Controller {
    model_sender: Sender<String>,
    view_sender: Sender<String>,
    receiver: Receiver<String>
}

impl Controller {
    pub fn init() -> Controller{
        let (send_to_model, receiver_model) = mpsc::channel::<String>();
        let (send_to_view, receiver_view) = mpsc::channel::<String>();

        let (send_to_controller, receiver_controller) = mpsc::channel::<String>();

        let send_to_controller_view = send_to_controller.clone();

        // make threads for model and view
        let _model_handler = thread::spawn(move||{
            let mut back = backend::Backend::init(send_to_controller, receiver_model);
            back.receive();
        });

        let _view_handler = thread::spawn(move || {
            let mut front = frontend::Frontend::init(send_to_controller_view, receiver_view);
            front.receive();
        });

        thread::sleep(Duration::from_millis(10));

        Controller { model_sender: send_to_model, view_sender: send_to_view, receiver: receiver_controller }
    }

    pub fn run(&self)
    {
        loop {
            match &self.receiver.try_recv() {
                Ok(message) => self.message_received(message),
                Err(TryRecvError::Disconnected) => break,
                _ => {}
            }
        }
    }

    fn message_received(&self, message: &String)
    {
        if message.contains("backend ")
        {
            let data = message.strip_prefix("backend ").unwrap();
            self.view_sender.send(String::from(data)).unwrap();
        }
        else if message.contains("frontend ")
        {
            let data = message.strip_prefix("frontend ").unwrap();
            self.model_sender.send(String::from(data)).unwrap();
        }
    }
}
