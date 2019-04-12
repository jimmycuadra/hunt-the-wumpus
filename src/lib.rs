#[macro_use]
extern crate stdweb;

mod components {
    pub mod controls;
    pub mod messages;
    pub mod stats;
}
mod game;
mod util;

use yew::{
    html,
    prelude::*,
};

use crate::{
    components::{
        controls::Controls,
        messages::Messages,
        stats::Stats,
    },
    game::Game,
    util::{js_rand, room_exits},
};

pub enum Model {
    Waiting(String),
    Playing(Game),
}

#[derive(Clone, Debug)]
pub enum Msg {
    StartGame,
    ShootArrow(u8),
    SwitchRoom(u8),
}

impl Default for Model {
    fn default() -> Self {
        Model::Waiting("New Game!".to_owned())
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model::default()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShootArrow(target) => match self {
                Model::Playing(game) => {
                    if game.wumpus == target {
                        *self = Model::Waiting(
                            "With a sickening, satisfying thwack, your arrow finds its mark. Wumpus for dinner tonight! You win.".to_owned()
                        );
                    } else {
                        game.arrows -= 1;
                        game.messages.push(
                            "You arrow whistles aimlessly into the void".to_owned()
                        );

                        if game.arrows == 0 {
                            *self = Model::Waiting(
                                "You fired your very last arrow - you are now wumpus food".to_owned()
                            );
                        } else {
                            let rand = js_rand(1, 4);

                            if rand == 1 {
                                game.messages.push(
                                    "You listen quietly for any sign of movement - but the cave remains still.".to_owned(),
                                );
                            } else {
                                game.messages.push(
                                    "You hear a deafening roar - you've disturbed the wumpus!".to_owned()
                                );

                                let wumpus_exits = room_exits(game.wumpus);
                                let rand_idx = js_rand(0, 2);
                                game.wumpus = wumpus_exits[rand_idx as usize];

                                if game.wumpus == game.current_room {
                                    *self = Model::Waiting(
                                        "You scared the wumpus right on top of you. Good going, mincemeat".to_owned(),
                                    );
                                }
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
            Msg::SwitchRoom(target) => match self {
                Model::Playing(game) => {
                    game.current_room = target;

                    if let Some(msg) = game.move_effects() {
                        *self = Model::Waiting(msg);
                    }
                }
                _ => unreachable!(),
            }
            Msg::StartGame => *self = Model::Playing(Game::default()),
        }

        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        match self {
            Model::Waiting(s) => html! {
                <div class="hunt",>
                    <span class="over-message",>{ s }</span>
                    <button onclick = |_| Msg::StartGame,>{ "Play Again" }</button>
                </div>
            },
            Model::Playing(game) => html! {
                <div class="hunt",>
                    <div class="header",>{ "Hunt the Wumpus" }</div>
                    <div class="window",>
                        <Stats: arrows = game.arrows, current_room = game.current_room, />
                        <Controls: exits = room_exits(game.current_room), onsignal = |msg| msg, />
                        <Messages: messages = &game.messages, />
                    </div>
                </div>
            },
        }
    }
}
