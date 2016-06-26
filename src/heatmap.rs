use std::collections::HashMap;
use libusb::Result as UsbResult;
use g910::*;

pub struct HeatmapHandler {
     heatmap: Heatmap,
}

impl HeatmapHandler {
    pub fn new() -> HeatmapHandler {
        HeatmapHandler {
            heatmap: Heatmap::new(),
        }
    }
}

impl HandleKey for HeatmapHandler {
    fn init(&mut self, keyboard: &mut Keyboard) -> UsbResult<()> {
        keyboard.set_all_colors(Color::new(0, 0, 0))
    }

    #[allow(unused_variables)]
    fn accept(&self, evt: &KeyEvent) -> bool {
        match evt {
            // we can't set colors of media keys
            &KeyEvent::KeyPressed(Key::Media(_)) => false,
            &KeyEvent::KeyPressed(_) => true,
            _ => false
        }
    }

    fn handle(&mut self, evt: &KeyEvent, keyboard: &mut Keyboard) -> UsbResult<()> {
        let key = match evt {
            &KeyEvent::KeyPressed(ref key) => key,
            _ => unreachable!()
        };
        self.heatmap.increment(key);
        keyboard.set_key_colors(self.heatmap.colors())
    }
}

impl From<HeatmapHandler> for Handler {
    fn from(handler: HeatmapHandler) -> Handler {
        Handler::HandleKey(Box::new(handler))
    }
}

const GRADIENT: [Color; 6] = [
    Color { red: 0, green: 0, blue: 0 },
    Color { red: 0, green: 0, blue: 255 },
    Color { red: 0, green: 255, blue: 255 },
    Color { red: 0, green: 255, blue: 0 },
    Color { red: 255, green: 255, blue: 0 },
    Color { red: 255, green: 0, blue: 0 },
];

pub struct Heatmap {
    data: HashMap<Key, u64>,
}

impl Heatmap {
    pub fn new() -> Heatmap {
        let mut data = HashMap::new();
        for key in Key::values() {
            match key {
                // we can't set the color of media keys
                Key::Media(_) => {},
                k => { data.insert(k, 0); },
            }
        }
        Heatmap {
            data: data,
        }
    }

    pub fn increment(&mut self, key: &Key) {
        match self.data.get_mut(&key) {
            Some(mut count) => *count += 1,
            None => unreachable!()
        }
    }

    /// Six Color Gradient:
    /// (1) black, (2) blue, (3) cyan, (4) green, (5) yellow, (6) red
    /// (http://www.andrewnoske.com/wiki/Code_-_heatmaps_and_color_gradients)
    pub fn colors<'a>(&'a self) -> Vec<KeyColor> {
        let max = match self.data.iter().map(|(_, v)| v).max() {
            Some(max) => max,
            None => unreachable!()
        };
        self.data.iter().map(|(k, v)| {
            let color;
            let v_scaled = *v as f64 / *max as f64;
            if v_scaled <= 0f64 {
                color = GRADIENT[0];
            } else if v_scaled >= 1f64 {
                color = GRADIENT[GRADIENT.len()-1];
            } else {
                let idx = (v_scaled * (GRADIENT.len()-1) as f64) as usize;
                let diff = (v_scaled * (GRADIENT.len()-1) as f64) - idx as f64;
                color = Color::new(
                    ((((GRADIENT[idx+1].red as i16 - GRADIENT[idx].red as i16) as f64) * diff) as i16 + GRADIENT[idx].red as i16) as u8,
                    ((((GRADIENT[idx+1].green as i16 - GRADIENT[idx].green as i16) as f64) * diff) as i16 + GRADIENT[idx].green as i16) as u8,
                    ((((GRADIENT[idx+1].blue as i16 - GRADIENT[idx].blue as i16) as f64) * diff) as i16 + GRADIENT[idx].blue as i16) as u8,
                );
            }
            KeyColor::new(k.clone(), color)
        }).collect()
    }
}

