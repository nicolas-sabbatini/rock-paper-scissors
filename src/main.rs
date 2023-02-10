use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::*;
use cursive::Cursive;
use rand::prelude::*;
use std::fmt;

const DIALOG_SIZE: (usize, usize) = (43, 11);

fn main() {
    let mut root = cursive::default();
    root.add_global_callback('q', Cursive::quit);

    root.add_layer(
        Dialog::text("Hi! Do you want to play a game of rock paper scissors?")
            .title("Main Menu")
            .button("Yes!😊", start_game)
            .button("No 😠", Cursive::quit)
            .h_align(HAlign::Center)
            .fixed_size(DIALOG_SIZE),
    );

    root.run();
}

#[derive(Debug, PartialEq)]
enum Options {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Results {
    Win,
    Lose,
    Tie,
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Results::Win => write!(f, "🍾🎉 YOU WIN!!! 🍾🎉"),
            Results::Lose => write!(f, "😭 You lose, better luck next time 😭"),
            Results::Tie => write!(f, "😱 Ohhh man, we tie 😱"),
        }
    }
}

impl Options {
    fn defeats(&self, aginst: &Options) -> Results {
        if self == aginst {
            return Results::Tie;
        } else if (*self == Options::Rock && *aginst == Options::Scissors)
            || (*self == Options::Paper && *aginst == Options::Rock)
            || (*self == Options::Scissors && *aginst == Options::Paper)
        {
            return Results::Win;
        }
        Results::Lose
    }

    fn from_rng() -> Self {
        let mut rng = rand::thread_rng();
        let select: f32 = rng.gen();

        if select < 0.33 {
            return Options::Rock;
        } else if select < 0.66 {
            return Options::Scissors;
        }
        Options::Paper
    }
}

fn start_game(root: &mut Cursive) {
    root.pop_layer();

    let options = SelectView::new()
        .h_align(HAlign::Center)
        .item("Rock", Options::Rock)
        .item("Paper", Options::Paper)
        .item("Scissors", Options::Scissors)
        .on_submit(play_action)
        .h_align(HAlign::Left);

    let message = TextView::new("🗡 Choose your weapon! 🗡").h_align(HAlign::Center);

    let layout = LinearLayout::vertical()
        .child(message)
        .child(DummyView)
        .child(options);

    root.add_layer(
        Dialog::around(layout)
            .button("I don't want to play 😠", Cursive::quit)
            .fixed_size(DIALOG_SIZE),
    );
}

fn play_action(root: &mut Cursive, selection: &Options) {
    root.pop_layer();

    let title = TextView::new("🎺 The results are 🎺").h_align(HAlign::Center);

    let cpu_selection = Options::from_rng();

    let comp_layout = LinearLayout::horizontal()
        .child(TextView::new(format!("You chose {selection:?}")).fixed_size((20, 2)))
        .child(DummyView)
        .child(
            TextView::new(format!("I chose {cpu_selection:?}"))
                .h_align(HAlign::Right)
                .fixed_size((20, 2)),
        );

    let layout = LinearLayout::vertical()
        .child(title)
        .child(DummyView)
        .child(DummyView)
        .child(comp_layout)
        .child(DummyView)
        .child(
            TextView::new(format!("{}", selection.defeats(&cpu_selection))).h_align(HAlign::Center),
        );

    root.add_layer(
        Dialog::around(layout)
            .button("Play again", start_game)
            .fixed_size(DIALOG_SIZE),
    );
}
