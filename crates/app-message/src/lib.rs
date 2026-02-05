



#[derive(Debug, Clone)]
pub enum Message {
    Button(usize, BTNMessage),
    Canvas(CanvasMessage),
    Card(usize, CardMessage),
    CheckBox(usize, CHKMessage),
    ColorPicker(usize, ColPikMessage),
    DatePicker(usize, DPMessage),
    Divider(usize, DivMessage),
    EventKeyboard(Event),
    EventMouse(Event),
    EventWindow((window::Id, Event)),
    EventTouch(Event),
    Image(usize, ImageMessage),
    // Modal(usize, ModalMessage),
    PickList(usize, PLMessage),
    Radio(usize, RDMessage),
    Scrolled(scrollable::Viewport, usize),
    SelectableText(usize, SLTXTMessage),
    Slider(usize, SLMessage),
    Svg(usize, SvgMessage),

    TableSync(scrollable::AbsoluteOffset, usize),
    TableDividerChanged((usize, usize, f32)),
    TableDividerReleased(usize),

    TextInput(usize, TIMessage),
    Toggler(usize, TOGMessage),
    CanvasTextBlink,
    Tick,
    CanvasTick,
    Timer(usize, TIMMessage),
    CanvasTimer(usize, CanvasTimerMessage),
    FontLoaded(Result<(), font::Error>),
    WindowOpened(window::Id, Option<Point>, Size),

    MouseAreaOnPress(usize),
    MouseAreaOnRelease(usize),
    MouseAreaOnRightPress(usize),
    MouseAreaOnRightRelease(usize),
    MouseAreaOnMiddlePress(usize),
    MouseAreaOnMiddleRelease(usize),
    MouseAreaOnEnter(usize),
    MouseAreaOnMove(Point, usize),
    MouseAreaOnExit(usize),

    OpaqueOnPress(usize),
}