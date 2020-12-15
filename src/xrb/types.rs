pub type Window = u32;
pub type Pixmap = u32;
pub type Cursor = u32;
pub type GContext = u32;
pub type Colormap = u32;
pub type Font = u32;
pub type Atom = u32;
pub type VisualId = u32;
pub type Value = u32;
pub type Timestamp = u32;
pub type KeySym = u32;
pub type KeyCode = u8;
pub type Button = u8;

pub enum Drawable {
    Pixmap(Pixmap),
    Window(Window),
}

pub enum Fontable {
    Font(Font),
    GContext(GContext),
}


#[derive(Debug)]
pub enum BitGravity {
    Forget = 0,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    Static,
    SouthEast,
    South,
}

pub enum WinGravity {
    Unmap,
    Static,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
}

pub enum Event {
    KeyPress,
    KeyRelease,
    OwnerGrabButton,
    ButtonPress,
    ButtonRelease,
    EnterWindow,
    LeaveWindow,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Buttion3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
    Exposure,
    VisibilityChange,
    StructureNotify,
    ResizeRedirect,
    SubstructureNotify,
    SubstructureRedirect,
    FocusChange,
    PropertyChange,
    ColormapChange,
    KeymapState,
}

pub enum PointerEvent {
    ButtonPress,
    ButtonRelease,
    EnterWindow,
    LeaveWindow,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
    KeymapState,
}

pub enum DeviceEvent {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    PointerMotion,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
}

pub enum KeyMask {
    Shift,
    Lock,
    Control,
    Mod1,
    Mod2,
    Mod3,
    Mod4,
    Mod5,
}

pub enum ButMask {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
}

pub enum KeyButMask {
    KeyMask(KeyMask),
    ButMask(ButMask),
}

pub struct Char2B {
    byte1: u8,
    byte2: u8,
}

pub type String8 = Vec<u8>;
pub type String16 = Vec<Char2B>;

pub struct Point {
    x: i16,
    y: i16,
}

pub struct Rectangle {
    p: Point,
    width: u16,
    height: u16,
}

pub struct Arc {
    p: Point,
    width: u16,
    height: u16,
    angle1: i16,
    angle2: i16,
}

//TODO: deliberate name of this
pub enum Family {
    Internet,
    InternetV6,
    ServerInterpreted,
    DECnet,
    Chaos,
}

pub struct Host {
    family: Family,
    address: Vec<u8>,
}
