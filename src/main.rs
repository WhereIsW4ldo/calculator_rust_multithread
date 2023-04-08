use controller::Controller;

pub mod backend;
pub mod controller;
pub mod frontend;

fn main() {
    let c = Controller::init();
    c.run();
}
