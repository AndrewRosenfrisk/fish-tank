use std::{io::stdout, thread, time::Duration};

use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        size, Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use rand::{thread_rng, Rng};

const NUM_KELP: usize = 2;
const NUM_BUBBLERS: usize = 1;
const NUM_FISH: u16 = 10;

fn main() {
    //Initialize terminal & related variables
    let (width, height) = size().unwrap();
    let right_edge = width - 11;
    let left_edge: u16 = 0;
    execute!(stdout(), Clear(All), Clear(Purge), Hide, DisableLineWrap).unwrap();

    //Initialization of drawable entities
    let mut fishes = vec![];
    let mut kelps = vec![];
    let mut bubblers = vec![];

    //Other variables
    let mut rng = thread_rng();
    let mut step: u16 = 1;

    //FISH
    (0..NUM_FISH).for_each(|_| {
        fishes.push(Fish::new());
    });
    //KELP
    (0..NUM_KELP).for_each(|kelp| {
        kelps.push((rng.gen_range(left_edge..=right_edge), vec![]));

        (0..rng.gen_range(6..height - 1)).for_each(|_segment| {
            if rng.gen_bool(0.5) {
                kelps[kelp].1.push("(");
            } else {
                kelps[kelp].1.push(")");
            }
        });
    });
    //BUBBLERS
    (0..NUM_BUBBLERS).for_each(|_| {
        bubblers.push(rng.gen_range(left_edge..=right_edge));
    });
    let mut bubbles = vec![];

    'main: loop {
        //SIMULATION
        //sim fish
        for fish in fishes.iter_mut() {
            fish.move_fish(step);

            //random direction changes
            fish.x_dir_change_time -= 1;
            if fish.x_dir_change_time == 0 {
                fish.x_dir_change_time = rng.gen_range(10..60);
                fish.reverse_fish();
            }

            //random vert direction change
            fish.y_dir_change_time -= 1;
            if fish.y_dir_change_time == 0 {
                fish.y_dir_change_time = rng.gen_range(2..20);
                fish.down = !fish.down;
            }
        }

        //sim bubbles
        for bubbler in &bubblers {
            if rng.gen_bool(0.25) {
                bubbles.push(Point {
                    x: *bubbler,
                    y: height - 2,
                });
            }
        }

        for bubble in bubbles.iter_mut() {
            let rand_chance = rng.gen_range(1..6);
            if rand_chance == 1 && bubble.x != left_edge {
                bubble.x -= 1;
            } else if rand_chance == 2 && bubble.x != right_edge {
                bubble.x += 1;
            }

            if bubble.y > 0 {
                bubble.y -= 1;
            }
        }
        bubbles = bubbles
            .into_iter()
            .filter(|b| b.y > 0)
            .collect::<Vec<Point>>();

        //sim kelp
        for kelp in &mut kelps {
            kelp.1.iter_mut().enumerate().for_each(|(_index, segment)| {
                if rng.gen_bool(0.05) {
                    if *segment == "(" {
                        *segment = ")"
                    } else {
                        *segment = "("
                    }
                }
            });
        }

        //PRINTING
        //bubbles
        for bubble in &bubbles {
            let b_char = if rng.gen_bool(0.5) { "o" } else { "O" };
            execute!(
                stdout(),
                MoveTo(bubble.x, bubble.y),
                SetForegroundColor(Color::White),
                Print(b_char)
            )
            .unwrap();
        }

        //fish
        for fish in &fishes {
            fish.print_fish(step);
        }

        //kelp
        for kelp in &kelps {
            let mut modifier = 0;
            kelp.1.iter().enumerate().for_each(|(index, segment)| {
                if *segment == ")" {
                    modifier = 1;
                }
                execute!(
                    stdout(),
                    MoveTo(kelp.0, height - index as u16),
                    SetForegroundColor(Color::Green),
                    Print(segment)
                )
                .unwrap();
            });
        }

        let sand = char::from_u32(9617)
            .unwrap()
            .to_string()
            .repeat(width as usize);

        execute!(
            stdout(),
            MoveTo(0, height),
            SetForegroundColor(Color::Yellow),
            Print(sand)
        )
        .unwrap();

        thread::sleep(Duration::from_secs_f32((0.25) as f32));

        //CLEARING
        for bubble in &bubbles {
            execute!(stdout(), MoveTo(bubble.x, bubble.y), Print(" ")).unwrap();
        }
        for fish in &fishes {
            let replacement = " ".repeat(fish.frameset.right.0[0].len());
            execute!(
                stdout(),
                MoveTo(fish.location.x, fish.location.y),
                Print(replacement)
            )
            .unwrap();
        }
        for kelp in &kelps {
            kelp.1.iter().enumerate().for_each(|(index, _segment)| {
                execute!(
                    stdout(),
                    MoveTo(kelp.0, height - 2 - index as u16),
                    Print(" ")
                )
                .unwrap();
            });
        }

        step += 1;

        if step == u16::MAX {
            break 'main;
        }
    }
}

impl Frameset {
    fn random() -> Frameset {
        let mut rng = thread_rng();

        let frames_reference = vec![
            (vec!["><>".to_string()], vec!["<><".to_string()]),
            (vec![">||>".to_string()], vec!["<||<".to_string()]),
            (vec![">))>".to_string()], vec!["<[[<".to_string()]),
            (
                vec![">||o".to_string(), ">||.".to_string()],
                vec!["o||<".to_string(), ".||<".to_string()],
            ),
            (
                vec![">))o".to_string(), ">)).".to_string()],
                vec!["o[[<".to_string(), ".[[<".to_string()],
            ),
            (vec![">-==>".to_string()], vec!["<==-<".to_string()]),
            (vec![">\\\\>".to_string()], vec!["<//<".to_string()]),
            (vec!["><)))*>".to_string()], vec!["<*(((><".to_string()]),
            (vec!["}-[[[*>".to_string()], vec!["<*]]]-{".to_string()]),
            (vec!["]-<)))b>".to_string()], vec!["<d(((>-[".to_string()]),
            (vec!["><XXX*>".to_string()], vec!["<*XXX><".to_string()]),
            (
                vec![
                    "_.-._.-^=>".to_string(),
                    ".-._.-.^=>".to_string(),
                    "-._.-._^=>".to_string(),
                    "._.-._.^=>".to_string(),
                ],
                vec![
                    "<=^-._.-._".to_string(),
                    "<=^.-._.-.".to_string(),
                    "<=^_.-._.-".to_string(),
                    "<=^._.-._.".to_string(),
                ],
            ),
        ];

        let choice = rng.gen_range(0..frames_reference.len());

        let (right, left) = frames_reference[choice].clone();

        let colors = Fish::get_colors(&right[0]);

        let mut colors_rev = colors.clone();
        colors_rev.reverse();

        Frameset {
            right: (right, colors),
            left: (left, colors_rev),
        }
    }
}

#[derive(Debug, Clone)]
struct Frameset {
    right: (Vec<String>, Vec<Color>),
    left: (Vec<String>, Vec<Color>),
}

#[derive(Debug)]
struct Fish {
    frameset: Frameset,
    right: bool,
    down: bool,
    location: Point,
    x_velocity: u16,
    x_dir_change_time: u16,
    y_velocity: u16,
    y_dir_change_time: u16,
}
#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl Fish {
    fn new() -> Self {
        let (x, y) = size().unwrap();
        let mut rng = thread_rng();
        let frameset = Frameset::random();

        Fish {
            frameset,
            right: rng.gen_bool(0.5),
            down: rng.gen_bool(0.5),
            location: Point {
                x: rng.gen_range(11..(x - 11)),
                y: rng.gen_range(2..(y - 2)),
            },
            x_velocity: rng.gen_range(1..6),
            x_dir_change_time: rng.gen_range(10..60),
            y_velocity: rng.gen_range(5..15),
            y_dir_change_time: rng.gen_range(2..20),
        }
    }
    fn get_frame(&self, index: usize) -> String {
        if self.right {
            self.frameset.right.0[index].to_string()
        } else {
            self.frameset.left.0[index].to_string()
        }
    }
    fn reverse_fish(&mut self) {
        self.right = !self.right
    }

    fn get_colors(frame: &String) -> Vec<Color> {
        let mut colors: Vec<Color> = vec![];

        let mut rng = thread_rng();

        let choice = rng.gen_range(0..3);

        if choice == 0 {
            //Random color for each character
            for _ in frame.chars() {
                colors.push(random_color());
            }
        } else if choice == 1 {
            //One color for the whole fish
            let random_color = random_color();
            for _ in frame.chars() {
                colors.push(random_color);
            }
        } else if choice == 2 {
            //Head & Tail are the same, Body is different
            let head_tail_color = random_color();
            let body_color = random_color();

            for _ in frame.chars() {
                colors.push(body_color)
            }

            colors[0] = head_tail_color;

            if let Some(last) = colors.last_mut() {
                *last = head_tail_color;
            }
        }
        colors
    }

    fn print_fish(&self, step: u16) {
        let f = step as usize % &self.frameset.right.0.len();

        let frame = self.get_frame(f);
        let colors = if self.right {
            &self.frameset.right.1
        } else {
            &self.frameset.left.1
        };

        execute!(stdout(), MoveTo(self.location.x, self.location.y)).unwrap();

        for (index, char) in frame.char_indices() {
            execute!(stdout(), SetForegroundColor(colors[index]), Print(char),).unwrap();
        }
    }

    fn move_fish(&mut self, step: u16) {
        let (x, y) = size().unwrap();
        let right_edge = x - 11;
        let left_edge: u16 = 0;
        let top_edge: u16 = 1;
        let bot_edge = y - 2;

        if step % self.x_velocity == 0 {
            if (self.location.x == left_edge && !self.right)
                || (self.location.x == right_edge && self.right)
            {
                self.reverse_fish();
            } else {
                if self.right {
                    self.location.x += 1
                } else {
                    self.location.x -= 1;
                }
            }
        }
        if step % self.y_velocity == 0 {
            if (self.location.y == bot_edge && self.down)
                || (self.location.y == top_edge && !self.down)
            {
                self.down = !self.down;
            } else {
                if self.down {
                    self.location.y += 1;
                } else {
                    self.location.y -= 1;
                }
            }
        }
    }
}

fn random_color() -> Color {
    let mut rng = thread_rng();

    let colors = vec![
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    colors[rng.gen_range(0..8)]
}
