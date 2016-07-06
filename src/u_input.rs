use g910::*;
use uinput;
use uinput::Device;
use uinput::device::Builder;
use uinput::event::{Keyboard as UinputKey};
use uinput::event::keyboard::{Key as UinputStandardKey, Misc, KeyPad};
use libusb::Result as UsbResult;

pub struct UinputHandler {
    device: Device,
}

impl UinputHandler {
    pub fn new() -> UinputHandler {
        let path = ::std::path::Path::new("/dev/uinput");
        let def = Builder::open(&path).unwrap();
        let name = def.name("logitech-g910-rs").unwrap();
        let event = name.event(uinput::event::Keyboard::All).unwrap();
        let device = event.create().unwrap();
        UinputHandler {
            device: device,
        }
    }

    fn accept(&self, evt: &KeyEvent) -> bool {
        let k = match evt {
            &KeyEvent::KeyPressed(ref k) => k,
            &KeyEvent::KeyReleased(ref k) => k,
        };
        match k {
            &Key::Standard(_) => true,
            &Key::Media(_) => true,
            _ => false
        }
    }

    #[allow(unused_variables)]
    fn handle(&mut self, evt: &KeyEvent, keyboard: &mut Keyboard) -> UsbResult<()> {
        match evt {
            &KeyEvent::KeyPressed(ref k) => {
                let key_opt = match k {
                    &Key::Standard(s) => s.to_uinput_key(),
                    &Key::Media(m) => m.to_uinput_key(),
                    _ => unreachable!()
                };
                match key_opt {
                    Some(key) => self.device.press(&key).unwrap(),
                    None => {}
                }
            },
            &KeyEvent::KeyReleased(ref k) => {
                let key_opt = match k {
                    &Key::Standard(s) => s.to_uinput_key(),
                    &Key::Media(m) => m.to_uinput_key(),
                    _ => unreachable!()
                };
                match key_opt {
                    Some(key) => self.device.release(&key).unwrap(),
                    None => {}
                }
            },
        };
        self.device.synchronize().unwrap();
        Ok(())
    }
}

impl From<UinputHandler> for Handler {
    fn from(handler: UinputHandler) -> Handler {
        HandlerBuilder::new(handler)
            .accept_key_fn(|handler, evt| handler.accept(evt))
            .handle_key_fn(|handler, evt, keyboard| handler.handle(evt, keyboard))
            .build()
    }
}

trait ToUinputKey {
    fn to_uinput_key(&self) -> Option<UinputKey>;
}

impl ToUinputKey for StandardKey {
    fn to_uinput_key(&self) -> Option<UinputKey> {
        match self {
            &StandardKey::None => None,
            &StandardKey::A => Some(UinputKey::Key(UinputStandardKey::A)),
            &StandardKey::B => Some(UinputKey::Key(UinputStandardKey::B)),
            &StandardKey::C => Some(UinputKey::Key(UinputStandardKey::C)),
            &StandardKey::D => Some(UinputKey::Key(UinputStandardKey::D)),
            &StandardKey::E => Some(UinputKey::Key(UinputStandardKey::E)),
            &StandardKey::F => Some(UinputKey::Key(UinputStandardKey::F)),
            &StandardKey::G => Some(UinputKey::Key(UinputStandardKey::G)),
            &StandardKey::H => Some(UinputKey::Key(UinputStandardKey::H)),
            &StandardKey::I => Some(UinputKey::Key(UinputStandardKey::I)),
            &StandardKey::J => Some(UinputKey::Key(UinputStandardKey::J)),
            &StandardKey::K => Some(UinputKey::Key(UinputStandardKey::K)),
            &StandardKey::L => Some(UinputKey::Key(UinputStandardKey::L)),
            &StandardKey::M => Some(UinputKey::Key(UinputStandardKey::M)),
            &StandardKey::N => Some(UinputKey::Key(UinputStandardKey::N)),
            &StandardKey::O => Some(UinputKey::Key(UinputStandardKey::O)),
            &StandardKey::P => Some(UinputKey::Key(UinputStandardKey::P)),
            &StandardKey::Q => Some(UinputKey::Key(UinputStandardKey::Q)),
            &StandardKey::R => Some(UinputKey::Key(UinputStandardKey::R)),
            &StandardKey::S => Some(UinputKey::Key(UinputStandardKey::S)),
            &StandardKey::T => Some(UinputKey::Key(UinputStandardKey::T)),
            &StandardKey::U => Some(UinputKey::Key(UinputStandardKey::U)),
            &StandardKey::V => Some(UinputKey::Key(UinputStandardKey::V)),
            &StandardKey::W => Some(UinputKey::Key(UinputStandardKey::W)),
            &StandardKey::X => Some(UinputKey::Key(UinputStandardKey::X)),
            &StandardKey::Z => Some(UinputKey::Key(UinputStandardKey::Y)),
            &StandardKey::Y => Some(UinputKey::Key(UinputStandardKey::Z)),
            &StandardKey::_1 => Some(UinputKey::Key(UinputStandardKey::_1)),
            &StandardKey::_2 => Some(UinputKey::Key(UinputStandardKey::_2)),
            &StandardKey::_3 => Some(UinputKey::Key(UinputStandardKey::_3)),
            &StandardKey::_4 => Some(UinputKey::Key(UinputStandardKey::_4)),
            &StandardKey::_5 => Some(UinputKey::Key(UinputStandardKey::_5)),
            &StandardKey::_6 => Some(UinputKey::Key(UinputStandardKey::_6)),
            &StandardKey::_7 => Some(UinputKey::Key(UinputStandardKey::_7)),
            &StandardKey::_8 => Some(UinputKey::Key(UinputStandardKey::_8)),
            &StandardKey::_9 => Some(UinputKey::Key(UinputStandardKey::_9)),
            &StandardKey::_0 => Some(UinputKey::Key(UinputStandardKey::_0)),
            &StandardKey::Return => Some(UinputKey::Key(UinputStandardKey::Enter)),
            &StandardKey::Esc => Some(UinputKey::Key(UinputStandardKey::Esc)),
            &StandardKey::Backspace => Some(UinputKey::Key(UinputStandardKey::Backspace)),
            &StandardKey::Tab => Some(UinputKey::Key(UinputStandardKey::Tab)),
            &StandardKey::Space => Some(UinputKey::Key(UinputStandardKey::Space)),
            &StandardKey::Sz => Some(UinputKey::Key(UinputStandardKey::Minus)),
            &StandardKey::Tick => Some(UinputKey::Key(UinputStandardKey::Equal)),
            &StandardKey::Uuml => Some(UinputKey::Key(UinputStandardKey::LeftBrace)),
            &StandardKey::Plus => Some(UinputKey::Key(UinputStandardKey::RightBrace)),
            &StandardKey::Pipe => Some(UinputKey::Key(UinputStandardKey::BackSlash)),
            &StandardKey::Sharp => Some(UinputKey::Key(UinputStandardKey::BackSlash)),
            &StandardKey::Ouml => Some(UinputKey::Key(UinputStandardKey::SemiColon)),
            &StandardKey::Auml => Some(UinputKey::Key(UinputStandardKey::Apostrophe)),
            &StandardKey::Circumflex => Some(UinputKey::Key(UinputStandardKey::Grave)),
            &StandardKey::Comma => Some(UinputKey::Key(UinputStandardKey::Comma)),
            &StandardKey::Dot => Some(UinputKey::Key(UinputStandardKey::Dot)),
            &StandardKey::Minus => Some(UinputKey::Key(UinputStandardKey::Slash)),
            &StandardKey::CapsLock => Some(UinputKey::Key(UinputStandardKey::CapsLock)),
            &StandardKey::F1 => Some(UinputKey::Key(UinputStandardKey::F1)),
            &StandardKey::F2 => Some(UinputKey::Key(UinputStandardKey::F2)),
            &StandardKey::F3 => Some(UinputKey::Key(UinputStandardKey::F3)),
            &StandardKey::F4 => Some(UinputKey::Key(UinputStandardKey::F4)),
            &StandardKey::F5 => Some(UinputKey::Key(UinputStandardKey::F5)),
            &StandardKey::F6 => Some(UinputKey::Key(UinputStandardKey::F6)),
            &StandardKey::F7 => Some(UinputKey::Key(UinputStandardKey::F7)),
            &StandardKey::F8 => Some(UinputKey::Key(UinputStandardKey::F8)),
            &StandardKey::F9 => Some(UinputKey::Key(UinputStandardKey::F9)),
            &StandardKey::F10 => Some(UinputKey::Key(UinputStandardKey::F10)),
            &StandardKey::F11 => Some(UinputKey::Key(UinputStandardKey::F11)),
            &StandardKey::F12 => Some(UinputKey::Key(UinputStandardKey::F12)),
            &StandardKey::Print => Some(UinputKey::Key(UinputStandardKey::SysRq)),
            &StandardKey::ScrollLock => Some(UinputKey::Key(UinputStandardKey::ScrollLock)),
            &StandardKey::Pause => Some(UinputKey::Misc(Misc::Pause)),
            &StandardKey::Insert => Some(UinputKey::Key(UinputStandardKey::Insert)),
            &StandardKey::Home => Some(UinputKey::Key(UinputStandardKey::Home)),
            &StandardKey::PageUp => Some(UinputKey::Key(UinputStandardKey::PageUp)),
            &StandardKey::Delete => Some(UinputKey::Key(UinputStandardKey::Delete)),
            &StandardKey::End => Some(UinputKey::Key(UinputStandardKey::End)),
            &StandardKey::PageDown => Some(UinputKey::Key(UinputStandardKey::PageDown)),
            &StandardKey::Right => Some(UinputKey::Key(UinputStandardKey::Right)),
            &StandardKey::Left => Some(UinputKey::Key(UinputStandardKey::Left)),
            &StandardKey::Down => Some(UinputKey::Key(UinputStandardKey::Down)),
            &StandardKey::Up => Some(UinputKey::Key(UinputStandardKey::Up)),
            &StandardKey::NumLock => Some(UinputKey::Key(UinputStandardKey::NumLock)),
            &StandardKey::NumSlash => Some(UinputKey::KeyPad(KeyPad::Slash)),
            &StandardKey::NumStar => Some(UinputKey::KeyPad(KeyPad::Asterisk)),
            &StandardKey::NumMinus => Some(UinputKey::KeyPad(KeyPad::Minus)),
            &StandardKey::NumPlus => Some(UinputKey::KeyPad(KeyPad::Plus)),
            &StandardKey::NumReturn => Some(UinputKey::KeyPad(KeyPad::Enter)),
            &StandardKey::Num1 => Some(UinputKey::KeyPad(KeyPad::_1)),
            &StandardKey::Num2 => Some(UinputKey::KeyPad(KeyPad::_2)),
            &StandardKey::Num3 => Some(UinputKey::KeyPad(KeyPad::_3)),
            &StandardKey::Num4 => Some(UinputKey::KeyPad(KeyPad::_4)),
            &StandardKey::Num5 => Some(UinputKey::KeyPad(KeyPad::_5)),
            &StandardKey::Num6 => Some(UinputKey::KeyPad(KeyPad::_6)),
            &StandardKey::Num7 => Some(UinputKey::KeyPad(KeyPad::_7)),
            &StandardKey::Num8 => Some(UinputKey::KeyPad(KeyPad::_8)),
            &StandardKey::Num9 => Some(UinputKey::KeyPad(KeyPad::_9)),
            &StandardKey::Num0 => Some(UinputKey::KeyPad(KeyPad::_0)),
            &StandardKey::NumComma => Some(UinputKey::KeyPad(KeyPad::Dot)),
            &StandardKey::SmallerThan => Some(UinputKey::Misc(Misc::ND102)),
            &StandardKey::Menu => Some(UinputKey::Misc(Misc::Menu)),
            &StandardKey::International1 => None,
            &StandardKey::International2 => None,
            &StandardKey::International3 => None,
            &StandardKey::International4 => None,
            &StandardKey::International5 => None,
            &StandardKey::LeftControl => Some(UinputKey::Key(UinputStandardKey::LeftControl)),
            &StandardKey::LeftShift => Some(UinputKey::Key(UinputStandardKey::LeftShift)),
            &StandardKey::LeftAlt => Some(UinputKey::Key(UinputStandardKey::LeftAlt)),
            &StandardKey::LeftWindows => Some(UinputKey::Key(UinputStandardKey::LeftMeta)),
            &StandardKey::RightControl => Some(UinputKey::Key(UinputStandardKey::RightControl)),
            &StandardKey::RightShift => Some(UinputKey::Key(UinputStandardKey::RightShift)),
            &StandardKey::RightAlt => Some(UinputKey::Key(UinputStandardKey::RightAlt)),
            &StandardKey::RightWindows => Some(UinputKey::Key(UinputStandardKey::RightMeta))
        }
    }
}

impl ToUinputKey for MediaKey {
    fn to_uinput_key(&self) -> Option<UinputKey> {
        match self {
            &MediaKey::None => None,
            &MediaKey::Forward => Some(UinputKey::Misc(Misc::NextSong)),
            &MediaKey::Backward => Some(UinputKey::Misc(Misc::PreviousSong)),
            &MediaKey::Stop => Some(UinputKey::Misc(Misc::Stop)),
            &MediaKey::PlayPause => Some(UinputKey::Misc(Misc::PlayPause)),
            &MediaKey::VolumeUp => Some(UinputKey::Misc(Misc::VolumeUp)),
            &MediaKey::VolumeDown => Some(UinputKey::Misc(Misc::VolumeDown)),
            &MediaKey::Mute => Some(UinputKey::Misc(Misc::Mute))
        }
    }
}
