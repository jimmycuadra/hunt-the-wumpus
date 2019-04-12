use crate::util::{js_rand, gen_range_avoiding, room_exits};

pub struct Game {
    pub arrows: u8,
    pub current_room: u8,
    pub messages: Vec<String>,
    pub wumpus: u8,
    bats: [u8; 2],
    pits: [u8; 2],
}

impl Game {
    pub fn configure_cave(&mut self) {
        self.messages.push(
            "You've entered a clammy, dark cave, armed with 5 arrows. You are very cold.".to_owned(),
        );
        self.wumpus = js_rand(1, 20);
        self.bats[0] = self.get_empty_room();
        self.bats[1] = self.get_empty_room();
        self.pits[0] = self.get_empty_room();
        self.pits[1] = self.get_empty_room();
        self.warning_messages();
    }

    pub fn move_effects(&mut self) -> Option<String> {
        self.warning_messages();

        if self.current_room == self.wumpus {
            Some("You have been eaten slowly and painfully by the wumpus".to_owned())
        } else if self.pits.contains(&self.current_room) {
            Some("You have fallen into a bottomless pit and must now wait to die, falling all the while".to_owned())
        } else if self.bats.contains(&self.current_room) {
            let current = self.current_room;
            let next = self.get_empty_room();
            self.messages.push(
                format!("A gigantic bat whisks you from room {} to room {} before you can even blink", current, next)
            );
            self.current_room = next;
            self.warning_messages();

            None
        } else {
            None
        }
    }

    fn get_empty_room(&self) -> u8 {
        gen_range_avoiding(
            0,
            20,
            vec![
                self.current_room,
                self.wumpus,
                self.bats[0],
                self.bats[1],
                self.pits[0],
                self.pits[1],
            ],
        )
    }

    fn warning_messages(&mut self) {
        for adj in &room_exits(self.current_room) {
            let t = *adj;

            if self.wumpus == t {
                self.messages.push(
                    "You smell something horrific and rancid.".to_owned(),
                );
            } else if self.pits.contains(&t) {
                self.messages.push(
                    "You feel a cold updraft from a nearby cavern.".to_owned(),
                );
            } else if self.bats.contains(&t) {
                self.messages.push(
                    "You hear a faint but distinct flapping of wings.".to_owned(),
                );
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut game = Self {
            arrows: 5,
            current_room: 1,
            messages: Vec::new(),
            wumpus: 0,
            bats: [0, 0],
            pits: [0, 0],
        };

        game.configure_cave();

        game
    }
}
