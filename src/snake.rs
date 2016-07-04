use std::time::Duration;
use std::collections::VecDeque;
use libusb::Result as UsbResult;
use rand::Rng;
use rand;
use g910::*;
use g910::StandardKey::*;

const FIELD: [[StandardKey; 13]; 4] = [
    [Circumflex,_1,_2,_3,_4,_5,_6,_7,_8,_9,_0,Sz,Tick],
    [Tab,Q,W,E,R,T,Z,U,I,O,P,Uuml,Plus],
    [CapsLock,A,S,D,F,G,H,J,K,L,Ouml,Auml,Sharp],
    [LeftShift,SmallerThan,Y,X,C,V,B,N,M,Comma,Dot,Minus,RightShift]
];

#[derive(Clone, Copy)]
enum Direction {
    Up, Down, Left, Right
}

#[derive(Debug, PartialEq)]
enum State {
    Running, Stop,
}

pub struct Snake {
    snake: VecDeque<(u8, u8)>,
    apple: (u8, u8),
    state: State,
    actions: VecDeque<Direction>,
    last_direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            snake: VecDeque::new(),
            apple: (0,0),
            state: State::Running,
            actions: VecDeque::new(),
            last_direction: Direction::Right,
        }
    }

    fn to_key_colors(&self) -> Vec<KeyColor> {
        let len = self.snake.len();
        let delta = 155.0 / len as f64;
        let mut vec: Vec<_> = self.snake.iter().skip(1).enumerate().map(|(i,&(x,y))| {
            let u = 100 + (delta * (len-i) as f64) as u8;
            KeyColor::new(Key::Standard(FIELD[y as usize][x as usize]),Color::new(0,0,u))
        }).collect();
        let (x,y) = self.snake[0];
        vec.push(KeyColor::new(Key::Standard(FIELD[y as usize][x as usize]), Color::new(0xe9,0x1e,0x63)));
        let (ax, ay) = self.apple;
        vec.push(KeyColor::new(Key::Standard(FIELD[ay as usize][ax as usize]), Color::new(0,255,0)));
        vec
    }

    fn new_apple(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let (ax, ay) = (rng.gen::<u8>() % FIELD[0].len() as u8, rng.gen::<u8>() % FIELD.len() as u8);
            if !self.snake.iter().any(|&(x,y)| x==ax && y==ay) {
                self.apple = (ax, ay);
                break;
            }
        }
    }

    fn init(&mut self, keyboard: &mut Keyboard) -> UsbResult<()> {
        self.snake.push_front((1, 2));
        self.snake.push_front((2, 2));
        self.snake.push_front((3, 2));
        self.new_apple();
        try!(keyboard.set_all_colors(Color::new(0, 0, 0)));
        keyboard.set_key_colors(self.to_key_colors())
    }

    fn accept_key(&self, evt: &KeyEvent) -> bool {
        if self.state == State::Running {
            match evt {
                &KeyEvent::KeyPressed(Key::Standard(Up)) => true,
                &KeyEvent::KeyPressed(Key::Standard(Down)) => true,
                &KeyEvent::KeyPressed(Key::Standard(Right)) => true,
                &KeyEvent::KeyPressed(Key::Standard(Left)) => true,
                _ => false
            }
        } else {
            match evt {
                &KeyEvent::KeyPressed(Key::Standard(NumReturn)) => true,
                _ => false,
            }
        }
    }

    fn handle_key(&mut self, evt: &KeyEvent, keyboard: &mut Keyboard) -> UsbResult<()> {
        if self.state == State::Running {
            if self.actions.len() >= 2 {
                self.actions.pop_front();
            }
            self.actions.push_back(match evt {
                &KeyEvent::KeyPressed(Key::Standard(Up)) => Direction::Up,
                &KeyEvent::KeyPressed(Key::Standard(Down)) => Direction::Down,
                &KeyEvent::KeyPressed(Key::Standard(Left)) => Direction::Left,
                &KeyEvent::KeyPressed(Key::Standard(Right)) => Direction::Right,
                _ => unreachable!()
            });
            Ok(())
        } else {
            ::std::mem::replace(self, Snake::new());
            self.init(keyboard)
        }
    }

    fn handle_time(&mut self, keyboard: &mut Keyboard) -> UsbResult<()> {
        if self.state == State::Stop {
            return Ok(());
        }
        // calc new coordinate
        let (x,y) = self.snake[0];
        let (x,y) = (x as i16, y as i16);
        if let Some(d) = self.actions.pop_front() {
            self.last_direction = d;
        }
        let (mut nx, mut ny) = match self.last_direction {
            Direction::Up => (x, y-1),
            Direction::Down => (x, y+1),
            Direction::Left => (x-1, y),
            Direction::Right => (x+1, y),
        };
        let xlen = FIELD[0].len() as i16;
        let ylen = FIELD.len() as i16;
        nx = ((nx % xlen) + xlen) % xlen;
        ny = ((ny % ylen) + ylen) % ylen;
        let (nx, ny) = (nx as u8, ny as u8);

        // eat an apple
        let eat = nx == self.apple.0 && ny == self.apple.1;
        let mut popped = Option::None;
        if !eat {
            popped = self.snake.pop_back();
        }

        // hit itself
        if self.snake.iter().any(|&(x,y)| x==nx && y==ny) {
            self.state = State::Stop;
            if let Some((px,py)) = popped {
                self.snake.push_back((px,py));
            }
            try!(keyboard.set_all_colors(Color::new(255,0,0)));
            let mut vec = Vec::new();
            vec.push(KeyColor::new(Key::Standard(StandardKey::NumReturn), Color::new(0,0,255)));
            vec.push(KeyColor::new(Key::Standard(FIELD[ny as usize][nx as usize]), Color::new(255,165,0)));
            try!(keyboard.set_key_colors(self.to_key_colors()));
            return keyboard.set_key_colors(vec)
        }

        // new tile
        self.snake.push_front((nx, ny));
        if eat {
            self.new_apple();
        }
        let mut vec = self.to_key_colors();
        if let Some((px,py)) = popped {
            if !self.snake.iter().any(|&(x,y)| x==px && y==py) {
                vec.push(KeyColor::new(Key::Standard(FIELD[py as usize][px as usize]), Color::new(0,0,0)));
            }
        }
        keyboard.set_key_colors(vec)
    }
}

impl From<Snake> for Handler {
    fn from(handler: Snake) -> Handler {
        HandlerBuilder::new(handler)
            .init_fn(|handler, keyboard| handler.init(keyboard))
            .accept_key_fn(|handler, evt| handler.accept_key(evt))
            .handle_key_fn(|handler, evt, keyboard| handler.handle_key(evt, keyboard))
            .handle_time_fn(|handler, _, keyboard| handler.handle_time(keyboard), Duration::from_millis(350))
            .build()
    }
}

