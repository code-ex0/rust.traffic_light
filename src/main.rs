use std::time::{
    SystemTime,
};

#[derive(PartialEq)]
enum State {
    RED,
    ORANGE,
    GREEN,
}

#[derive(Copy, Clone)]
struct Time {
    timestamp: u128,
}

impl Time {
    fn new() -> Time {
        Time {
            timestamp: millis()
        }
    }

    fn start(&mut self) {
        self.timestamp = millis();
    }

    fn elapsed(&self) -> u128 {
        millis() - self.timestamp
    }
}

struct Fsm {
    pub state: State,
}

impl Fsm {
    pub fn new() -> Fsm {
        Fsm {
            state: State::RED,
        }
    }

    pub fn update(&mut self,current_state: State, next_state: State, condition: bool, time: &mut Time) {
        if condition && self.state == current_state {
            self.state = next_state;
            time.start();
        }
    }

    pub fn execute(&self) {
        match self.state {
            State::RED => {
                println!("Red");
            },
            State::GREEN => {
                println!("Green");
            },
            State::ORANGE => {
                println!("Orange");
            },
        }
    }
}

fn millis() -> u128 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(x) => {x.as_millis()}
        Err(_) => {0}
    }
}

fn main() {

    let mut fsm = Fsm::new();
    let mut time = Time::new();

    loop {
        fsm.update(State::RED, State::GREEN, time.elapsed() > 10000, &mut time);
        fsm.update(State::GREEN, State::ORANGE, time.elapsed() > 30000, &mut time);
        fsm.update(State::ORANGE, State::RED, time.elapsed() > 3000, &mut time);
        fsm.execute();
    }
}
