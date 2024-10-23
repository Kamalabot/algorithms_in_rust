#[derive(Debug)]
enum State {
    Start,
    Processing,
    Finished,
}

struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::Start,
        }
    }

    fn transit(&mut self) {
        self.state = match self.state {
            State::Start => {
                println!("Transit to Processing");
                State::Processing
            }
            State::Processing => {
                println!("Transit to Finished");
                State::Finished
            }
            State::Finished => {
                println!("Already in Finished stage");
                State::Processing
            }
        }
    }

    fn current_state(&self) -> &State {
        &self.state
    }
}

fn main() {
    let mut sm = StateMachine::new();

    println!("{:?}", sm.current_state());
    sm.transit();
    println!("{:?}", sm.current_state());
    sm.transit();
}
