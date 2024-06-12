// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

enum Message {
    // Définition des variantes de l'énumération `Message` basées sur leur utilisation dans les tests.
    Quit,                      //utilisée pour indiquer que le programme doit quitter.
    Echo(String),              //utilisée pour mettre à jour un message.
    Move(Point),               //utilisée pour mettre à jour la position.
    ChangeColor(u8, u8, u8),   //utilisée pour changer la couleur.
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // Utilisation de `match` pour traiter les différentes variantes de `Message`.
        match message {
            // Lorsqu'un `Message::ChangeColor` est reçu, met à jour la couleur de l'état.
            Message::ChangeColor(r, g, b) => self.color = (r, g, b),
            // Lorsqu'un `Message::Echo` est reçu, met à jour le message de l'état.
            Message::Echo(s) => self.message = s,
            // Lorsqu'un `Message::Move` est reçu, met à jour la position de l'état.
            Message::Move(x) => self.position = x,
            // Lorsqu'un `Message::Quit` est reçu, met à jour l'état pour indiquer que le programme doit quitter.
            Message::Quit => self.quit = true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
