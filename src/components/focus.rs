
pub enum CurrentScreen {
    Main,
    Success,
    Failure,
    Exiting,
}

pub enum CurrentInput {
    Expression,
    Xdomain,
}

pub struct Focus {
    pub current_screen: CurrentScreen,
    pub current_input: Option<CurrentInput>,
}

impl Focus {
    pub fn new() -> Focus {
        Focus {
            current_screen: CurrentScreen::Main,
            current_input: None,
        }
    }

    pub fn reset(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.current_input = None;
    }

    // helper method to change the current input mode
    // if current mode is None change current input to Expression
    pub fn toggle_input(&mut self) {
        if let Some(edit_mode) = &self.current_input {
            match edit_mode {
                CurrentInput::Expression => {
                    self.current_input = Some(CurrentInput::Xdomain)
                }
                CurrentInput::Xdomain => {
                    self.current_input = Some(CurrentInput::Expression)
                }
            };
        }
        else {
            self.current_input = Some(CurrentInput::Expression);
        }
    }
}