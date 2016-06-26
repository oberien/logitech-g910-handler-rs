use libusb::Result as UsbResult;
use g910::*;

pub struct FlashHandler;

impl FlashHandler {
    pub fn new() -> FlashHandler {
        FlashHandler { }
    }
}

impl HandleKey for FlashHandler {
    fn init(&mut self, keyboard: &mut Keyboard) -> UsbResult<()> {
        keyboard.set_all_colors(Color::new(0, 0, 255))
    }

    #[allow(unused_variables)]
    fn accept(&self, evt: &KeyEvent) -> bool {
        true
    }

    fn handle(&mut self, evt: &KeyEvent, keyboard: &mut Keyboard) -> UsbResult<()> {
        match evt {
            &KeyEvent::KeyPressed(_) => {
                keyboard.set_all_colors(Color::new(255, 0, 0))
            },
            &KeyEvent::KeyReleased(_) => {
                keyboard.set_all_colors(Color::new(0, 0, 255))
            },
        }
    }
}

impl From<FlashHandler> for Handler {
    fn from(handler: FlashHandler) -> Handler {
        Handler::HandleKey(Box::new(handler))
    }
}

