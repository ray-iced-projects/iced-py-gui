#![allow(dead_code)]
//! Bootstrap icons.

use pyo3::{Python, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

/// Bootstrap icons
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgIcon {
    /// alarm
    Alarm,
    /// alarm-fill
    AlarmFill,
    /// align-bottom
    AlignBottom,
    /// align-center
    AlignCenter,
    /// align-end
    AlignEnd,
    /// align-middle
    AlignMiddle,
    /// align-start
    AlignStart,
    /// align-top
    AlignTop,
    /// alt
    Alt,
    /// app
    App,
    /// app-indicator
    AppIndicator,
    /// archive
    Archive,
    /// archive-fill
    ArchiveFill,
    /// arrow-90deg-down
    Arrow90DegDown,
    /// arrow-90deg-left
    Arrow90DegLeft,
    /// arrow-90deg-right
    Arrow90DegRight,
    /// arrow-90deg-up
    Arrow90DegUp,
    /// arrow-bar-down
    ArrowBarDown,
    /// arrow-bar-left
    ArrowBarLeft,
    /// arrow-bar-right
    ArrowBarRight,
    /// arrow-bar-up
    ArrowBarUp,
    /// arrow-clockwise
    ArrowClockwise,
    /// arrow-counterclockwise
    ArrowCounterclockwise,
    /// arrow-down
    ArrowDown,
    /// arrow-down-circle
    ArrowDownCircle,
    /// arrow-down-circle-fill
    ArrowDownCircleFill,
    /// arrow-down-left
    ArrowDownLeft,
    /// arrow-down-left-circle
    ArrowDownLeftCircle,
    /// arrow-down-left-circle-fill
    ArrowDownLeftCircleFill,
    /// arrow-down-left-square
    ArrowDownLeftSquare,
    /// arrow-down-left-square-fill
    ArrowDownLeftSquareFill,
    /// arrow-down-right
    ArrowDownRight,
    /// arrow-down-right-circle
    ArrowDownRightCircle,
    /// arrow-down-right-circle-fill
    ArrowDownRightCircleFill,
    /// arrow-down-right-square
    ArrowDownRightSquare,
    /// arrow-down-right-square-fill
    ArrowDownRightSquareFill,
    /// arrow-down-short
    ArrowDownShort,
    /// arrow-down-square
    ArrowDownSquare,
    /// arrow-down-square-fill
    ArrowDownSquareFill,
    /// arrow-down-up
    ArrowDownUp,
    /// arrow-left
    ArrowLeft,
    /// arrow-left-circle
    ArrowLeftCircle,
    /// arrow-left-circle-fill
    ArrowLeftCircleFill,
    /// arrow-left-right
    ArrowLeftRight,
    /// arrow-left-short
    ArrowLeftShort,
    /// arrow-left-square
    ArrowLeftSquare,
    /// arrow-left-square-fill
    ArrowLeftSquareFill,
    /// arrow-repeat
    ArrowRepeat,
    /// arrow-return-left
    ArrowReturnLeft,
    /// arrow-return-right
    ArrowReturnRight,
    /// arrow-right
    ArrowRight,
    /// arrow-right-circle
    ArrowRightCircle,
    /// arrow-right-circle-fill
    ArrowRightCircleFill,
    /// arrow-right-short
    ArrowRightShort,
    /// arrow-right-square
    ArrowRightSquare,
    /// arrow-right-square-fill
    ArrowRightSquareFill,
    /// arrow-up
    ArrowUp,
    /// arrow-up-circle
    ArrowUpCircle,
    /// arrow-up-circle-fill
    ArrowUpCircleFill,
    /// arrow-up-left
    ArrowUpLeft,
    /// arrow-up-left-circle
    ArrowUpLeftCircle,
    /// arrow-up-left-circle-fill
    ArrowUpLeftCircleFill,
    /// arrow-up-left-square
    ArrowUpLeftSquare,
    /// arrow-up-left-square-fill
    ArrowUpLeftSquareFill,
    /// arrow-up-right
    ArrowUpRight,
    /// arrow-up-right-circle
    ArrowUpRightCircle,
    /// arrow-up-right-circle-fill
    ArrowUpRightCircleFill,
    /// arrow-up-right-square
    ArrowUpRightSquare,
    /// arrow-up-right-square-fill
    ArrowUpRightSquareFill,
    /// arrow-up-short
    ArrowUpShort,
    /// arrow-up-square
    ArrowUpSquare,
    /// arrow-up-square-fill
    ArrowUpSquareFill,
    /// arrows-angle-contract
    ArrowsAngleContract,
    /// arrows-angle-expand
    ArrowsAngleExpand,
    /// arrows-collapse
    ArrowsCollapse,
    /// arrows-expand
    ArrowsExpand,
    /// arrows-fullscreen
    ArrowsFullscreen,
    /// arrows-move
    ArrowsMove,
    /// aspect-ratio
    AspectRatio,
    /// aspect-ratio-fill
    AspectRatioFill,
    /// asterisk
    Asterisk,
    /// at
    At,
    /// award
    Award,
    /// award-fill
    AwardFill,
    /// back
    Back,
    /// backspace
    Backspace,
    /// backspace-fill
    BackspaceFill,
    /// backspace-reverse
    BackspaceReverse,
    /// backspace-reverse-fill
    BackspaceReverseFill,
    /// badge-4k
    Badge4K,
    /// badge-4k-fill
    Badge4KFill,
    /// badge-8k
    Badge8K,
    /// badge-8k-fill
    Badge8KFill,
    /// badge-ad
    BadgeAd,
    /// badge-ad-fill
    BadgeAdFill,
    /// badge-cc
    BadgeCc,
    /// badge-cc-fill
    BadgeCcFill,
    /// badge-hd
    BadgeHd,
    /// badge-hd-fill
    BadgeHdFill,
    /// badge-tm
    BadgeTm,
    /// badge-tm-fill
    BadgeTmFill,
    /// badge-vo
    BadgeVo,
    /// badge-vo-fill
    BadgeVoFill,
    /// bag
    Bag,
    /// bag-check
    BagCheck,
    /// bag-check-fill
    BagCheckFill,
    /// bag-dash
    BagDash,
    /// bag-dash-fill
    BagDashFill,
    /// bag-fill
    BagFill,
    /// bag-plus
    BagPlus,
    /// bag-plus-fill
    BagPlusFill,
    /// bag-x
    BagX,
    /// bag-x-fill
    BagXFill,
    /// bar-chart
    BarChart,
    /// bar-chart-fill
    BarChartFill,
    /// bar-chart-line
    BarChartLine,
    /// bar-chart-line-fill
    BarChartLineFill,
    /// bar-chart-steps
    BarChartSteps,
    /// basket
    Basket,
    /// basket-fill
    BasketFill,
    /// basket2
    Basket2,
    /// basket2-fill
    Basket2Fill,
    /// basket3
    Basket3,
    /// basket3-fill
    Basket3Fill,
    /// battery
    Battery,
    /// battery-charging
    BatteryCharging,
    /// battery-full
    BatteryFull,
    /// battery-half
    BatteryHalf,
    /// bell
    Bell,
    /// bell-fill
    BellFill,
    /// bezier
    Bezier,
    /// bezier2
    Bezier2,
    /// bicycle
    Bicycle,
    /// binoculars
    Binoculars,
    /// binoculars-fill
    BinocularsFill,
    /// blockquote-left
    BlockquoteLeft,
    /// blockquote-right
    BlockquoteRight,
    /// book
    Book,
    /// book-fill
    BookFill,
    /// book-half
    BookHalf,
    /// bookmark
    Bookmark,
    /// bookmark-check
    BookmarkCheck,
    /// bookmark-check-fill
    BookmarkCheckFill,
    /// bookmark-dash
    BookmarkDash,
    /// bookmark-dash-fill
    BookmarkDashFill,
    /// bookmark-fill
    BookmarkFill,
    /// bookmark-heart
    BookmarkHeart,
    /// bookmark-heart-fill
    BookmarkHeartFill,
    /// bookmark-plus
    BookmarkPlus,
    /// bookmark-plus-fill
    BookmarkPlusFill,
    /// bookmark-star
    BookmarkStar,
    /// bookmark-star-fill
    BookmarkStarFill,
    /// bookmark-x
    BookmarkX,
    /// bookmark-x-fill
    BookmarkXFill,
    /// bookmarks
    Bookmarks,
    /// bookmarks-fill
    BookmarksFill,
    /// bookshelf
    Bookshelf,
    /// bootstrap
    Bootstrap,
    /// bootstrap-fill
    BootstrapFill,
    /// bootstrap-reboot
    BootstrapReboot,
    /// border-style
    BorderStyle,
    /// border-width
    BorderWidth,
    /// bounding-box
    BoundingBox,
    /// bounding-box-circles
    BoundingBoxCircles,
    /// box
    Box,
    /// box-arrow-down
    BoxArrowDown,
    /// box-arrow-down-left
    BoxArrowDownLeft,
    /// box-arrow-down-right
    BoxArrowDownRight,
    /// box-arrow-in-down
    BoxArrowInDown,
    /// box-arrow-in-down-left
    BoxArrowInDownLeft,
    /// box-arrow-in-down-right
    BoxArrowInDownRight,
    /// box-arrow-in-left
    BoxArrowInLeft,
    /// box-arrow-in-right
    BoxArrowInRight,
    /// box-arrow-in-up
    BoxArrowInUp,
    /// box-arrow-in-up-left
    BoxArrowInUpLeft,
    /// box-arrow-in-up-right
    BoxArrowInUpRight,
    /// box-arrow-left
    BoxArrowLeft,
    /// box-arrow-right
    BoxArrowRight,
    /// box-arrow-up
    BoxArrowUp,
    /// box-arrow-up-left
    BoxArrowUpLeft,
    /// box-arrow-up-right
    BoxArrowUpRight,
    /// box-seam
    BoxSeam,
    /// braces
    Braces,
    /// bricks
    Bricks,
    /// briefcase
    Briefcase,
    /// briefcase-fill
    BriefcaseFill,
    /// brightness-alt-high
    BrightnessAltHigh,
    /// brightness-alt-high-fill
    BrightnessAltHighFill,
    /// brightness-alt-low
    BrightnessAltLow,
    /// brightness-alt-low-fill
    BrightnessAltLowFill,
    /// brightness-high
    BrightnessHigh,
    /// brightness-high-fill
    BrightnessHighFill,
    /// brightness-low
    BrightnessLow,
    /// brightness-low-fill
    BrightnessLowFill,
    /// broadcast
    Broadcast,
    /// broadcast-pin
    BroadcastPin,
    /// brush
    Brush,
    /// brush-fill
    BrushFill,
    /// bucket
    Bucket,
    /// bucket-fill
    BucketFill,
    /// bug
    Bug,
    /// bug-fill
    BugFill,
    /// building
    Building,
    /// bullseye
    Bullseye,
    /// calculator
    Calculator,
    /// calculator-fill
    CalculatorFill,
    /// calendar
    Calendar,
    /// calendar-check
    CalendarCheck,
    /// calendar-check-fill
    CalendarCheckFill,
    /// calendar-date
    CalendarDate,
    /// calendar-date-fill
    CalendarDateFill,
    /// calendar-day
    CalendarDay,
    /// calendar-day-fill
    CalendarDayFill,
    /// calendar-event
    CalendarEvent,
    /// calendar-event-fill
    CalendarEventFill,
    /// calendar-fill
    CalendarFill,
    /// calendar-minus
    CalendarMinus,
    /// calendar-minus-fill
    CalendarMinusFill,
    /// calendar-month
    CalendarMonth,
    /// calendar-month-fill
    CalendarMonthFill,
    /// calendar-plus
    CalendarPlus,
    /// calendar-plus-fill
    CalendarPlusFill,
    /// calendar-range
    CalendarRange,
    /// calendar-range-fill
    CalendarRangeFill,
    /// calendar-week
    CalendarWeek,
    /// calendar-week-fill
    CalendarWeekFill,
    /// calendar-x
    CalendarX,
    /// calendar-x-fill
    CalendarXFill,
    /// calendar2
    Calendar2,
    /// calendar2-check
    Calendar2Check,
    /// calendar2-check-fill
    Calendar2CheckFill,
    /// calendar2-date
    Calendar2Date,
    /// calendar2-date-fill
    Calendar2DateFill,
    /// calendar2-day
    Calendar2Day,
    /// calendar2-day-fill
    Calendar2DayFill,
    /// calendar2-event
    Calendar2Event,
    /// calendar2-event-fill
    Calendar2EventFill,
    /// calendar2-fill
    Calendar2Fill,
    /// calendar2-minus
    Calendar2Minus,
    /// calendar2-minus-fill
    Calendar2MinusFill,
    /// calendar2-month
    Calendar2Month,
    /// calendar2-month-fill
    Calendar2MonthFill,
    /// calendar2-plus
    Calendar2Plus,
    /// calendar2-plus-fill
    Calendar2PlusFill,
    /// calendar2-range
    Calendar2Range,
    /// calendar2-range-fill
    Calendar2RangeFill,
    /// calendar2-week
    Calendar2Week,
    /// calendar2-week-fill
    Calendar2WeekFill,
    /// calendar2-x
    Calendar2X,
    /// calendar2-x-fill
    Calendar2XFill,
    /// calendar3
    Calendar3,
    /// calendar3-event
    Calendar3Event,
    /// calendar3-event-fill
    Calendar3EventFill,
    /// calendar3-fill
    Calendar3Fill,
    /// calendar3-range
    Calendar3Range,
    /// calendar3-range-fill
    Calendar3RangeFill,
    /// calendar3-week
    Calendar3Week,
    /// calendar3-week-fill
    Calendar3WeekFill,
    /// calendar4
    Calendar4,
    /// calendar4-event
    Calendar4Event,
    /// calendar4-range
    Calendar4Range,
    /// calendar4-week
    Calendar4Week,
    /// camera
    Camera,
    /// camera-fill
    CameraFill,
    /// camera-reels
    CameraReels,
    /// camera-reels-fill
    CameraReelsFill,
    /// camera-video
    CameraVideo,
    /// camera-video-fill
    CameraVideoFill,
    /// camera-video-off
    CameraVideoOff,
    /// camera-video-off-fill
    CameraVideoOffFill,
    /// camera2
    Camera2,
    /// capslock
    Capslock,
    /// capslock-fill
    CapslockFill,
    /// card-checklist
    CardChecklist,
    /// card-heading
    CardHeading,
    /// card-image
    CardImage,
    /// card-list
    CardList,
    /// card-text
    CardText,
    /// caret-down
    CaretDown,
    /// caret-down-fill
    CaretDownFill,
    /// caret-down-square
    CaretDownSquare,
    /// caret-down-square-fill
    CaretDownSquareFill,
    /// caret-left
    CaretLeft,
    /// caret-left-fill
    CaretLeftFill,
    /// caret-left-square
    CaretLeftSquare,
    /// caret-left-square-fill
    CaretLeftSquareFill,
    /// caret-right
    CaretRight,
    /// caret-right-fill
    CaretRightFill,
    /// caret-right-square
    CaretRightSquare,
    /// caret-right-square-fill
    CaretRightSquareFill,
    /// caret-up
    CaretUp,
    /// caret-up-fill
    CaretUpFill,
    /// caret-up-square
    CaretUpSquare,
    /// caret-up-square-fill
    CaretUpSquareFill,
    /// cart
    Cart,
    /// cart-check
    CartCheck,
    /// cart-check-fill
    CartCheckFill,
    /// cart-dash
    CartDash,
    /// cart-dash-fill
    CartDashFill,
    /// cart-fill
    CartFill,
    /// cart-plus
    CartPlus,
    /// cart-plus-fill
    CartPlusFill,
    /// cart-x
    CartX,
    /// cart-x-fill
    CartXFill,
    /// cart2
    Cart2,
    /// cart3
    Cart3,
    /// cart4
    Cart4,
    /// cash
    Cash,
    /// cash-stack
    CashStack,
    /// cast
    Cast,
    /// chat
    Chat,
    /// chat-dots
    ChatDots,
    /// chat-dots-fill
    ChatDotsFill,
    /// chat-fill
    ChatFill,
    /// chat-left
    ChatLeft,
    /// chat-left-dots
    ChatLeftDots,
    /// chat-left-dots-fill
    ChatLeftDotsFill,
    /// chat-left-fill
    ChatLeftFill,
    /// chat-left-quote
    ChatLeftQuote,
    /// chat-left-quote-fill
    ChatLeftQuoteFill,
    /// chat-left-text
    ChatLeftText,
    /// chat-left-text-fill
    ChatLeftTextFill,
    /// chat-quote
    ChatQuote,
    /// chat-quote-fill
    ChatQuoteFill,
    /// chat-right
    ChatRight,
    /// chat-right-dots
    ChatRightDots,
    /// chat-right-dots-fill
    ChatRightDotsFill,
    /// chat-right-fill
    ChatRightFill,
    /// chat-right-quote
    ChatRightQuote,
    /// chat-right-quote-fill
    ChatRightQuoteFill,
    /// chat-right-text
    ChatRightText,
    /// chat-right-text-fill
    ChatRightTextFill,
    /// chat-square
    ChatSquare,
    /// chat-square-dots
    ChatSquareDots,
    /// chat-square-dots-fill
    ChatSquareDotsFill,
    /// chat-square-fill
    ChatSquareFill,
    /// chat-square-quote
    ChatSquareQuote,
    /// chat-square-quote-fill
    ChatSquareQuoteFill,
    /// chat-square-text
    ChatSquareText,
    /// chat-square-text-fill
    ChatSquareTextFill,
    /// chat-text
    ChatText,
    /// chat-text-fill
    ChatTextFill,
    /// check
    Check,
    /// check-all
    CheckAll,
    /// check-circle
    CheckCircle,
    /// check-circle-fill
    CheckCircleFill,
    /// check-square
    CheckSquare,
    /// check-square-fill
    CheckSquareFill,
    /// check2
    Check2,
    /// check2-all
    Check2All,
    /// check2-circle
    Check2Circle,
    /// check2-square
    Check2Square,
    /// chevron-bar-contract
    ChevronBarContract,
    /// chevron-bar-down
    ChevronBarDown,
    /// chevron-bar-expand
    ChevronBarExpand,
    /// chevron-bar-left
    ChevronBarLeft,
    /// chevron-bar-right
    ChevronBarRight,
    /// chevron-bar-up
    ChevronBarUp,
    /// chevron-compact-down
    ChevronCompactDown,
    /// chevron-compact-left
    ChevronCompactLeft,
    /// chevron-compact-right
    ChevronCompactRight,
    /// chevron-compact-up
    ChevronCompactUp,
    /// chevron-contract
    ChevronContract,
    /// chevron-double-down
    ChevronDoubleDown,
    /// chevron-double-left
    ChevronDoubleLeft,
    /// chevron-double-right
    ChevronDoubleRight,
    /// chevron-double-up
    ChevronDoubleUp,
    /// chevron-down
    ChevronDown,
    /// chevron-expand
    ChevronExpand,
    /// chevron-left
    ChevronLeft,
    /// chevron-right
    ChevronRight,
    /// chevron-up
    ChevronUp,
    /// circle
    Circle,
    /// circle-fill
    CircleFill,
    /// circle-half
    CircleHalf,
    /// circle-square
    CircleSquare,
    /// clipboard
    Clipboard,
    /// clipboard-check
    ClipboardCheck,
    /// clipboard-data
    ClipboardData,
    /// clipboard-minus
    ClipboardMinus,
    /// clipboard-plus
    ClipboardPlus,
    /// clipboard-x
    ClipboardX,
    /// clock
    Clock,
    /// clock-fill
    ClockFill,
    /// clock-history
    ClockHistory,
    /// cloud
    Cloud,
    /// cloud-arrow-down
    CloudArrowDown,
    /// cloud-arrow-down-fill
    CloudArrowDownFill,
    /// cloud-arrow-up
    CloudArrowUp,
    /// cloud-arrow-up-fill
    CloudArrowUpFill,
    /// cloud-check
    CloudCheck,
    /// cloud-check-fill
    CloudCheckFill,
    /// cloud-download
    CloudDownload,
    /// cloud-download-fill
    CloudDownloadFill,
    /// cloud-fill
    CloudFill,
    /// cloud-minus
    CloudMinus,
    /// cloud-minus-fill
    CloudMinusFill,
    /// cloud-plus
    CloudPlus,
    /// cloud-plus-fill
    CloudPlusFill,
    /// cloud-slash
    CloudSlash,
    /// cloud-slash-fill
    CloudSlashFill,
    /// cloud-upload
    CloudUpload,
    /// cloud-upload-fill
    CloudUploadFill,
    /// code
    Code,
    /// code-slash
    CodeSlash,
    /// code-square
    CodeSquare,
    /// collection
    Collection,
    /// collection-fill
    CollectionFill,
    /// collection-play
    CollectionPlay,
    /// collection-play-fill
    CollectionPlayFill,
    /// columns
    Columns,
    /// columns-gap
    ColumnsGap,
    /// command
    Command,
    /// compass
    Compass,
    /// compass-fill
    CompassFill,
    /// cone
    Cone,
    /// cone-striped
    ConeStriped,
    /// controller
    Controller,
    /// cpu
    Cpu,
    /// cpu-fill
    CpuFill,
    /// credit-card
    CreditCard,
    /// credit-card-2-back
    CreditCard2Back,
    /// credit-card-2-back-fill
    CreditCard2BackFill,
    /// credit-card-2-front
    CreditCard2Front,
    /// credit-card-2-front-fill
    CreditCard2FrontFill,
    /// credit-card-fill
    CreditCardFill,
    /// crop
    Crop,
    /// cup
    Cup,
    /// cup-fill
    CupFill,
    /// cup-straw
    CupStraw,
    /// cursor
    Cursor,
    /// cursor-fill
    CursorFill,
    /// cursor-text
    CursorText,
    /// dash
    Dash,
    /// dash-circle
    DashCircle,
    /// dash-circle-fill
    DashCircleFill,
    /// dash-square
    DashSquare,
    /// dash-square-fill
    DashSquareFill,
    /// diagram-2
    Diagram2,
    /// diagram-2-fill
    Diagram2Fill,
    /// diagram-3
    Diagram3,
    /// diagram-3-fill
    Diagram3Fill,
    /// diamond
    Diamond,
    /// diamond-fill
    DiamondFill,
    /// diamond-half
    DiamondHalf,
    /// dice-1
    Dice1,
    /// dice-1-fill
    Dice1Fill,
    /// dice-2
    Dice2,
    /// dice-2-fill
    Dice2Fill,
    /// dice-3
    Dice3,
    /// dice-3-fill
    Dice3Fill,
    /// dice-4
    Dice4,
    /// dice-4-fill
    Dice4Fill,
    /// dice-5
    Dice5,
    /// dice-5-fill
    Dice5Fill,
    /// dice-6
    Dice6,
    /// dice-6-fill
    Dice6Fill,
    /// disc
    Disc,
    /// disc-fill
    DiscFill,
    /// discord
    Discord,
    /// display
    Display,
    /// display-fill
    DisplayFill,
    /// distribute-horizontal
    DistributeHorizontal,
    /// distribute-vertical
    DistributeVertical,
    /// door-closed
    DoorClosed,
    /// door-closed-fill
    DoorClosedFill,
    /// door-open
    DoorOpen,
    /// door-open-fill
    DoorOpenFill,
    /// dot
    Dot,
    /// download
    Download,
    /// droplet
    Droplet,
    /// droplet-fill
    DropletFill,
    /// droplet-half
    DropletHalf,
    /// earbuds
    Earbuds,
    /// easel
    Easel,
    /// easel-fill
    EaselFill,
    /// egg
    Egg,
    /// egg-fill
    EggFill,
    /// egg-fried
    EggFried,
    /// eject
    Eject,
    /// eject-fill
    EjectFill,
    /// emoji-angry
    EmojiAngry,
    /// emoji-angry-fill
    EmojiAngryFill,
    /// emoji-dizzy
    EmojiDizzy,
    /// emoji-dizzy-fill
    EmojiDizzyFill,
    /// emoji-expressionless
    EmojiExpressionless,
    /// emoji-expressionless-fill
    EmojiExpressionlessFill,
    /// emoji-frown
    EmojiFrown,
    /// emoji-frown-fill
    EmojiFrownFill,
    /// emoji-heart-eyes
    EmojiHeartEyes,
    /// emoji-heart-eyes-fill
    EmojiHeartEyesFill,
    /// emoji-laughing
    EmojiLaughing,
    /// emoji-laughing-fill
    EmojiLaughingFill,
    /// emoji-neutral
    EmojiNeutral,
    /// emoji-neutral-fill
    EmojiNeutralFill,
    /// emoji-smile
    EmojiSmile,
    /// emoji-smile-fill
    EmojiSmileFill,
    /// emoji-smile-upside-down
    EmojiSmileUpsideDown,
    /// emoji-smile-upside-down-fill
    EmojiSmileUpsideDownFill,
    /// emoji-sunglasses
    EmojiSunglasses,
    /// emoji-sunglasses-fill
    EmojiSunglassesFill,
    /// emoji-wink
    EmojiWink,
    /// emoji-wink-fill
    EmojiWinkFill,
    /// envelope
    Envelope,
    /// envelope-fill
    EnvelopeFill,
    /// envelope-open
    EnvelopeOpen,
    /// envelope-open-fill
    EnvelopeOpenFill,
    /// exclamation
    Exclamation,
    /// exclamation-circle
    ExclamationCircle,
    /// exclamation-circle-fill
    ExclamationCircleFill,
    /// exclamation-diamond
    ExclamationDiamond,
    /// exclamation-diamond-fill
    ExclamationDiamondFill,
    /// exclamation-octagon
    ExclamationOctagon,
    /// exclamation-octagon-fill
    ExclamationOctagonFill,
    /// exclamation-square
    ExclamationSquare,
    /// exclamation-square-fill
    ExclamationSquareFill,
    /// exclamation-triangle
    ExclamationTriangle,
    /// exclamation-triangle-fill
    ExclamationTriangleFill,
    /// exclude
    Exclude,
    /// eye
    Eye,
    /// eye-fill
    EyeFill,
    /// eye-slash
    EyeSlash,
    /// eye-slash-fill
    EyeSlashFill,
    /// eyeglasses
    Eyeglasses,
    /// facebook
    Facebook,
    /// file
    File,
    /// file-arrow-down
    FileArrowDown,
    /// file-arrow-down-fill
    FileArrowDownFill,
    /// file-arrow-up
    FileArrowUp,
    /// file-arrow-up-fill
    FileArrowUpFill,
    /// file-bar-graph
    FileBarGraph,
    /// file-bar-graph-fill
    FileBarGraphFill,
    /// file-binary
    FileBinary,
    /// file-binary-fill
    FileBinaryFill,
    /// file-break
    FileBreak,
    /// file-break-fill
    FileBreakFill,
    /// file-check
    FileCheck,
    /// file-check-fill
    FileCheckFill,
    /// file-code
    FileCode,
    /// file-code-fill
    FileCodeFill,
    /// file-diff
    FileDiff,
    /// file-diff-fill
    FileDiffFill,
    /// file-earmark
    FileEarmark,
    /// file-earmark-arrow-down
    FileEarmarkArrowDown,
    /// file-earmark-arrow-down-fill
    FileEarmarkArrowDownFill,
    /// file-earmark-arrow-up
    FileEarmarkArrowUp,
    /// file-earmark-arrow-up-fill
    FileEarmarkArrowUpFill,
    /// file-earmark-bar-graph
    FileEarmarkBarGraph,
    /// file-earmark-bar-graph-fill
    FileEarmarkBarGraphFill,
    /// file-earmark-binary
    FileEarmarkBinary,
    /// file-earmark-binary-fill
    FileEarmarkBinaryFill,
    /// file-earmark-break
    FileEarmarkBreak,
    /// file-earmark-break-fill
    FileEarmarkBreakFill,
    /// file-earmark-check
    FileEarmarkCheck,
    /// file-earmark-check-fill
    FileEarmarkCheckFill,
    /// file-earmark-code
    FileEarmarkCode,
    /// file-earmark-code-fill
    FileEarmarkCodeFill,
    /// file-earmark-diff
    FileEarmarkDiff,
    /// file-earmark-diff-fill
    FileEarmarkDiffFill,
    /// file-earmark-easel
    FileEarmarkEasel,
    /// file-earmark-easel-fill
    FileEarmarkEaselFill,
    /// file-earmark-excel
    FileEarmarkExcel,
    /// file-earmark-excel-fill
    FileEarmarkExcelFill,
    /// file-earmark-fill
    FileEarmarkFill,
    /// file-earmark-font
    FileEarmarkFont,
    /// file-earmark-font-fill
    FileEarmarkFontFill,
    /// file-earmark-image
    FileEarmarkImage,
    /// file-earmark-image-fill
    FileEarmarkImageFill,
    /// file-earmark-lock
    FileEarmarkLock,
    /// file-earmark-lock-fill
    FileEarmarkLockFill,
    /// file-earmark-lock2
    FileEarmarkLock2,
    /// file-earmark-lock2-fill
    FileEarmarkLock2Fill,
    /// file-earmark-medical
    FileEarmarkMedical,
    /// file-earmark-medical-fill
    FileEarmarkMedicalFill,
    /// file-earmark-minus
    FileEarmarkMinus,
    /// file-earmark-minus-fill
    FileEarmarkMinusFill,
    /// file-earmark-music
    FileEarmarkMusic,
    /// file-earmark-music-fill
    FileEarmarkMusicFill,
    /// file-earmark-person
    FileEarmarkPerson,
    /// file-earmark-person-fill
    FileEarmarkPersonFill,
    /// file-earmark-play
    FileEarmarkPlay,
    /// file-earmark-play-fill
    FileEarmarkPlayFill,
    /// file-earmark-plus
    FileEarmarkPlus,
    /// file-earmark-plus-fill
    FileEarmarkPlusFill,
    /// file-earmark-post
    FileEarmarkPost,
    /// file-earmark-post-fill
    FileEarmarkPostFill,
    /// file-earmark-ppt
    FileEarmarkPpt,
    /// file-earmark-ppt-fill
    FileEarmarkPptFill,
    /// file-earmark-richtext
    FileEarmarkRichtext,
    /// file-earmark-richtext-fill
    FileEarmarkRichtextFill,
    /// file-earmark-ruled
    FileEarmarkRuled,
    /// file-earmark-ruled-fill
    FileEarmarkRuledFill,
    /// file-earmark-slides
    FileEarmarkSlides,
    /// file-earmark-slides-fill
    FileEarmarkSlidesFill,
    /// file-earmark-spreadsheet
    FileEarmarkSpreadsheet,
    /// file-earmark-spreadsheet-fill
    FileEarmarkSpreadsheetFill,
    /// file-earmark-text
    FileEarmarkText,
    /// file-earmark-text-fill
    FileEarmarkTextFill,
    /// file-earmark-word
    FileEarmarkWord,
    /// file-earmark-word-fill
    FileEarmarkWordFill,
    /// file-earmark-x
    FileEarmarkX,
    /// file-earmark-x-fill
    FileEarmarkXFill,
    /// file-earmark-zip
    FileEarmarkZip,
    /// file-earmark-zip-fill
    FileEarmarkZipFill,
    /// file-easel
    FileEasel,
    /// file-easel-fill
    FileEaselFill,
    /// file-excel
    FileExcel,
    /// file-excel-fill
    FileExcelFill,
    /// file-fill
    FileFill,
    /// file-font
    FileFont,
    /// file-font-fill
    FileFontFill,
    /// file-image
    FileImage,
    /// file-image-fill
    FileImageFill,
    /// file-lock
    FileLock,
    /// file-lock-fill
    FileLockFill,
    /// file-lock2
    FileLock2,
    /// file-lock2-fill
    FileLock2Fill,
    /// file-medical
    FileMedical,
    /// file-medical-fill
    FileMedicalFill,
    /// file-minus
    FileMinus,
    /// file-minus-fill
    FileMinusFill,
    /// file-music
    FileMusic,
    /// file-music-fill
    FileMusicFill,
    /// file-person
    FilePerson,
    /// file-person-fill
    FilePersonFill,
    /// file-play
    FilePlay,
    /// file-play-fill
    FilePlayFill,
    /// file-plus
    FilePlus,
    /// file-plus-fill
    FilePlusFill,
    /// file-post
    FilePost,
    /// file-post-fill
    FilePostFill,
    /// file-ppt
    FilePpt,
    /// file-ppt-fill
    FilePptFill,
    /// file-richtext
    FileRichtext,
    /// file-richtext-fill
    FileRichtextFill,
    /// file-ruled
    FileRuled,
    /// file-ruled-fill
    FileRuledFill,
    /// file-slides
    FileSlides,
    /// file-slides-fill
    FileSlidesFill,
    /// file-spreadsheet
    FileSpreadsheet,
    /// file-spreadsheet-fill
    FileSpreadsheetFill,
    /// file-text
    FileText,
    /// file-text-fill
    FileTextFill,
    /// file-word
    FileWord,
    /// file-word-fill
    FileWordFill,
    /// file-x
    FileX,
    /// file-x-fill
    FileXFill,
    /// file-zip
    FileZip,
    /// file-zip-fill
    FileZipFill,
    /// files
    Files,
    /// files-alt
    FilesAlt,
    /// film
    Film,
    /// filter
    Filter,
    /// filter-circle
    FilterCircle,
    /// filter-circle-fill
    FilterCircleFill,
    /// filter-left
    FilterLeft,
    /// filter-right
    FilterRight,
    /// filter-square
    FilterSquare,
    /// filter-square-fill
    FilterSquareFill,
    /// flag
    Flag,
    /// flag-fill
    FlagFill,
    /// flower1
    Flower1,
    /// flower2
    Flower2,
    /// flower3
    Flower3,
    /// folder
    Folder,
    /// folder-check
    FolderCheck,
    /// folder-fill
    FolderFill,
    /// folder-minus
    FolderMinus,
    /// folder-plus
    FolderPlus,
    /// folder-symlink
    FolderSymlink,
    /// folder-symlink-fill
    FolderSymlinkFill,
    /// folder-x
    FolderX,
    /// folder2
    Folder2,
    /// folder2-open
    Folder2Open,
    /// fonts
    Fonts,
    /// forward
    Forward,
    /// forward-fill
    ForwardFill,
    /// front
    Front,
    /// fullscreen
    Fullscreen,
    /// fullscreen-exit
    FullscreenExit,
    /// funnel
    Funnel,
    /// funnel-fill
    FunnelFill,
    /// gear
    Gear,
    /// gear-fill
    GearFill,
    /// gear-wide
    GearWide,
    /// gear-wide-connected
    GearWideConnected,
    /// gem
    Gem,
    /// geo
    Geo,
    /// geo-alt
    GeoAlt,
    /// geo-alt-fill
    GeoAltFill,
    /// geo-fill
    GeoFill,
    /// gift
    Gift,
    /// gift-fill
    GiftFill,
    /// github
    Github,
    /// globe
    Globe,
    /// globe2
    Globe2,
    /// google
    Google,
    /// graph-down
    GraphDown,
    /// graph-up
    GraphUp,
    /// grid
    Grid,
    /// grid-1x2
    Grid1X2,
    /// grid-1x2-fill
    Grid1X2Fill,
    /// grid-3x2
    Grid3X2,
    /// grid-3x2-gap
    Grid3X2Gap,
    /// grid-3x2-gap-fill
    Grid3X2GapFill,
    /// grid-3x3
    Grid3X3,
    /// grid-3x3-gap
    Grid3X3Gap,
    /// grid-3x3-gap-fill
    Grid3X3GapFill,
    /// grid-fill
    GridFill,
    /// grip-horizontal
    GripHorizontal,
    /// grip-vertical
    GripVertical,
    /// hammer
    Hammer,
    /// hand-index
    HandIndex,
    /// hand-index-thumb
    HandIndexThumb,
    /// hand-thumbs-down
    HandThumbsDown,
    /// hand-thumbs-up
    HandThumbsUp,
    /// handbag
    Handbag,
    /// handbag-fill
    HandbagFill,
    /// hash
    Hash,
    /// hdd
    Hdd,
    /// hdd-fill
    HddFill,
    /// hdd-network
    HddNetwork,
    /// hdd-network-fill
    HddNetworkFill,
    /// hdd-rack
    HddRack,
    /// hdd-rack-fill
    HddRackFill,
    /// hdd-stack
    HddStack,
    /// hdd-stack-fill
    HddStackFill,
    /// headphones
    Headphones,
    /// headset
    Headset,
    /// heart
    Heart,
    /// heart-fill
    HeartFill,
    /// heart-half
    HeartHalf,
    /// heptagon
    Heptagon,
    /// heptagon-fill
    HeptagonFill,
    /// heptagon-half
    HeptagonHalf,
    /// hexagon
    Hexagon,
    /// hexagon-fill
    HexagonFill,
    /// hexagon-half
    HexagonHalf,
    /// hourglass
    Hourglass,
    /// hourglass-bottom
    HourglassBottom,
    /// hourglass-split
    HourglassSplit,
    /// hourglass-top
    HourglassTop,
    /// house
    House,
    /// house-door
    HouseDoor,
    /// house-door-fill
    HouseDoorFill,
    /// house-fill
    HouseFill,
    /// hr
    Hr,
    /// image
    Image,
    /// image-alt
    ImageAlt,
    /// image-fill
    ImageFill,
    /// images
    Images,
    /// inbox
    Inbox,
    /// inbox-fill
    InboxFill,
    /// inboxes
    Inboxes,
    /// inboxes-fill
    InboxesFill,
    /// info
    Info,
    /// info-circle
    InfoCircle,
    /// info-circle-fill
    InfoCircleFill,
    /// info-square
    InfoSquare,
    /// info-square-fill
    InfoSquareFill,
    /// input-cursor
    InputCursor,
    /// input-cursor-text
    InputCursorText,
    /// instagram
    Instagram,
    /// intersect
    Intersect,
    /// journal
    Journal,
    /// journal-album
    JournalAlbum,
    /// journal-arrow-down
    JournalArrowDown,
    /// journal-arrow-up
    JournalArrowUp,
    /// journal-bookmark
    JournalBookmark,
    /// journal-bookmark-fill
    JournalBookmarkFill,
    /// journal-check
    JournalCheck,
    /// journal-code
    JournalCode,
    /// journal-medical
    JournalMedical,
    /// journal-minus
    JournalMinus,
    /// journal-plus
    JournalPlus,
    /// journal-richtext
    JournalRichtext,
    /// journal-text
    JournalText,
    /// journal-x
    JournalX,
    /// journals
    Journals,
    /// joystick
    Joystick,
    /// justify
    Justify,
    /// justify-left
    JustifyLeft,
    /// justify-right
    JustifyRight,
    /// kanban
    Kanban,
    /// kanban-fill
    KanbanFill,
    /// key
    Key,
    /// key-fill
    KeyFill,
    /// keyboard
    Keyboard,
    /// keyboard-fill
    KeyboardFill,
    /// ladder
    Ladder,
    /// lamp
    Lamp,
    /// lamp-fill
    LampFill,
    /// laptop
    Laptop,
    /// laptop-fill
    LaptopFill,
    /// layers
    Layers,
    /// layers-fill
    LayersFill,
    /// layers-half
    LayersHalf,
    /// layout-sidebar
    LayoutSidebar,
    /// layout-sidebar-inset
    LayoutSidebarInset,
    /// layout-sidebar-inset-reverse
    LayoutSidebarInsetReverse,
    /// layout-sidebar-reverse
    LayoutSidebarReverse,
    /// layout-split
    LayoutSplit,
    /// layout-text-sidebar
    LayoutTextSidebar,
    /// layout-text-sidebar-reverse
    LayoutTextSidebarReverse,
    /// layout-text-window
    LayoutTextWindow,
    /// layout-text-window-reverse
    LayoutTextWindowReverse,
    /// layout-three-columns
    LayoutThreeColumns,
    /// layout-wtf
    LayoutWtf,
    /// life-preserver
    LifePreserver,
    /// lightning
    Lightning,
    /// lightning-fill
    LightningFill,
    /// link
    Link,
    /// link-45deg
    Link45Deg,
    /// linkedin
    Linkedin,
    /// list
    List,
    /// list-check
    ListCheck,
    /// list-nested
    ListNested,
    /// list-ol
    ListOl,
    /// list-stars
    ListStars,
    /// list-task
    ListTask,
    /// list-ul
    ListUl,
    /// lock
    Lock,
    /// lock-fill
    LockFill,
    /// mailbox
    Mailbox,
    /// mailbox2
    Mailbox2,
    /// map
    Map,
    /// map-fill
    MapFill,
    /// markdown
    Markdown,
    /// markdown-fill
    MarkdownFill,
    /// menu-app
    MenuApp,
    /// menu-app-fill
    MenuAppFill,
    /// menu-button
    MenuButton,
    /// menu-button-fill
    MenuButtonFill,
    /// menu-button-wide
    MenuButtonWide,
    /// menu-button-wide-fill
    MenuButtonWideFill,
    /// menu-down
    MenuDown,
    /// menu-up
    MenuUp,
    /// mic
    Mic,
    /// mic-fill
    MicFill,
    /// mic-mute
    MicMute,
    /// mic-mute-fill
    MicMuteFill,
    /// minecart
    Minecart,
    /// minecart-loaded
    MinecartLoaded,
    /// moon
    Moon,
    /// mouse
    Mouse,
    /// mouse2
    Mouse2,
    /// mouse3
    Mouse3,
    /// music-note
    MusicNote,
    /// music-note-beamed
    MusicNoteBeamed,
    /// music-note-list
    MusicNoteList,
    /// music-player
    MusicPlayer,
    /// music-player-fill
    MusicPlayerFill,
    /// newspaper
    Newspaper,
    /// node-minus
    NodeMinus,
    /// node-minus-fill
    NodeMinusFill,
    /// node-plus
    NodePlus,
    /// node-plus-fill
    NodePlusFill,
    /// nut
    Nut,
    /// nut-fill
    NutFill,
    /// octagon
    Octagon,
    /// octagon-fill
    OctagonFill,
    /// octagon-half
    OctagonHalf,
    /// option
    Option,
    /// outlet
    Outlet,
    /// paperclip
    Paperclip,
    /// paragraph
    Paragraph,
    /// patch-check
    PatchCheck,
    /// patch-check-fll
    PatchCheckFll,
    /// patch-exclamation
    PatchExclamation,
    /// patch-exclamation-fll
    PatchExclamationFll,
    /// patch-minus
    PatchMinus,
    /// patch-minus-fll
    PatchMinusFll,
    /// patch-plus
    PatchPlus,
    /// patch-plus-fll
    PatchPlusFll,
    /// patch-question
    PatchQuestion,
    /// patch-question-fll
    PatchQuestionFll,
    /// pause
    Pause,
    /// pause-btn
    PauseBtn,
    /// pause-btn-fill
    PauseBtnFill,
    /// pause-circle
    PauseCircle,
    /// pause-circle-fill
    PauseCircleFill,
    /// pause-fill
    PauseFill,
    /// peace
    Peace,
    /// peace-fill
    PeaceFill,
    /// pen
    Pen,
    /// pen-fill
    PenFill,
    /// pencil
    Pencil,
    /// pencil-fill
    PencilFill,
    /// pencil-square
    PencilSquare,
    /// pentagon
    Pentagon,
    /// pentagon-fill
    PentagonFill,
    /// pentagon-half
    PentagonHalf,
    /// people
    People,
    /// people-fill
    PeopleFill,
    /// percent
    Percent,
    /// person
    Person,
    /// person-badge
    PersonBadge,
    /// person-badge-fill
    PersonBadgeFill,
    /// person-bounding-box
    PersonBoundingBox,
    /// person-check
    PersonCheck,
    /// person-check-fill
    PersonCheckFill,
    /// person-circle
    PersonCircle,
    /// person-dash
    PersonDash,
    /// person-dash-fill
    PersonDashFill,
    /// person-fill
    PersonFill,
    /// person-lines-fill
    PersonLinesFill,
    /// person-plus
    PersonPlus,
    /// person-plus-fill
    PersonPlusFill,
    /// person-square
    PersonSquare,
    /// person-x
    PersonX,
    /// person-x-fill
    PersonXFill,
    /// phone
    Phone,
    /// phone-fill
    PhoneFill,
    /// phone-landscape
    PhoneLandscape,
    /// phone-landscape-fill
    PhoneLandscapeFill,
    /// phone-vibrate
    PhoneVibrate,
    /// pie-chart
    PieChart,
    /// pie-chart-fill
    PieChartFill,
    /// pip
    Pip,
    /// pip-fill
    PipFill,
    /// play
    Play,
    /// play-btn
    PlayBtn,
    /// play-btn-fill
    PlayBtnFill,
    /// play-circle
    PlayCircle,
    /// play-circle-fill
    PlayCircleFill,
    /// play-fill
    PlayFill,
    /// plug
    Plug,
    /// plug-fill
    PlugFill,
    /// plus
    Plus,
    /// plus-circle
    PlusCircle,
    /// plus-circle-fill
    PlusCircleFill,
    /// plus-square
    PlusSquare,
    /// plus-square-fill
    PlusSquareFill,
    /// power
    Power,
    /// printer
    Printer,
    /// printer-fill
    PrinterFill,
    /// puzzle
    Puzzle,
    /// puzzle-fill
    PuzzleFill,
    /// question
    Question,
    /// question-circle
    QuestionCircle,
    /// question-circle-fill
    QuestionCircleFill,
    /// question-diamond
    QuestionDiamond,
    /// question-diamond-fill
    QuestionDiamondFill,
    /// question-octagon
    QuestionOctagon,
    /// question-octagon-fill
    QuestionOctagonFill,
    /// question-square
    QuestionSquare,
    /// question-square-fill
    QuestionSquareFill,
    /// receipt
    Receipt,
    /// receipt-cutoff
    ReceiptCutoff,
    /// reception-0
    Reception0,
    /// reception-1
    Reception1,
    /// reception-2
    Reception2,
    /// reception-3
    Reception3,
    /// reception-4
    Reception4,
    /// record
    Record,
    /// record-btn
    RecordBtn,
    /// record-btn-fill
    RecordBtnFill,
    /// record-circle
    RecordCircle,
    /// record-circle-fill
    RecordCircleFill,
    /// record-fill
    RecordFill,
    /// record2
    Record2,
    /// record2-fill
    Record2Fill,
    /// reply
    Reply,
    /// reply-all
    ReplyAll,
    /// reply-all-fill
    ReplyAllFill,
    /// reply-fill
    ReplyFill,
    /// rss
    Rss,
    /// rss-fill
    RssFill,
    /// scissors
    Scissors,
    /// screwdriver
    Screwdriver,
    /// search
    Search,
    /// segmented-nav
    SegmentedNav,
    /// server
    Server,
    /// share
    Share,
    /// share-fill
    ShareFill,
    /// shield
    Shield,
    /// shield-check
    ShieldCheck,
    /// shield-exclamation
    ShieldExclamation,
    /// shield-fill
    ShieldFill,
    /// shield-fill-check
    ShieldFillCheck,
    /// shield-fill-exclamation
    ShieldFillExclamation,
    /// shield-fill-minus
    ShieldFillMinus,
    /// shield-fill-plus
    ShieldFillPlus,
    /// shield-fill-x
    ShieldFillX,
    /// shield-lock
    ShieldLock,
    /// shield-lock-fill
    ShieldLockFill,
    /// shield-minus
    ShieldMinus,
    /// shield-plus
    ShieldPlus,
    /// shield-shaded
    ShieldShaded,
    /// shield-slash
    ShieldSlash,
    /// shield-slash-fill
    ShieldSlashFill,
    /// shield-x
    ShieldX,
    /// shift
    Shift,
    /// shift-fill
    ShiftFill,
    /// shop
    Shop,
    /// shop-window
    ShopWindow,
    /// shuffle
    Shuffle,
    /// signpost
    Signpost,
    /// signpost-2
    Signpost2,
    /// signpost-2-fill
    Signpost2Fill,
    /// signpost-fill
    SignpostFill,
    /// signpost-split
    SignpostSplit,
    /// signpost-split-fill
    SignpostSplitFill,
    /// sim
    Sim,
    /// sim-fill
    SimFill,
    /// skip-backward
    SkipBackward,
    /// skip-backward-btn
    SkipBackwardBtn,
    /// skip-backward-btn-fill
    SkipBackwardBtnFill,
    /// skip-backward-circle
    SkipBackwardCircle,
    /// skip-backward-circle-fill
    SkipBackwardCircleFill,
    /// skip-backward-fill
    SkipBackwardFill,
    /// skip-end
    SkipEnd,
    /// skip-end-btn
    SkipEndBtn,
    /// skip-end-btn-fill
    SkipEndBtnFill,
    /// skip-end-circle
    SkipEndCircle,
    /// skip-end-circle-fill
    SkipEndCircleFill,
    /// skip-end-fill
    SkipEndFill,
    /// skip-forward
    SkipForward,
    /// skip-forward-btn
    SkipForwardBtn,
    /// skip-forward-btn-fill
    SkipForwardBtnFill,
    /// skip-forward-circle
    SkipForwardCircle,
    /// skip-forward-circle-fill
    SkipForwardCircleFill,
    /// skip-forward-fill
    SkipForwardFill,
    /// skip-start
    SkipStart,
    /// skip-start-btn
    SkipStartBtn,
    /// skip-start-btn-fill
    SkipStartBtnFill,
    /// skip-start-circle
    SkipStartCircle,
    /// skip-start-circle-fill
    SkipStartCircleFill,
    /// skip-start-fill
    SkipStartFill,
    /// slack
    Slack,
    /// slash
    Slash,
    /// slash-circle
    SlashCircle,
    /// slash-circle-fill
    SlashCircleFill,
    /// slash-square
    SlashSquare,
    /// slash-square-fill
    SlashSquareFill,
    /// sliders
    Sliders,
    /// smartwatch
    Smartwatch,
    /// sort-alpha-down
    SortAlphaDown,
    /// sort-alpha-down-alt
    SortAlphaDownAlt,
    /// sort-alpha-up
    SortAlphaUp,
    /// sort-alpha-up-alt
    SortAlphaUpAlt,
    /// sort-down
    SortDown,
    /// sort-down-alt
    SortDownAlt,
    /// sort-numeric-down
    SortNumericDown,
    /// sort-numeric-down-alt
    SortNumericDownAlt,
    /// sort-numeric-up
    SortNumericUp,
    /// sort-numeric-up-alt
    SortNumericUpAlt,
    /// sort-up
    SortUp,
    /// sort-up-alt
    SortUpAlt,
    /// soundwave
    Soundwave,
    /// speaker
    Speaker,
    /// speaker-fill
    SpeakerFill,
    /// spellcheck
    Spellcheck,
    /// square
    Square,
    /// square-fill
    SquareFill,
    /// square-half
    SquareHalf,
    /// star
    Star,
    /// star-fill
    StarFill,
    /// star-half
    StarHalf,
    /// stickies
    Stickies,
    /// stickies-fill
    StickiesFill,
    /// sticky
    Sticky,
    /// sticky-fill
    StickyFill,
    /// stop
    Stop,
    /// stop-btn
    StopBtn,
    /// stop-btn-fill
    StopBtnFill,
    /// stop-circle
    StopCircle,
    /// stop-circle-fill
    StopCircleFill,
    /// stop-fill
    StopFill,
    /// stoplights
    Stoplights,
    /// stoplights-fill
    StoplightsFill,
    /// stopwatch
    Stopwatch,
    /// stopwatch-fill
    StopwatchFill,
    /// subtract
    Subtract,
    /// suit-club
    SuitClub,
    /// suit-club-fill
    SuitClubFill,
    /// suit-diamond
    SuitDiamond,
    /// suit-diamond-fill
    SuitDiamondFill,
    /// suit-heart
    SuitHeart,
    /// suit-heart-fill
    SuitHeartFill,
    /// suit-spade
    SuitSpade,
    /// suit-spade-fill
    SuitSpadeFill,
    /// sun
    Sun,
    /// sunglasses
    Sunglasses,
    /// table
    Table,
    /// tablet
    Tablet,
    /// tablet-fill
    TabletFill,
    /// tablet-landscape
    TabletLandscape,
    /// tablet-landscape-fill
    TabletLandscapeFill,
    /// tag
    Tag,
    /// tag-fill
    TagFill,
    /// tags
    Tags,
    /// tags-fill
    TagsFill,
    /// telephone
    Telephone,
    /// telephone-fill
    TelephoneFill,
    /// telephone-forward
    TelephoneForward,
    /// telephone-forward-fill
    TelephoneForwardFill,
    /// telephone-inbound
    TelephoneInbound,
    /// telephone-inbound-fill
    TelephoneInboundFill,
    /// telephone-minus
    TelephoneMinus,
    /// telephone-minus-fill
    TelephoneMinusFill,
    /// telephone-outbound
    TelephoneOutbound,
    /// telephone-outbound-fill
    TelephoneOutboundFill,
    /// telephone-plus
    TelephonePlus,
    /// telephone-plus-fill
    TelephonePlusFill,
    /// telephone-x
    TelephoneX,
    /// telephone-x-fill
    TelephoneXFill,
    /// terminal
    Terminal,
    /// terminal-fill
    TerminalFill,
    /// text-center
    TextCenter,
    /// text-indent-left
    TextIndentLeft,
    /// text-indent-right
    TextIndentRight,
    /// text-left
    TextLeft,
    /// text-paragraph
    TextParagraph,
    /// text-right
    TextRight,
    /// textarea
    Textarea,
    /// textarea-resize
    TextareaResize,
    /// textarea-t
    TextareaT,
    /// thermometer
    Thermometer,
    /// thermometer-half
    ThermometerHalf,
    /// three-dots
    ThreeDots,
    /// three-dots-vertical
    ThreeDotsVertical,
    /// toggle-off
    ToggleOff,
    /// toggle-on
    ToggleOn,
    /// toggle2-off
    Toggle2Off,
    /// toggle2-on
    Toggle2On,
    /// toggles
    Toggles,
    /// toggles2
    Toggles2,
    /// tools
    Tools,
    /// trash
    Trash,
    /// trash-fill
    TrashFill,
    /// trash2
    Trash2,
    /// trash2-fill
    Trash2Fill,
    /// tree
    Tree,
    /// tree-fill
    TreeFill,
    /// triangle
    Triangle,
    /// triangle-fill
    TriangleFill,
    /// triangle-half
    TriangleHalf,
    /// trophy
    Trophy,
    /// trophy-fill
    TrophyFill,
    /// truck
    Truck,
    /// truck-flatbed
    TruckFlatbed,
    /// tv
    Tv,
    /// tv-fill
    TvFill,
    /// twitch
    Twitch,
    /// twitter
    Twitter,
    /// type
    Type,
    /// type-bold
    TypeBold,
    /// type-h1
    TypeH1,
    /// type-h2
    TypeH2,
    /// type-h3
    TypeH3,
    /// type-italic
    TypeItalic,
    /// type-strikethrough
    TypeStrikethrough,
    /// type-underline
    TypeUnderline,
    /// ui-checks
    UiChecks,
    /// ui-checks-grid
    UiChecksGrid,
    /// ui-radios
    UiRadios,
    /// ui-radios-grid
    UiRadiosGrid,
    /// union
    Union,
    /// unlock
    Unlock,
    /// unlock-fill
    UnlockFill,
    /// upc
    Upc,
    /// upc-scan
    UpcScan,
    /// upload
    Upload,
    /// vector-pen
    VectorPen,
    /// view-list
    ViewList,
    /// view-stacked
    ViewStacked,
    /// vinyl
    Vinyl,
    /// vinyl-fill
    VinylFill,
    /// voicemail
    Voicemail,
    /// volume-down
    VolumeDown,
    /// volume-down-fill
    VolumeDownFill,
    /// volume-mute
    VolumeMute,
    /// volume-mute-fill
    VolumeMuteFill,
    /// volume-off
    VolumeOff,
    /// volume-off-fill
    VolumeOffFill,
    /// volume-up
    VolumeUp,
    /// volume-up-fill
    VolumeUpFill,
    /// vr
    Vr,
    /// wallet
    Wallet,
    /// wallet-fill
    WalletFill,
    /// wallet2
    Wallet2,
    /// watch
    Watch,
    /// wifi
    Wifi,
    /// wifi-1
    Wifi1,
    /// wifi-2
    Wifi2,
    /// wifi-off
    WifiOff,
    /// window
    Window,
    /// wrench
    Wrench,
    /// x
    X,
    /// x-circle
    XCircle,
    /// x-circle-fill
    XCircleFill,
    /// x-diamond
    XDiamond,
    /// x-diamond-fill
    XDiamondFill,
    /// x-octagon
    XOctagon,
    /// x-octagon-fill
    XOctagonFill,
    /// x-square
    XSquare,
    /// x-square-fill
    XSquareFill,
    /// youtube
    Youtube,
    /// zoom-in
    ZoomIn,
    /// zoom-out
    ZoomOut,
}

impl IpgIcon {
    pub fn extract(update_obj: &PyObject) -> IpgIcon {
        Python::attach(|py| {
            let res = update_obj.extract::<IpgIcon>(py);
            match res {
                Ok(update) => update,
                Err(_) => panic!("IpgIcon update extraction failed"),
            }
        })
    }
}

/// Converts an icon into a char.
#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn icon_to_char(icon: IpgIcon) -> char {
    match icon {
        IpgIcon::Alarm => '\u{f102}',
        IpgIcon::AlarmFill => '\u{f101}',
        IpgIcon::AlignBottom => '\u{f103}',
        IpgIcon::AlignCenter => '\u{f104}',
        IpgIcon::AlignEnd => '\u{f105}',
        IpgIcon::AlignMiddle => '\u{f106}',
        IpgIcon::AlignStart => '\u{f107}',
        IpgIcon::AlignTop => '\u{f108}',
        IpgIcon::Alt => '\u{f109}',
        IpgIcon::App => '\u{f10b}',
        IpgIcon::AppIndicator => '\u{f10a}',
        IpgIcon::Archive => '\u{f10d}',
        IpgIcon::ArchiveFill => '\u{f10c}',
        IpgIcon::Arrow90DegDown => '\u{f10e}',
        IpgIcon::Arrow90DegLeft => '\u{f10f}',
        IpgIcon::Arrow90DegRight => '\u{f110}',
        IpgIcon::Arrow90DegUp => '\u{f111}',
        IpgIcon::ArrowBarDown => '\u{f112}',
        IpgIcon::ArrowBarLeft => '\u{f113}',
        IpgIcon::ArrowBarRight => '\u{f114}',
        IpgIcon::ArrowBarUp => '\u{f115}',
        IpgIcon::ArrowClockwise => '\u{f116}',
        IpgIcon::ArrowCounterclockwise => '\u{f117}',
        IpgIcon::ArrowDown => '\u{f128}',
        IpgIcon::ArrowDownCircle => '\u{f119}',
        IpgIcon::ArrowDownCircleFill => '\u{f118}',
        IpgIcon::ArrowDownLeft => '\u{f11e}',
        IpgIcon::ArrowDownLeftCircle => '\u{f11b}',
        IpgIcon::ArrowDownLeftCircleFill => '\u{f11a}',
        IpgIcon::ArrowDownLeftSquare => '\u{f11d}',
        IpgIcon::ArrowDownLeftSquareFill => '\u{f11c}',
        IpgIcon::ArrowDownRight => '\u{f123}',
        IpgIcon::ArrowDownRightCircle => '\u{f120}',
        IpgIcon::ArrowDownRightCircleFill => '\u{f11f}',
        IpgIcon::ArrowDownRightSquare => '\u{f122}',
        IpgIcon::ArrowDownRightSquareFill => '\u{f121}',
        IpgIcon::ArrowDownShort => '\u{f124}',
        IpgIcon::ArrowDownSquare => '\u{f126}',
        IpgIcon::ArrowDownSquareFill => '\u{f125}',
        IpgIcon::ArrowDownUp => '\u{f127}',
        IpgIcon::ArrowLeft => '\u{f12f}',
        IpgIcon::ArrowLeftCircle => '\u{f12a}',
        IpgIcon::ArrowLeftCircleFill => '\u{f129}',
        IpgIcon::ArrowLeftRight => '\u{f12b}',
        IpgIcon::ArrowLeftShort => '\u{f12c}',
        IpgIcon::ArrowLeftSquare => '\u{f12e}',
        IpgIcon::ArrowLeftSquareFill => '\u{f12d}',
        IpgIcon::ArrowRepeat => '\u{f130}',
        IpgIcon::ArrowReturnLeft => '\u{f131}',
        IpgIcon::ArrowReturnRight => '\u{f132}',
        IpgIcon::ArrowRight => '\u{f138}',
        IpgIcon::ArrowRightCircle => '\u{f134}',
        IpgIcon::ArrowRightCircleFill => '\u{f133}',
        IpgIcon::ArrowRightShort => '\u{f135}',
        IpgIcon::ArrowRightSquare => '\u{f137}',
        IpgIcon::ArrowRightSquareFill => '\u{f136}',
        IpgIcon::ArrowUp => '\u{f148}',
        IpgIcon::ArrowUpCircle => '\u{f13a}',
        IpgIcon::ArrowUpCircleFill => '\u{f139}',
        IpgIcon::ArrowUpLeft => '\u{f13f}',
        IpgIcon::ArrowUpLeftCircle => '\u{f13c}',
        IpgIcon::ArrowUpLeftCircleFill => '\u{f13b}',
        IpgIcon::ArrowUpLeftSquare => '\u{f13e}',
        IpgIcon::ArrowUpLeftSquareFill => '\u{f13d}',
        IpgIcon::ArrowUpRight => '\u{f144}',
        IpgIcon::ArrowUpRightCircle => '\u{f141}',
        IpgIcon::ArrowUpRightCircleFill => '\u{f140}',
        IpgIcon::ArrowUpRightSquare => '\u{f143}',
        IpgIcon::ArrowUpRightSquareFill => '\u{f142}',
        IpgIcon::ArrowUpShort => '\u{f145}',
        IpgIcon::ArrowUpSquare => '\u{f147}',
        IpgIcon::ArrowUpSquareFill => '\u{f146}',
        IpgIcon::ArrowsAngleContract => '\u{f149}',
        IpgIcon::ArrowsAngleExpand => '\u{f14a}',
        IpgIcon::ArrowsCollapse => '\u{f14b}',
        IpgIcon::ArrowsExpand => '\u{f14c}',
        IpgIcon::ArrowsFullscreen => '\u{f14d}',
        IpgIcon::ArrowsMove => '\u{f14e}',
        IpgIcon::AspectRatio => '\u{f150}',
        IpgIcon::AspectRatioFill => '\u{f14f}',
        IpgIcon::Asterisk => '\u{f151}',
        IpgIcon::At => '\u{f152}',
        IpgIcon::Award => '\u{f154}',
        IpgIcon::AwardFill => '\u{f153}',
        IpgIcon::Back => '\u{f155}',
        IpgIcon::Backspace => '\u{f159}',
        IpgIcon::BackspaceFill => '\u{f156}',
        IpgIcon::BackspaceReverse => '\u{f158}',
        IpgIcon::BackspaceReverseFill => '\u{f157}',
        IpgIcon::Badge4K => '\u{f15b}',
        IpgIcon::Badge4KFill => '\u{f15a}',
        IpgIcon::Badge8K => '\u{f15d}',
        IpgIcon::Badge8KFill => '\u{f15c}',
        IpgIcon::BadgeAd => '\u{f15f}',
        IpgIcon::BadgeAdFill => '\u{f15e}',
        IpgIcon::BadgeCc => '\u{f161}',
        IpgIcon::BadgeCcFill => '\u{f160}',
        IpgIcon::BadgeHd => '\u{f163}',
        IpgIcon::BadgeHdFill => '\u{f162}',
        IpgIcon::BadgeTm => '\u{f165}',
        IpgIcon::BadgeTmFill => '\u{f164}',
        IpgIcon::BadgeVo => '\u{f167}',
        IpgIcon::BadgeVoFill => '\u{f166}',
        IpgIcon::Bag => '\u{f171}',
        IpgIcon::BagCheck => '\u{f169}',
        IpgIcon::BagCheckFill => '\u{f168}',
        IpgIcon::BagDash => '\u{f16b}',
        IpgIcon::BagDashFill => '\u{f16a}',
        IpgIcon::BagFill => '\u{f16c}',
        IpgIcon::BagPlus => '\u{f16e}',
        IpgIcon::BagPlusFill => '\u{f16d}',
        IpgIcon::BagX => '\u{f170}',
        IpgIcon::BagXFill => '\u{f16f}',
        IpgIcon::BarChart => '\u{f176}',
        IpgIcon::BarChartFill => '\u{f172}',
        IpgIcon::BarChartLine => '\u{f174}',
        IpgIcon::BarChartLineFill => '\u{f173}',
        IpgIcon::BarChartSteps => '\u{f175}',
        IpgIcon::Basket => '\u{f178}',
        IpgIcon::BasketFill => '\u{f177}',
        IpgIcon::Basket2 => '\u{f17a}',
        IpgIcon::Basket2Fill => '\u{f179}',
        IpgIcon::Basket3 => '\u{f17c}',
        IpgIcon::Basket3Fill => '\u{f17b}',
        IpgIcon::Battery => '\u{f180}',
        IpgIcon::BatteryCharging => '\u{f17d}',
        IpgIcon::BatteryFull => '\u{f17e}',
        IpgIcon::BatteryHalf => '\u{f17f}',
        IpgIcon::Bell => '\u{f182}',
        IpgIcon::BellFill => '\u{f181}',
        IpgIcon::Bezier => '\u{f183}',
        IpgIcon::Bezier2 => '\u{f184}',
        IpgIcon::Bicycle => '\u{f185}',
        IpgIcon::Binoculars => '\u{f187}',
        IpgIcon::BinocularsFill => '\u{f186}',
        IpgIcon::BlockquoteLeft => '\u{f188}',
        IpgIcon::BlockquoteRight => '\u{f189}',
        IpgIcon::Book => '\u{f18c}',
        IpgIcon::BookFill => '\u{f18a}',
        IpgIcon::BookHalf => '\u{f18b}',
        IpgIcon::Bookmark => '\u{f19a}',
        IpgIcon::BookmarkCheck => '\u{f18e}',
        IpgIcon::BookmarkCheckFill => '\u{f18d}',
        IpgIcon::BookmarkDash => '\u{f190}',
        IpgIcon::BookmarkDashFill => '\u{f18f}',
        IpgIcon::BookmarkFill => '\u{f191}',
        IpgIcon::BookmarkHeart => '\u{f193}',
        IpgIcon::BookmarkHeartFill => '\u{f192}',
        IpgIcon::BookmarkPlus => '\u{f195}',
        IpgIcon::BookmarkPlusFill => '\u{f194}',
        IpgIcon::BookmarkStar => '\u{f197}',
        IpgIcon::BookmarkStarFill => '\u{f196}',
        IpgIcon::BookmarkX => '\u{f199}',
        IpgIcon::BookmarkXFill => '\u{f198}',
        IpgIcon::Bookmarks => '\u{f19c}',
        IpgIcon::BookmarksFill => '\u{f19b}',
        IpgIcon::Bookshelf => '\u{f19d}',
        IpgIcon::Bootstrap => '\u{f1a0}',
        IpgIcon::BootstrapFill => '\u{f19e}',
        IpgIcon::BootstrapReboot => '\u{f19f}',
        IpgIcon::BorderStyle => '\u{f1a1}',
        IpgIcon::BorderWidth => '\u{f1a2}',
        IpgIcon::BoundingBox => '\u{f1a4}',
        IpgIcon::BoundingBoxCircles => '\u{f1a3}',
        IpgIcon::Box => '\u{f1b6}',
        IpgIcon::BoxArrowDown => '\u{f1a7}',
        IpgIcon::BoxArrowDownLeft => '\u{f1a5}',
        IpgIcon::BoxArrowDownRight => '\u{f1a6}',
        IpgIcon::BoxArrowInDown => '\u{f1aa}',
        IpgIcon::BoxArrowInDownLeft => '\u{f1a8}',
        IpgIcon::BoxArrowInDownRight => '\u{f1a9}',
        IpgIcon::BoxArrowInLeft => '\u{f1ab}',
        IpgIcon::BoxArrowInRight => '\u{f1ac}',
        IpgIcon::BoxArrowInUp => '\u{f1af}',
        IpgIcon::BoxArrowInUpLeft => '\u{f1ad}',
        IpgIcon::BoxArrowInUpRight => '\u{f1ae}',
        IpgIcon::BoxArrowLeft => '\u{f1b0}',
        IpgIcon::BoxArrowRight => '\u{f1b1}',
        IpgIcon::BoxArrowUp => '\u{f1b4}',
        IpgIcon::BoxArrowUpLeft => '\u{f1b2}',
        IpgIcon::BoxArrowUpRight => '\u{f1b3}',
        IpgIcon::BoxSeam => '\u{f1b5}',
        IpgIcon::Braces => '\u{f1b7}',
        IpgIcon::Bricks => '\u{f1b8}',
        IpgIcon::Briefcase => '\u{f1ba}',
        IpgIcon::BriefcaseFill => '\u{f1b9}',
        IpgIcon::BrightnessAltHigh => '\u{f1bc}',
        IpgIcon::BrightnessAltHighFill => '\u{f1bb}',
        IpgIcon::BrightnessAltLow => '\u{f1be}',
        IpgIcon::BrightnessAltLowFill => '\u{f1bd}',
        IpgIcon::BrightnessHigh => '\u{f1c0}',
        IpgIcon::BrightnessHighFill => '\u{f1bf}',
        IpgIcon::BrightnessLow => '\u{f1c2}',
        IpgIcon::BrightnessLowFill => '\u{f1c1}',
        IpgIcon::Broadcast => '\u{f1c4}',
        IpgIcon::BroadcastPin => '\u{f1c3}',
        IpgIcon::Brush => '\u{f1c6}',
        IpgIcon::BrushFill => '\u{f1c5}',
        IpgIcon::Bucket => '\u{f1c8}',
        IpgIcon::BucketFill => '\u{f1c7}',
        IpgIcon::Bug => '\u{f1ca}',
        IpgIcon::BugFill => '\u{f1c9}',
        IpgIcon::Building => '\u{f1cb}',
        IpgIcon::Bullseye => '\u{f1cc}',
        IpgIcon::Calculator => '\u{f1ce}',
        IpgIcon::CalculatorFill => '\u{f1cd}',
        IpgIcon::Calendar => '\u{f1e4}',
        IpgIcon::CalendarCheck => '\u{f1d0}',
        IpgIcon::CalendarCheckFill => '\u{f1cf}',
        IpgIcon::CalendarDate => '\u{f1d2}',
        IpgIcon::CalendarDateFill => '\u{f1d1}',
        IpgIcon::CalendarDay => '\u{f1d4}',
        IpgIcon::CalendarDayFill => '\u{f1d3}',
        IpgIcon::CalendarEvent => '\u{f1d6}',
        IpgIcon::CalendarEventFill => '\u{f1d5}',
        IpgIcon::CalendarFill => '\u{f1d7}',
        IpgIcon::CalendarMinus => '\u{f1d9}',
        IpgIcon::CalendarMinusFill => '\u{f1d8}',
        IpgIcon::CalendarMonth => '\u{f1db}',
        IpgIcon::CalendarMonthFill => '\u{f1da}',
        IpgIcon::CalendarPlus => '\u{f1dd}',
        IpgIcon::CalendarPlusFill => '\u{f1dc}',
        IpgIcon::CalendarRange => '\u{f1df}',
        IpgIcon::CalendarRangeFill => '\u{f1de}',
        IpgIcon::CalendarWeek => '\u{f1e1}',
        IpgIcon::CalendarWeekFill => '\u{f1e0}',
        IpgIcon::CalendarX => '\u{f1e3}',
        IpgIcon::CalendarXFill => '\u{f1e2}',
        IpgIcon::Calendar2 => '\u{f1fa}',
        IpgIcon::Calendar2Check => '\u{f1e6}',
        IpgIcon::Calendar2CheckFill => '\u{f1e5}',
        IpgIcon::Calendar2Date => '\u{f1e8}',
        IpgIcon::Calendar2DateFill => '\u{f1e7}',
        IpgIcon::Calendar2Day => '\u{f1ea}',
        IpgIcon::Calendar2DayFill => '\u{f1e9}',
        IpgIcon::Calendar2Event => '\u{f1ec}',
        IpgIcon::Calendar2EventFill => '\u{f1eb}',
        IpgIcon::Calendar2Fill => '\u{f1ed}',
        IpgIcon::Calendar2Minus => '\u{f1ef}',
        IpgIcon::Calendar2MinusFill => '\u{f1ee}',
        IpgIcon::Calendar2Month => '\u{f1f1}',
        IpgIcon::Calendar2MonthFill => '\u{f1f0}',
        IpgIcon::Calendar2Plus => '\u{f1f3}',
        IpgIcon::Calendar2PlusFill => '\u{f1f2}',
        IpgIcon::Calendar2Range => '\u{f1f5}',
        IpgIcon::Calendar2RangeFill => '\u{f1f4}',
        IpgIcon::Calendar2Week => '\u{f1f7}',
        IpgIcon::Calendar2WeekFill => '\u{f1f6}',
        IpgIcon::Calendar2X => '\u{f1f9}',
        IpgIcon::Calendar2XFill => '\u{f1f8}',
        IpgIcon::Calendar3 => '\u{f202}',
        IpgIcon::Calendar3Event => '\u{f1fc}',
        IpgIcon::Calendar3EventFill => '\u{f1fb}',
        IpgIcon::Calendar3Fill => '\u{f1fd}',
        IpgIcon::Calendar3Range => '\u{f1ff}',
        IpgIcon::Calendar3RangeFill => '\u{f1fe}',
        IpgIcon::Calendar3Week => '\u{f201}',
        IpgIcon::Calendar3WeekFill => '\u{f200}',
        IpgIcon::Calendar4 => '\u{f206}',
        IpgIcon::Calendar4Event => '\u{f203}',
        IpgIcon::Calendar4Range => '\u{f204}',
        IpgIcon::Calendar4Week => '\u{f205}',
        IpgIcon::Camera => '\u{f20e}',
        IpgIcon::CameraFill => '\u{f207}',
        IpgIcon::CameraReels => '\u{f209}',
        IpgIcon::CameraReelsFill => '\u{f208}',
        IpgIcon::CameraVideo => '\u{f20d}',
        IpgIcon::CameraVideoFill => '\u{f20a}',
        IpgIcon::CameraVideoOff => '\u{f20c}',
        IpgIcon::CameraVideoOffFill => '\u{f20b}',
        IpgIcon::Camera2 => '\u{f20f}',
        IpgIcon::Capslock => '\u{f211}',
        IpgIcon::CapslockFill => '\u{f210}',
        IpgIcon::CardChecklist => '\u{f212}',
        IpgIcon::CardHeading => '\u{f213}',
        IpgIcon::CardImage => '\u{f214}',
        IpgIcon::CardList => '\u{f215}',
        IpgIcon::CardText => '\u{f216}',
        IpgIcon::CaretDown => '\u{f21a}',
        IpgIcon::CaretDownFill => '\u{f217}',
        IpgIcon::CaretDownSquare => '\u{f219}',
        IpgIcon::CaretDownSquareFill => '\u{f218}',
        IpgIcon::CaretLeft => '\u{f21e}',
        IpgIcon::CaretLeftFill => '\u{f21b}',
        IpgIcon::CaretLeftSquare => '\u{f21d}',
        IpgIcon::CaretLeftSquareFill => '\u{f21c}',
        IpgIcon::CaretRight => '\u{f222}',
        IpgIcon::CaretRightFill => '\u{f21f}',
        IpgIcon::CaretRightSquare => '\u{f221}',
        IpgIcon::CaretRightSquareFill => '\u{f220}',
        IpgIcon::CaretUp => '\u{f226}',
        IpgIcon::CaretUpFill => '\u{f223}',
        IpgIcon::CaretUpSquare => '\u{f225}',
        IpgIcon::CaretUpSquareFill => '\u{f224}',
        IpgIcon::Cart => '\u{f230}',
        IpgIcon::CartCheck => '\u{f228}',
        IpgIcon::CartCheckFill => '\u{f227}',
        IpgIcon::CartDash => '\u{f22a}',
        IpgIcon::CartDashFill => '\u{f229}',
        IpgIcon::CartFill => '\u{f22b}',
        IpgIcon::CartPlus => '\u{f22d}',
        IpgIcon::CartPlusFill => '\u{f22c}',
        IpgIcon::CartX => '\u{f22f}',
        IpgIcon::CartXFill => '\u{f22e}',
        IpgIcon::Cart2 => '\u{f231}',
        IpgIcon::Cart3 => '\u{f232}',
        IpgIcon::Cart4 => '\u{f233}',
        IpgIcon::Cash => '\u{f235}',
        IpgIcon::CashStack => '\u{f234}',
        IpgIcon::Cast => '\u{f236}',
        IpgIcon::Chat => '\u{f256}',
        IpgIcon::ChatDots => '\u{f238}',
        IpgIcon::ChatDotsFill => '\u{f237}',
        IpgIcon::ChatFill => '\u{f239}',
        IpgIcon::ChatLeft => '\u{f241}',
        IpgIcon::ChatLeftDots => '\u{f23b}',
        IpgIcon::ChatLeftDotsFill => '\u{f23a}',
        IpgIcon::ChatLeftFill => '\u{f23c}',
        IpgIcon::ChatLeftQuote => '\u{f23e}',
        IpgIcon::ChatLeftQuoteFill => '\u{f23d}',
        IpgIcon::ChatLeftText => '\u{f240}',
        IpgIcon::ChatLeftTextFill => '\u{f23f}',
        IpgIcon::ChatQuote => '\u{f243}',
        IpgIcon::ChatQuoteFill => '\u{f242}',
        IpgIcon::ChatRight => '\u{f24b}',
        IpgIcon::ChatRightDots => '\u{f245}',
        IpgIcon::ChatRightDotsFill => '\u{f244}',
        IpgIcon::ChatRightFill => '\u{f246}',
        IpgIcon::ChatRightQuote => '\u{f248}',
        IpgIcon::ChatRightQuoteFill => '\u{f247}',
        IpgIcon::ChatRightText => '\u{f24a}',
        IpgIcon::ChatRightTextFill => '\u{f249}',
        IpgIcon::ChatSquare => '\u{f253}',
        IpgIcon::ChatSquareDots => '\u{f24d}',
        IpgIcon::ChatSquareDotsFill => '\u{f24c}',
        IpgIcon::ChatSquareFill => '\u{f24e}',
        IpgIcon::ChatSquareQuote => '\u{f250}',
        IpgIcon::ChatSquareQuoteFill => '\u{f24f}',
        IpgIcon::ChatSquareText => '\u{f252}',
        IpgIcon::ChatSquareTextFill => '\u{f251}',
        IpgIcon::ChatText => '\u{f255}',
        IpgIcon::ChatTextFill => '\u{f254}',
        IpgIcon::Check => '\u{f25c}',
        IpgIcon::CheckAll => '\u{f257}',
        IpgIcon::CheckCircle => '\u{f259}',
        IpgIcon::CheckCircleFill => '\u{f258}',
        IpgIcon::CheckSquare => '\u{f25b}',
        IpgIcon::CheckSquareFill => '\u{f25a}',
        IpgIcon::Check2 => '\u{f260}',
        IpgIcon::Check2All => '\u{f25d}',
        IpgIcon::Check2Circle => '\u{f25e}',
        IpgIcon::Check2Square => '\u{f25f}',
        IpgIcon::ChevronBarContract => '\u{f261}',
        IpgIcon::ChevronBarDown => '\u{f262}',
        IpgIcon::ChevronBarExpand => '\u{f263}',
        IpgIcon::ChevronBarLeft => '\u{f264}',
        IpgIcon::ChevronBarRight => '\u{f265}',
        IpgIcon::ChevronBarUp => '\u{f266}',
        IpgIcon::ChevronCompactDown => '\u{f267}',
        IpgIcon::ChevronCompactLeft => '\u{f268}',
        IpgIcon::ChevronCompactRight => '\u{f269}',
        IpgIcon::ChevronCompactUp => '\u{f26a}',
        IpgIcon::ChevronContract => '\u{f26b}',
        IpgIcon::ChevronDoubleDown => '\u{f26c}',
        IpgIcon::ChevronDoubleLeft => '\u{f26d}',
        IpgIcon::ChevronDoubleRight => '\u{f26e}',
        IpgIcon::ChevronDoubleUp => '\u{f26f}',
        IpgIcon::ChevronDown => '\u{f270}',
        IpgIcon::ChevronExpand => '\u{f271}',
        IpgIcon::ChevronLeft => '\u{f272}',
        IpgIcon::ChevronRight => '\u{f273}',
        IpgIcon::ChevronUp => '\u{f274}',
        IpgIcon::Circle => '\u{f278}',
        IpgIcon::CircleFill => '\u{f275}',
        IpgIcon::CircleHalf => '\u{f276}',
        IpgIcon::CircleSquare => '\u{f277}',
        IpgIcon::Clipboard => '\u{f27e}',
        IpgIcon::ClipboardCheck => '\u{f279}',
        IpgIcon::ClipboardData => '\u{f27a}',
        IpgIcon::ClipboardMinus => '\u{f27b}',
        IpgIcon::ClipboardPlus => '\u{f27c}',
        IpgIcon::ClipboardX => '\u{f27d}',
        IpgIcon::Clock => '\u{f281}',
        IpgIcon::ClockFill => '\u{f27f}',
        IpgIcon::ClockHistory => '\u{f280}',
        IpgIcon::Cloud => '\u{f293}',
        IpgIcon::CloudArrowDown => '\u{f283}',
        IpgIcon::CloudArrowDownFill => '\u{f282}',
        IpgIcon::CloudArrowUp => '\u{f285}',
        IpgIcon::CloudArrowUpFill => '\u{f284}',
        IpgIcon::CloudCheck => '\u{f287}',
        IpgIcon::CloudCheckFill => '\u{f286}',
        IpgIcon::CloudDownload => '\u{f289}',
        IpgIcon::CloudDownloadFill => '\u{f288}',
        IpgIcon::CloudFill => '\u{f28a}',
        IpgIcon::CloudMinus => '\u{f28c}',
        IpgIcon::CloudMinusFill => '\u{f28b}',
        IpgIcon::CloudPlus => '\u{f28e}',
        IpgIcon::CloudPlusFill => '\u{f28d}',
        IpgIcon::CloudSlash => '\u{f290}',
        IpgIcon::CloudSlashFill => '\u{f28f}',
        IpgIcon::CloudUpload => '\u{f292}',
        IpgIcon::CloudUploadFill => '\u{f291}',
        IpgIcon::Code => '\u{f296}',
        IpgIcon::CodeSlash => '\u{f294}',
        IpgIcon::CodeSquare => '\u{f295}',
        IpgIcon::Collection => '\u{f29a}',
        IpgIcon::CollectionFill => '\u{f297}',
        IpgIcon::CollectionPlay => '\u{f299}',
        IpgIcon::CollectionPlayFill => '\u{f298}',
        IpgIcon::Columns => '\u{f29c}',
        IpgIcon::ColumnsGap => '\u{f29b}',
        IpgIcon::Command => '\u{f29d}',
        IpgIcon::Compass => '\u{f29f}',
        IpgIcon::CompassFill => '\u{f29e}',
        IpgIcon::Cone => '\u{f2a1}',
        IpgIcon::ConeStriped => '\u{f2a0}',
        IpgIcon::Controller => '\u{f2a2}',
        IpgIcon::Cpu => '\u{f2a4}',
        IpgIcon::CpuFill => '\u{f2a3}',
        IpgIcon::CreditCard => '\u{f2aa}',
        IpgIcon::CreditCard2Back => '\u{f2a6}',
        IpgIcon::CreditCard2BackFill => '\u{f2a5}',
        IpgIcon::CreditCard2Front => '\u{f2a8}',
        IpgIcon::CreditCard2FrontFill => '\u{f2a7}',
        IpgIcon::CreditCardFill => '\u{f2a9}',
        IpgIcon::Crop => '\u{f2ab}',
        IpgIcon::Cup => '\u{f2ae}',
        IpgIcon::CupFill => '\u{f2ac}',
        IpgIcon::CupStraw => '\u{f2ad}',
        IpgIcon::Cursor => '\u{f2b1}',
        IpgIcon::CursorFill => '\u{f2af}',
        IpgIcon::CursorText => '\u{f2b0}',
        IpgIcon::Dash => '\u{f2b6}',
        IpgIcon::DashCircle => '\u{f2b3}',
        IpgIcon::DashCircleFill => '\u{f2b2}',
        IpgIcon::DashSquare => '\u{f2b5}',
        IpgIcon::DashSquareFill => '\u{f2b4}',
        IpgIcon::Diagram2 => '\u{f2b8}',
        IpgIcon::Diagram2Fill => '\u{f2b7}',
        IpgIcon::Diagram3 => '\u{f2ba}',
        IpgIcon::Diagram3Fill => '\u{f2b9}',
        IpgIcon::Diamond => '\u{f2bd}',
        IpgIcon::DiamondFill => '\u{f2bb}',
        IpgIcon::DiamondHalf => '\u{f2bc}',
        IpgIcon::Dice1 => '\u{f2bf}',
        IpgIcon::Dice1Fill => '\u{f2be}',
        IpgIcon::Dice2 => '\u{f2c1}',
        IpgIcon::Dice2Fill => '\u{f2c0}',
        IpgIcon::Dice3 => '\u{f2c3}',
        IpgIcon::Dice3Fill => '\u{f2c2}',
        IpgIcon::Dice4 => '\u{f2c5}',
        IpgIcon::Dice4Fill => '\u{f2c4}',
        IpgIcon::Dice5 => '\u{f2c7}',
        IpgIcon::Dice5Fill => '\u{f2c6}',
        IpgIcon::Dice6 => '\u{f2c9}',
        IpgIcon::Dice6Fill => '\u{f2c8}',
        IpgIcon::Disc => '\u{f2cb}',
        IpgIcon::DiscFill => '\u{f2ca}',
        IpgIcon::Discord => '\u{f2cc}',
        IpgIcon::Display => '\u{f2ce}',
        IpgIcon::DisplayFill => '\u{f2cd}',
        IpgIcon::DistributeHorizontal => '\u{f2cf}',
        IpgIcon::DistributeVertical => '\u{f2d0}',
        IpgIcon::DoorClosed => '\u{f2d2}',
        IpgIcon::DoorClosedFill => '\u{f2d1}',
        IpgIcon::DoorOpen => '\u{f2d4}',
        IpgIcon::DoorOpenFill => '\u{f2d3}',
        IpgIcon::Dot => '\u{f2d5}',
        IpgIcon::Download => '\u{f2d6}',
        IpgIcon::Droplet => '\u{f2d9}',
        IpgIcon::DropletFill => '\u{f2d7}',
        IpgIcon::DropletHalf => '\u{f2d8}',
        IpgIcon::Earbuds => '\u{f2da}',
        IpgIcon::Easel => '\u{f2dc}',
        IpgIcon::EaselFill => '\u{f2db}',
        IpgIcon::Egg => '\u{f2df}',
        IpgIcon::EggFill => '\u{f2dd}',
        IpgIcon::EggFried => '\u{f2de}',
        IpgIcon::Eject => '\u{f2e1}',
        IpgIcon::EjectFill => '\u{f2e0}',
        IpgIcon::EmojiAngry => '\u{f2e3}',
        IpgIcon::EmojiAngryFill => '\u{f2e2}',
        IpgIcon::EmojiDizzy => '\u{f2e5}',
        IpgIcon::EmojiDizzyFill => '\u{f2e4}',
        IpgIcon::EmojiExpressionless => '\u{f2e7}',
        IpgIcon::EmojiExpressionlessFill => '\u{f2e6}',
        IpgIcon::EmojiFrown => '\u{f2e9}',
        IpgIcon::EmojiFrownFill => '\u{f2e8}',
        IpgIcon::EmojiHeartEyes => '\u{f2eb}',
        IpgIcon::EmojiHeartEyesFill => '\u{f2ea}',
        IpgIcon::EmojiLaughing => '\u{f2ed}',
        IpgIcon::EmojiLaughingFill => '\u{f2ec}',
        IpgIcon::EmojiNeutral => '\u{f2ef}',
        IpgIcon::EmojiNeutralFill => '\u{f2ee}',
        IpgIcon::EmojiSmile => '\u{f2f3}',
        IpgIcon::EmojiSmileFill => '\u{f2f0}',
        IpgIcon::EmojiSmileUpsideDown => '\u{f2f2}',
        IpgIcon::EmojiSmileUpsideDownFill => '\u{f2f1}',
        IpgIcon::EmojiSunglasses => '\u{f2f5}',
        IpgIcon::EmojiSunglassesFill => '\u{f2f4}',
        IpgIcon::EmojiWink => '\u{f2f7}',
        IpgIcon::EmojiWinkFill => '\u{f2f6}',
        IpgIcon::Envelope => '\u{f2fb}',
        IpgIcon::EnvelopeFill => '\u{f2f8}',
        IpgIcon::EnvelopeOpen => '\u{f2fa}',
        IpgIcon::EnvelopeOpenFill => '\u{f2f9}',
        IpgIcon::Exclamation => '\u{f306}',
        IpgIcon::ExclamationCircle => '\u{f2fd}',
        IpgIcon::ExclamationCircleFill => '\u{f2fc}',
        IpgIcon::ExclamationDiamond => '\u{f2ff}',
        IpgIcon::ExclamationDiamondFill => '\u{f2fe}',
        IpgIcon::ExclamationOctagon => '\u{f301}',
        IpgIcon::ExclamationOctagonFill => '\u{f300}',
        IpgIcon::ExclamationSquare => '\u{f303}',
        IpgIcon::ExclamationSquareFill => '\u{f302}',
        IpgIcon::ExclamationTriangle => '\u{f305}',
        IpgIcon::ExclamationTriangleFill => '\u{f304}',
        IpgIcon::Exclude => '\u{f307}',
        IpgIcon::Eye => '\u{f30b}',
        IpgIcon::EyeFill => '\u{f308}',
        IpgIcon::EyeSlash => '\u{f30a}',
        IpgIcon::EyeSlashFill => '\u{f309}',
        IpgIcon::Eyeglasses => '\u{f30c}',
        IpgIcon::Facebook => '\u{f30d}',
        IpgIcon::File => '\u{f389}',
        IpgIcon::FileArrowDown => '\u{f30f}',
        IpgIcon::FileArrowDownFill => '\u{f30e}',
        IpgIcon::FileArrowUp => '\u{f311}',
        IpgIcon::FileArrowUpFill => '\u{f310}',
        IpgIcon::FileBarGraph => '\u{f313}',
        IpgIcon::FileBarGraphFill => '\u{f312}',
        IpgIcon::FileBinary => '\u{f315}',
        IpgIcon::FileBinaryFill => '\u{f314}',
        IpgIcon::FileBreak => '\u{f317}',
        IpgIcon::FileBreakFill => '\u{f316}',
        IpgIcon::FileCheck => '\u{f319}',
        IpgIcon::FileCheckFill => '\u{f318}',
        IpgIcon::FileCode => '\u{f31b}',
        IpgIcon::FileCodeFill => '\u{f31a}',
        IpgIcon::FileDiff => '\u{f31d}',
        IpgIcon::FileDiffFill => '\u{f31c}',
        IpgIcon::FileEarmark => '\u{f35b}',
        IpgIcon::FileEarmarkArrowDown => '\u{f31f}',
        IpgIcon::FileEarmarkArrowDownFill => '\u{f31e}',
        IpgIcon::FileEarmarkArrowUp => '\u{f321}',
        IpgIcon::FileEarmarkArrowUpFill => '\u{f320}',
        IpgIcon::FileEarmarkBarGraph => '\u{f323}',
        IpgIcon::FileEarmarkBarGraphFill => '\u{f322}',
        IpgIcon::FileEarmarkBinary => '\u{f325}',
        IpgIcon::FileEarmarkBinaryFill => '\u{f324}',
        IpgIcon::FileEarmarkBreak => '\u{f327}',
        IpgIcon::FileEarmarkBreakFill => '\u{f326}',
        IpgIcon::FileEarmarkCheck => '\u{f329}',
        IpgIcon::FileEarmarkCheckFill => '\u{f328}',
        IpgIcon::FileEarmarkCode => '\u{f32b}',
        IpgIcon::FileEarmarkCodeFill => '\u{f32a}',
        IpgIcon::FileEarmarkDiff => '\u{f32d}',
        IpgIcon::FileEarmarkDiffFill => '\u{f32c}',
        IpgIcon::FileEarmarkEasel => '\u{f32f}',
        IpgIcon::FileEarmarkEaselFill => '\u{f32e}',
        IpgIcon::FileEarmarkExcel => '\u{f331}',
        IpgIcon::FileEarmarkExcelFill => '\u{f330}',
        IpgIcon::FileEarmarkFill => '\u{f332}',
        IpgIcon::FileEarmarkFont => '\u{f334}',
        IpgIcon::FileEarmarkFontFill => '\u{f333}',
        IpgIcon::FileEarmarkImage => '\u{f336}',
        IpgIcon::FileEarmarkImageFill => '\u{f335}',
        IpgIcon::FileEarmarkLock => '\u{f338}',
        IpgIcon::FileEarmarkLockFill => '\u{f337}',
        IpgIcon::FileEarmarkLock2 => '\u{f33a}',
        IpgIcon::FileEarmarkLock2Fill => '\u{f339}',
        IpgIcon::FileEarmarkMedical => '\u{f33c}',
        IpgIcon::FileEarmarkMedicalFill => '\u{f33b}',
        IpgIcon::FileEarmarkMinus => '\u{f33e}',
        IpgIcon::FileEarmarkMinusFill => '\u{f33d}',
        IpgIcon::FileEarmarkMusic => '\u{f340}',
        IpgIcon::FileEarmarkMusicFill => '\u{f33f}',
        IpgIcon::FileEarmarkPerson => '\u{f342}',
        IpgIcon::FileEarmarkPersonFill => '\u{f341}',
        IpgIcon::FileEarmarkPlay => '\u{f344}',
        IpgIcon::FileEarmarkPlayFill => '\u{f343}',
        IpgIcon::FileEarmarkPlus => '\u{f346}',
        IpgIcon::FileEarmarkPlusFill => '\u{f345}',
        IpgIcon::FileEarmarkPost => '\u{f348}',
        IpgIcon::FileEarmarkPostFill => '\u{f347}',
        IpgIcon::FileEarmarkPpt => '\u{f34a}',
        IpgIcon::FileEarmarkPptFill => '\u{f349}',
        IpgIcon::FileEarmarkRichtext => '\u{f34c}',
        IpgIcon::FileEarmarkRichtextFill => '\u{f34b}',
        IpgIcon::FileEarmarkRuled => '\u{f34e}',
        IpgIcon::FileEarmarkRuledFill => '\u{f34d}',
        IpgIcon::FileEarmarkSlides => '\u{f350}',
        IpgIcon::FileEarmarkSlidesFill => '\u{f34f}',
        IpgIcon::FileEarmarkSpreadsheet => '\u{f352}',
        IpgIcon::FileEarmarkSpreadsheetFill => '\u{f351}',
        IpgIcon::FileEarmarkText => '\u{f354}',
        IpgIcon::FileEarmarkTextFill => '\u{f353}',
        IpgIcon::FileEarmarkWord => '\u{f356}',
        IpgIcon::FileEarmarkWordFill => '\u{f355}',
        IpgIcon::FileEarmarkX => '\u{f358}',
        IpgIcon::FileEarmarkXFill => '\u{f357}',
        IpgIcon::FileEarmarkZip => '\u{f35a}',
        IpgIcon::FileEarmarkZipFill => '\u{f359}',
        IpgIcon::FileEasel => '\u{f35d}',
        IpgIcon::FileEaselFill => '\u{f35c}',
        IpgIcon::FileExcel => '\u{f35f}',
        IpgIcon::FileExcelFill => '\u{f35e}',
        IpgIcon::FileFill => '\u{f360}',
        IpgIcon::FileFont => '\u{f362}',
        IpgIcon::FileFontFill => '\u{f361}',
        IpgIcon::FileImage => '\u{f364}',
        IpgIcon::FileImageFill => '\u{f363}',
        IpgIcon::FileLock => '\u{f366}',
        IpgIcon::FileLockFill => '\u{f365}',
        IpgIcon::FileLock2 => '\u{f368}',
        IpgIcon::FileLock2Fill => '\u{f367}',
        IpgIcon::FileMedical => '\u{f36a}',
        IpgIcon::FileMedicalFill => '\u{f369}',
        IpgIcon::FileMinus => '\u{f36c}',
        IpgIcon::FileMinusFill => '\u{f36b}',
        IpgIcon::FileMusic => '\u{f36e}',
        IpgIcon::FileMusicFill => '\u{f36d}',
        IpgIcon::FilePerson => '\u{f370}',
        IpgIcon::FilePersonFill => '\u{f36f}',
        IpgIcon::FilePlay => '\u{f372}',
        IpgIcon::FilePlayFill => '\u{f371}',
        IpgIcon::FilePlus => '\u{f374}',
        IpgIcon::FilePlusFill => '\u{f373}',
        IpgIcon::FilePost => '\u{f376}',
        IpgIcon::FilePostFill => '\u{f375}',
        IpgIcon::FilePpt => '\u{f378}',
        IpgIcon::FilePptFill => '\u{f377}',
        IpgIcon::FileRichtext => '\u{f37a}',
        IpgIcon::FileRichtextFill => '\u{f379}',
        IpgIcon::FileRuled => '\u{f37c}',
        IpgIcon::FileRuledFill => '\u{f37b}',
        IpgIcon::FileSlides => '\u{f37e}',
        IpgIcon::FileSlidesFill => '\u{f37d}',
        IpgIcon::FileSpreadsheet => '\u{f380}',
        IpgIcon::FileSpreadsheetFill => '\u{f37f}',
        IpgIcon::FileText => '\u{f382}',
        IpgIcon::FileTextFill => '\u{f381}',
        IpgIcon::FileWord => '\u{f384}',
        IpgIcon::FileWordFill => '\u{f383}',
        IpgIcon::FileX => '\u{f386}',
        IpgIcon::FileXFill => '\u{f385}',
        IpgIcon::FileZip => '\u{f388}',
        IpgIcon::FileZipFill => '\u{f387}',
        IpgIcon::Files => '\u{f38b}',
        IpgIcon::FilesAlt => '\u{f38a}',
        IpgIcon::Film => '\u{f38c}',
        IpgIcon::Filter => '\u{f393}',
        IpgIcon::FilterCircle => '\u{f38e}',
        IpgIcon::FilterCircleFill => '\u{f38d}',
        IpgIcon::FilterLeft => '\u{f38f}',
        IpgIcon::FilterRight => '\u{f390}',
        IpgIcon::FilterSquare => '\u{f392}',
        IpgIcon::FilterSquareFill => '\u{f391}',
        IpgIcon::Flag => '\u{f395}',
        IpgIcon::FlagFill => '\u{f394}',
        IpgIcon::Flower1 => '\u{f396}',
        IpgIcon::Flower2 => '\u{f397}',
        IpgIcon::Flower3 => '\u{f398}',
        IpgIcon::Folder => '\u{f3a0}',
        IpgIcon::FolderCheck => '\u{f399}',
        IpgIcon::FolderFill => '\u{f39a}',
        IpgIcon::FolderMinus => '\u{f39b}',
        IpgIcon::FolderPlus => '\u{f39c}',
        IpgIcon::FolderSymlink => '\u{f39e}',
        IpgIcon::FolderSymlinkFill => '\u{f39d}',
        IpgIcon::FolderX => '\u{f39f}',
        IpgIcon::Folder2 => '\u{f3a2}',
        IpgIcon::Folder2Open => '\u{f3a1}',
        IpgIcon::Fonts => '\u{f3a3}',
        IpgIcon::Forward => '\u{f3a5}',
        IpgIcon::ForwardFill => '\u{f3a4}',
        IpgIcon::Front => '\u{f3a6}',
        IpgIcon::Fullscreen => '\u{f3a8}',
        IpgIcon::FullscreenExit => '\u{f3a7}',
        IpgIcon::Funnel => '\u{f3aa}',
        IpgIcon::FunnelFill => '\u{f3a9}',
        IpgIcon::Gear => '\u{f3ae}',
        IpgIcon::GearFill => '\u{f3ab}',
        IpgIcon::GearWide => '\u{f3ad}',
        IpgIcon::GearWideConnected => '\u{f3ac}',
        IpgIcon::Gem => '\u{f3af}',
        IpgIcon::Geo => '\u{f3b3}',
        IpgIcon::GeoAlt => '\u{f3b1}',
        IpgIcon::GeoAltFill => '\u{f3b0}',
        IpgIcon::GeoFill => '\u{f3b2}',
        IpgIcon::Gift => '\u{f3b5}',
        IpgIcon::GiftFill => '\u{f3b4}',
        IpgIcon::Github => '\u{f3b6}',
        IpgIcon::Globe => '\u{f3b7}',
        IpgIcon::Globe2 => '\u{f3b8}',
        IpgIcon::Google => '\u{f3b9}',
        IpgIcon::GraphDown => '\u{f3ba}',
        IpgIcon::GraphUp => '\u{f3bb}',
        IpgIcon::Grid => '\u{f3c5}',
        IpgIcon::Grid1X2 => '\u{f3bd}',
        IpgIcon::Grid1X2Fill => '\u{f3bc}',
        IpgIcon::Grid3X2 => '\u{f3c0}',
        IpgIcon::Grid3X2Gap => '\u{f3bf}',
        IpgIcon::Grid3X2GapFill => '\u{f3be}',
        IpgIcon::Grid3X3 => '\u{f3c3}',
        IpgIcon::Grid3X3Gap => '\u{f3c2}',
        IpgIcon::Grid3X3GapFill => '\u{f3c1}',
        IpgIcon::GridFill => '\u{f3c4}',
        IpgIcon::GripHorizontal => '\u{f3c6}',
        IpgIcon::GripVertical => '\u{f3c7}',
        IpgIcon::Hammer => '\u{f3c8}',
        IpgIcon::HandIndex => '\u{f3ca}',
        IpgIcon::HandIndexThumb => '\u{f3c9}',
        IpgIcon::HandThumbsDown => '\u{f3cb}',
        IpgIcon::HandThumbsUp => '\u{f3cc}',
        IpgIcon::Handbag => '\u{f3ce}',
        IpgIcon::HandbagFill => '\u{f3cd}',
        IpgIcon::Hash => '\u{f3cf}',
        IpgIcon::Hdd => '\u{f3d7}',
        IpgIcon::HddFill => '\u{f3d0}',
        IpgIcon::HddNetwork => '\u{f3d2}',
        IpgIcon::HddNetworkFill => '\u{f3d1}',
        IpgIcon::HddRack => '\u{f3d4}',
        IpgIcon::HddRackFill => '\u{f3d3}',
        IpgIcon::HddStack => '\u{f3d6}',
        IpgIcon::HddStackFill => '\u{f3d5}',
        IpgIcon::Headphones => '\u{f3d8}',
        IpgIcon::Headset => '\u{f3d9}',
        IpgIcon::Heart => '\u{f3dc}',
        IpgIcon::HeartFill => '\u{f3da}',
        IpgIcon::HeartHalf => '\u{f3db}',
        IpgIcon::Heptagon => '\u{f3df}',
        IpgIcon::HeptagonFill => '\u{f3dd}',
        IpgIcon::HeptagonHalf => '\u{f3de}',
        IpgIcon::Hexagon => '\u{f3e2}',
        IpgIcon::HexagonFill => '\u{f3e0}',
        IpgIcon::HexagonHalf => '\u{f3e1}',
        IpgIcon::Hourglass => '\u{f3e6}',
        IpgIcon::HourglassBottom => '\u{f3e3}',
        IpgIcon::HourglassSplit => '\u{f3e4}',
        IpgIcon::HourglassTop => '\u{f3e5}',
        IpgIcon::House => '\u{f3ea}',
        IpgIcon::HouseDoor => '\u{f3e8}',
        IpgIcon::HouseDoorFill => '\u{f3e7}',
        IpgIcon::HouseFill => '\u{f3e9}',
        IpgIcon::Hr => '\u{f3eb}',
        IpgIcon::Image => '\u{f3ee}',
        IpgIcon::ImageAlt => '\u{f3ec}',
        IpgIcon::ImageFill => '\u{f3ed}',
        IpgIcon::Images => '\u{f3ef}',
        IpgIcon::Inbox => '\u{f3f1}',
        IpgIcon::InboxFill => '\u{f3f0}',
        IpgIcon::Inboxes => '\u{f3f3}',
        IpgIcon::InboxesFill => '\u{f3f2}',
        IpgIcon::Info => '\u{f3f8}',
        IpgIcon::InfoCircle => '\u{f3f5}',
        IpgIcon::InfoCircleFill => '\u{f3f4}',
        IpgIcon::InfoSquare => '\u{f3f7}',
        IpgIcon::InfoSquareFill => '\u{f3f6}',
        IpgIcon::InputCursor => '\u{f3fa}',
        IpgIcon::InputCursorText => '\u{f3f9}',
        IpgIcon::Instagram => '\u{f3fb}',
        IpgIcon::Intersect => '\u{f3fc}',
        IpgIcon::Journal => '\u{f40a}',
        IpgIcon::JournalAlbum => '\u{f3fd}',
        IpgIcon::JournalArrowDown => '\u{f3fe}',
        IpgIcon::JournalArrowUp => '\u{f3ff}',
        IpgIcon::JournalBookmark => '\u{f401}',
        IpgIcon::JournalBookmarkFill => '\u{f400}',
        IpgIcon::JournalCheck => '\u{f402}',
        IpgIcon::JournalCode => '\u{f403}',
        IpgIcon::JournalMedical => '\u{f404}',
        IpgIcon::JournalMinus => '\u{f405}',
        IpgIcon::JournalPlus => '\u{f406}',
        IpgIcon::JournalRichtext => '\u{f407}',
        IpgIcon::JournalText => '\u{f408}',
        IpgIcon::JournalX => '\u{f409}',
        IpgIcon::Journals => '\u{f40b}',
        IpgIcon::Joystick => '\u{f40c}',
        IpgIcon::Justify => '\u{f40f}',
        IpgIcon::JustifyLeft => '\u{f40d}',
        IpgIcon::JustifyRight => '\u{f40e}',
        IpgIcon::Kanban => '\u{f411}',
        IpgIcon::KanbanFill => '\u{f410}',
        IpgIcon::Key => '\u{f413}',
        IpgIcon::KeyFill => '\u{f412}',
        IpgIcon::Keyboard => '\u{f415}',
        IpgIcon::KeyboardFill => '\u{f414}',
        IpgIcon::Ladder => '\u{f416}',
        IpgIcon::Lamp => '\u{f418}',
        IpgIcon::LampFill => '\u{f417}',
        IpgIcon::Laptop => '\u{f41a}',
        IpgIcon::LaptopFill => '\u{f419}',
        IpgIcon::Layers => '\u{f41d}',
        IpgIcon::LayersFill => '\u{f41b}',
        IpgIcon::LayersHalf => '\u{f41c}',
        IpgIcon::LayoutSidebar => '\u{f421}',
        IpgIcon::LayoutSidebarInset => '\u{f41f}',
        IpgIcon::LayoutSidebarInsetReverse => '\u{f41e}',
        IpgIcon::LayoutSidebarReverse => '\u{f420}',
        IpgIcon::LayoutSplit => '\u{f422}',
        IpgIcon::LayoutTextSidebar => '\u{f424}',
        IpgIcon::LayoutTextSidebarReverse => '\u{f423}',
        IpgIcon::LayoutTextWindow => '\u{f426}',
        IpgIcon::LayoutTextWindowReverse => '\u{f425}',
        IpgIcon::LayoutThreeColumns => '\u{f427}',
        IpgIcon::LayoutWtf => '\u{f428}',
        IpgIcon::LifePreserver => '\u{f429}',
        IpgIcon::Lightning => '\u{f42b}',
        IpgIcon::LightningFill => '\u{f42a}',
        IpgIcon::Link => '\u{f42d}',
        IpgIcon::Link45Deg => '\u{f42c}',
        IpgIcon::Linkedin => '\u{f42e}',
        IpgIcon::List => '\u{f435}',
        IpgIcon::ListCheck => '\u{f42f}',
        IpgIcon::ListNested => '\u{f430}',
        IpgIcon::ListOl => '\u{f431}',
        IpgIcon::ListStars => '\u{f432}',
        IpgIcon::ListTask => '\u{f433}',
        IpgIcon::ListUl => '\u{f434}',
        IpgIcon::Lock => '\u{f437}',
        IpgIcon::LockFill => '\u{f436}',
        IpgIcon::Mailbox => '\u{f438}',
        IpgIcon::Mailbox2 => '\u{f439}',
        IpgIcon::Map => '\u{f43b}',
        IpgIcon::MapFill => '\u{f43a}',
        IpgIcon::Markdown => '\u{f43d}',
        IpgIcon::MarkdownFill => '\u{f43c}',
        IpgIcon::MenuApp => '\u{f43f}',
        IpgIcon::MenuAppFill => '\u{f43e}',
        IpgIcon::MenuButton => '\u{f443}',
        IpgIcon::MenuButtonFill => '\u{f440}',
        IpgIcon::MenuButtonWide => '\u{f442}',
        IpgIcon::MenuButtonWideFill => '\u{f441}',
        IpgIcon::MenuDown => '\u{f444}',
        IpgIcon::MenuUp => '\u{f445}',
        IpgIcon::Mic => '\u{f449}',
        IpgIcon::MicFill => '\u{f446}',
        IpgIcon::MicMute => '\u{f448}',
        IpgIcon::MicMuteFill => '\u{f447}',
        IpgIcon::Minecart => '\u{f44b}',
        IpgIcon::MinecartLoaded => '\u{f44a}',
        IpgIcon::Moon => '\u{f44c}',
        IpgIcon::Mouse => '\u{f44d}',
        IpgIcon::Mouse2 => '\u{f44e}',
        IpgIcon::Mouse3 => '\u{f44f}',
        IpgIcon::MusicNote => '\u{f452}',
        IpgIcon::MusicNoteBeamed => '\u{f450}',
        IpgIcon::MusicNoteList => '\u{f451}',
        IpgIcon::MusicPlayer => '\u{f454}',
        IpgIcon::MusicPlayerFill => '\u{f453}',
        IpgIcon::Newspaper => '\u{f455}',
        IpgIcon::NodeMinus => '\u{f457}',
        IpgIcon::NodeMinusFill => '\u{f456}',
        IpgIcon::NodePlus => '\u{f459}',
        IpgIcon::NodePlusFill => '\u{f458}',
        IpgIcon::Nut => '\u{f45b}',
        IpgIcon::NutFill => '\u{f45a}',
        IpgIcon::Octagon => '\u{f45e}',
        IpgIcon::OctagonFill => '\u{f45c}',
        IpgIcon::OctagonHalf => '\u{f45d}',
        IpgIcon::Option => '\u{f45f}',
        IpgIcon::Outlet => '\u{f460}',
        IpgIcon::Paperclip => '\u{f461}',
        IpgIcon::Paragraph => '\u{f462}',
        IpgIcon::PatchCheck => '\u{f464}',
        IpgIcon::PatchCheckFll => '\u{f463}',
        IpgIcon::PatchExclamation => '\u{f466}',
        IpgIcon::PatchExclamationFll => '\u{f465}',
        IpgIcon::PatchMinus => '\u{f468}',
        IpgIcon::PatchMinusFll => '\u{f467}',
        IpgIcon::PatchPlus => '\u{f46a}',
        IpgIcon::PatchPlusFll => '\u{f469}',
        IpgIcon::PatchQuestion => '\u{f46c}',
        IpgIcon::PatchQuestionFll => '\u{f46b}',
        IpgIcon::Pause => '\u{f472}',
        IpgIcon::PauseBtn => '\u{f46e}',
        IpgIcon::PauseBtnFill => '\u{f46d}',
        IpgIcon::PauseCircle => '\u{f470}',
        IpgIcon::PauseCircleFill => '\u{f46f}',
        IpgIcon::PauseFill => '\u{f471}',
        IpgIcon::Peace => '\u{f474}',
        IpgIcon::PeaceFill => '\u{f473}',
        IpgIcon::Pen => '\u{f476}',
        IpgIcon::PenFill => '\u{f475}',
        IpgIcon::Pencil => '\u{f479}',
        IpgIcon::PencilFill => '\u{f477}',
        IpgIcon::PencilSquare => '\u{f478}',
        IpgIcon::Pentagon => '\u{f47c}',
        IpgIcon::PentagonFill => '\u{f47a}',
        IpgIcon::PentagonHalf => '\u{f47b}',
        IpgIcon::People => '\u{f47e}',
        IpgIcon::PeopleFill => '\u{f47d}',
        IpgIcon::Percent => '\u{f47f}',
        IpgIcon::Person => '\u{f48f}',
        IpgIcon::PersonBadge => '\u{f481}',
        IpgIcon::PersonBadgeFill => '\u{f480}',
        IpgIcon::PersonBoundingBox => '\u{f482}',
        IpgIcon::PersonCheck => '\u{f484}',
        IpgIcon::PersonCheckFill => '\u{f483}',
        IpgIcon::PersonCircle => '\u{f485}',
        IpgIcon::PersonDash => '\u{f487}',
        IpgIcon::PersonDashFill => '\u{f486}',
        IpgIcon::PersonFill => '\u{f488}',
        IpgIcon::PersonLinesFill => '\u{f489}',
        IpgIcon::PersonPlus => '\u{f48b}',
        IpgIcon::PersonPlusFill => '\u{f48a}',
        IpgIcon::PersonSquare => '\u{f48c}',
        IpgIcon::PersonX => '\u{f48e}',
        IpgIcon::PersonXFill => '\u{f48d}',
        IpgIcon::Phone => '\u{f494}',
        IpgIcon::PhoneFill => '\u{f490}',
        IpgIcon::PhoneLandscape => '\u{f492}',
        IpgIcon::PhoneLandscapeFill => '\u{f491}',
        IpgIcon::PhoneVibrate => '\u{f493}',
        IpgIcon::PieChart => '\u{f496}',
        IpgIcon::PieChartFill => '\u{f495}',
        IpgIcon::Pip => '\u{f498}',
        IpgIcon::PipFill => '\u{f497}',
        IpgIcon::Play => '\u{f49e}',
        IpgIcon::PlayBtn => '\u{f49a}',
        IpgIcon::PlayBtnFill => '\u{f499}',
        IpgIcon::PlayCircle => '\u{f49c}',
        IpgIcon::PlayCircleFill => '\u{f49b}',
        IpgIcon::PlayFill => '\u{f49d}',
        IpgIcon::Plug => '\u{f4a0}',
        IpgIcon::PlugFill => '\u{f49f}',
        IpgIcon::Plus => '\u{f4a5}',
        IpgIcon::PlusCircle => '\u{f4a2}',
        IpgIcon::PlusCircleFill => '\u{f4a1}',
        IpgIcon::PlusSquare => '\u{f4a4}',
        IpgIcon::PlusSquareFill => '\u{f4a3}',
        IpgIcon::Power => '\u{f4a6}',
        IpgIcon::Printer => '\u{f4a8}',
        IpgIcon::PrinterFill => '\u{f4a7}',
        IpgIcon::Puzzle => '\u{f4aa}',
        IpgIcon::PuzzleFill => '\u{f4a9}',
        IpgIcon::Question => '\u{f4b3}',
        IpgIcon::QuestionCircle => '\u{f4ac}',
        IpgIcon::QuestionCircleFill => '\u{f4ab}',
        IpgIcon::QuestionDiamond => '\u{f4ae}',
        IpgIcon::QuestionDiamondFill => '\u{f4ad}',
        IpgIcon::QuestionOctagon => '\u{f4b0}',
        IpgIcon::QuestionOctagonFill => '\u{f4af}',
        IpgIcon::QuestionSquare => '\u{f4b2}',
        IpgIcon::QuestionSquareFill => '\u{f4b1}',
        IpgIcon::Receipt => '\u{f4b5}',
        IpgIcon::ReceiptCutoff => '\u{f4b4}',
        IpgIcon::Reception0 => '\u{f4b6}',
        IpgIcon::Reception1 => '\u{f4b7}',
        IpgIcon::Reception2 => '\u{f4b8}',
        IpgIcon::Reception3 => '\u{f4b9}',
        IpgIcon::Reception4 => '\u{f4ba}',
        IpgIcon::Record => '\u{f4c0}',
        IpgIcon::RecordBtn => '\u{f4bc}',
        IpgIcon::RecordBtnFill => '\u{f4bb}',
        IpgIcon::RecordCircle => '\u{f4be}',
        IpgIcon::RecordCircleFill => '\u{f4bd}',
        IpgIcon::RecordFill => '\u{f4bf}',
        IpgIcon::Record2 => '\u{f4c2}',
        IpgIcon::Record2Fill => '\u{f4c1}',
        IpgIcon::Reply => '\u{f4c6}',
        IpgIcon::ReplyAll => '\u{f4c4}',
        IpgIcon::ReplyAllFill => '\u{f4c3}',
        IpgIcon::ReplyFill => '\u{f4c5}',
        IpgIcon::Rss => '\u{f4c8}',
        IpgIcon::RssFill => '\u{f4c7}',
        IpgIcon::Scissors => '\u{f4c9}',
        IpgIcon::Screwdriver => '\u{f4ca}',
        IpgIcon::Search => '\u{f4cb}',
        IpgIcon::SegmentedNav => '\u{f4cc}',
        IpgIcon::Server => '\u{f4cd}',
        IpgIcon::Share => '\u{f4cf}',
        IpgIcon::ShareFill => '\u{f4ce}',
        IpgIcon::Shield => '\u{f4e0}',
        IpgIcon::ShieldCheck => '\u{f4d0}',
        IpgIcon::ShieldExclamation => '\u{f4d1}',
        IpgIcon::ShieldFill => '\u{f4d7}',
        IpgIcon::ShieldFillCheck => '\u{f4d2}',
        IpgIcon::ShieldFillExclamation => '\u{f4d3}',
        IpgIcon::ShieldFillMinus => '\u{f4d4}',
        IpgIcon::ShieldFillPlus => '\u{f4d5}',
        IpgIcon::ShieldFillX => '\u{f4d6}',
        IpgIcon::ShieldLock => '\u{f4d9}',
        IpgIcon::ShieldLockFill => '\u{f4d8}',
        IpgIcon::ShieldMinus => '\u{f4da}',
        IpgIcon::ShieldPlus => '\u{f4db}',
        IpgIcon::ShieldShaded => '\u{f4dc}',
        IpgIcon::ShieldSlash => '\u{f4de}',
        IpgIcon::ShieldSlashFill => '\u{f4dd}',
        IpgIcon::ShieldX => '\u{f4df}',
        IpgIcon::Shift => '\u{f4e2}',
        IpgIcon::ShiftFill => '\u{f4e1}',
        IpgIcon::Shop => '\u{f4e4}',
        IpgIcon::ShopWindow => '\u{f4e3}',
        IpgIcon::Shuffle => '\u{f4e5}',
        IpgIcon::Signpost => '\u{f4eb}',
        IpgIcon::Signpost2 => '\u{f4e7}',
        IpgIcon::Signpost2Fill => '\u{f4e6}',
        IpgIcon::SignpostFill => '\u{f4e8}',
        IpgIcon::SignpostSplit => '\u{f4ea}',
        IpgIcon::SignpostSplitFill => '\u{f4e9}',
        IpgIcon::Sim => '\u{f4ed}',
        IpgIcon::SimFill => '\u{f4ec}',
        IpgIcon::SkipBackward => '\u{f4f3}',
        IpgIcon::SkipBackwardBtn => '\u{f4ef}',
        IpgIcon::SkipBackwardBtnFill => '\u{f4ee}',
        IpgIcon::SkipBackwardCircle => '\u{f4f1}',
        IpgIcon::SkipBackwardCircleFill => '\u{f4f0}',
        IpgIcon::SkipBackwardFill => '\u{f4f2}',
        IpgIcon::SkipEnd => '\u{f4f9}',
        IpgIcon::SkipEndBtn => '\u{f4f5}',
        IpgIcon::SkipEndBtnFill => '\u{f4f4}',
        IpgIcon::SkipEndCircle => '\u{f4f7}',
        IpgIcon::SkipEndCircleFill => '\u{f4f6}',
        IpgIcon::SkipEndFill => '\u{f4f8}',
        IpgIcon::SkipForward => '\u{f4ff}',
        IpgIcon::SkipForwardBtn => '\u{f4fb}',
        IpgIcon::SkipForwardBtnFill => '\u{f4fa}',
        IpgIcon::SkipForwardCircle => '\u{f4fd}',
        IpgIcon::SkipForwardCircleFill => '\u{f4fc}',
        IpgIcon::SkipForwardFill => '\u{f4fe}',
        IpgIcon::SkipStart => '\u{f505}',
        IpgIcon::SkipStartBtn => '\u{f501}',
        IpgIcon::SkipStartBtnFill => '\u{f500}',
        IpgIcon::SkipStartCircle => '\u{f503}',
        IpgIcon::SkipStartCircleFill => '\u{f502}',
        IpgIcon::SkipStartFill => '\u{f504}',
        IpgIcon::Slack => '\u{f506}',
        IpgIcon::Slash => '\u{f50b}',
        IpgIcon::SlashCircle => '\u{f508}',
        IpgIcon::SlashCircleFill => '\u{f507}',
        IpgIcon::SlashSquare => '\u{f50a}',
        IpgIcon::SlashSquareFill => '\u{f509}',
        IpgIcon::Sliders => '\u{f50c}',
        IpgIcon::Smartwatch => '\u{f50d}',
        IpgIcon::SortAlphaDown => '\u{f50f}',
        IpgIcon::SortAlphaDownAlt => '\u{f50e}',
        IpgIcon::SortAlphaUp => '\u{f511}',
        IpgIcon::SortAlphaUpAlt => '\u{f510}',
        IpgIcon::SortDown => '\u{f513}',
        IpgIcon::SortDownAlt => '\u{f512}',
        IpgIcon::SortNumericDown => '\u{f515}',
        IpgIcon::SortNumericDownAlt => '\u{f514}',
        IpgIcon::SortNumericUp => '\u{f517}',
        IpgIcon::SortNumericUpAlt => '\u{f516}',
        IpgIcon::SortUp => '\u{f519}',
        IpgIcon::SortUpAlt => '\u{f518}',
        IpgIcon::Soundwave => '\u{f51a}',
        IpgIcon::Speaker => '\u{f51c}',
        IpgIcon::SpeakerFill => '\u{f51b}',
        IpgIcon::Spellcheck => '\u{f51d}',
        IpgIcon::Square => '\u{f520}',
        IpgIcon::SquareFill => '\u{f51e}',
        IpgIcon::SquareHalf => '\u{f51f}',
        IpgIcon::Star => '\u{f523}',
        IpgIcon::StarFill => '\u{f521}',
        IpgIcon::StarHalf => '\u{f522}',
        IpgIcon::Stickies => '\u{f525}',
        IpgIcon::StickiesFill => '\u{f524}',
        IpgIcon::Sticky => '\u{f527}',
        IpgIcon::StickyFill => '\u{f526}',
        IpgIcon::Stop => '\u{f52d}',
        IpgIcon::StopBtn => '\u{f529}',
        IpgIcon::StopBtnFill => '\u{f528}',
        IpgIcon::StopCircle => '\u{f52b}',
        IpgIcon::StopCircleFill => '\u{f52a}',
        IpgIcon::StopFill => '\u{f52c}',
        IpgIcon::Stoplights => '\u{f52f}',
        IpgIcon::StoplightsFill => '\u{f52e}',
        IpgIcon::Stopwatch => '\u{f531}',
        IpgIcon::StopwatchFill => '\u{f530}',
        IpgIcon::Subtract => '\u{f532}',
        IpgIcon::SuitClub => '\u{f534}',
        IpgIcon::SuitClubFill => '\u{f533}',
        IpgIcon::SuitDiamond => '\u{f536}',
        IpgIcon::SuitDiamondFill => '\u{f535}',
        IpgIcon::SuitHeart => '\u{f538}',
        IpgIcon::SuitHeartFill => '\u{f537}',
        IpgIcon::SuitSpade => '\u{f53a}',
        IpgIcon::SuitSpadeFill => '\u{f539}',
        IpgIcon::Sun => '\u{f53b}',
        IpgIcon::Sunglasses => '\u{f53c}',
        IpgIcon::Table => '\u{f53d}',
        IpgIcon::Tablet => '\u{f541}',
        IpgIcon::TabletFill => '\u{f53e}',
        IpgIcon::TabletLandscape => '\u{f540}',
        IpgIcon::TabletLandscapeFill => '\u{f53f}',
        IpgIcon::Tag => '\u{f543}',
        IpgIcon::TagFill => '\u{f542}',
        IpgIcon::Tags => '\u{f545}',
        IpgIcon::TagsFill => '\u{f544}',
        IpgIcon::Telephone => '\u{f553}',
        IpgIcon::TelephoneFill => '\u{f546}',
        IpgIcon::TelephoneForward => '\u{f548}',
        IpgIcon::TelephoneForwardFill => '\u{f547}',
        IpgIcon::TelephoneInbound => '\u{f54a}',
        IpgIcon::TelephoneInboundFill => '\u{f549}',
        IpgIcon::TelephoneMinus => '\u{f54c}',
        IpgIcon::TelephoneMinusFill => '\u{f54b}',
        IpgIcon::TelephoneOutbound => '\u{f54e}',
        IpgIcon::TelephoneOutboundFill => '\u{f54d}',
        IpgIcon::TelephonePlus => '\u{f550}',
        IpgIcon::TelephonePlusFill => '\u{f54f}',
        IpgIcon::TelephoneX => '\u{f552}',
        IpgIcon::TelephoneXFill => '\u{f551}',
        IpgIcon::Terminal => '\u{f555}',
        IpgIcon::TerminalFill => '\u{f554}',
        IpgIcon::TextCenter => '\u{f556}',
        IpgIcon::TextIndentLeft => '\u{f557}',
        IpgIcon::TextIndentRight => '\u{f558}',
        IpgIcon::TextLeft => '\u{f559}',
        IpgIcon::TextParagraph => '\u{f55a}',
        IpgIcon::TextRight => '\u{f55b}',
        IpgIcon::Textarea => '\u{f55e}',
        IpgIcon::TextareaResize => '\u{f55c}',
        IpgIcon::TextareaT => '\u{f55d}',
        IpgIcon::Thermometer => '\u{f560}',
        IpgIcon::ThermometerHalf => '\u{f55f}',
        IpgIcon::ThreeDots => '\u{f562}',
        IpgIcon::ThreeDotsVertical => '\u{f561}',
        IpgIcon::ToggleOff => '\u{f563}',
        IpgIcon::ToggleOn => '\u{f564}',
        IpgIcon::Toggle2Off => '\u{f565}',
        IpgIcon::Toggle2On => '\u{f566}',
        IpgIcon::Toggles => '\u{f567}',
        IpgIcon::Toggles2 => '\u{f568}',
        IpgIcon::Tools => '\u{f569}',
        IpgIcon::Trash => '\u{f56b}',
        IpgIcon::TrashFill => '\u{f56a}',
        IpgIcon::Trash2 => '\u{f56d}',
        IpgIcon::Trash2Fill => '\u{f56c}',
        IpgIcon::Tree => '\u{f56f}',
        IpgIcon::TreeFill => '\u{f56e}',
        IpgIcon::Triangle => '\u{f572}',
        IpgIcon::TriangleFill => '\u{f570}',
        IpgIcon::TriangleHalf => '\u{f571}',
        IpgIcon::Trophy => '\u{f574}',
        IpgIcon::TrophyFill => '\u{f573}',
        IpgIcon::Truck => '\u{f576}',
        IpgIcon::TruckFlatbed => '\u{f575}',
        IpgIcon::Tv => '\u{f578}',
        IpgIcon::TvFill => '\u{f577}',
        IpgIcon::Twitch => '\u{f579}',
        IpgIcon::Twitter => '\u{f57a}',
        IpgIcon::Type => '\u{f582}',
        IpgIcon::TypeBold => '\u{f57b}',
        IpgIcon::TypeH1 => '\u{f57c}',
        IpgIcon::TypeH2 => '\u{f57d}',
        IpgIcon::TypeH3 => '\u{f57e}',
        IpgIcon::TypeItalic => '\u{f57f}',
        IpgIcon::TypeStrikethrough => '\u{f580}',
        IpgIcon::TypeUnderline => '\u{f581}',
        IpgIcon::UiChecks => '\u{f584}',
        IpgIcon::UiChecksGrid => '\u{f583}',
        IpgIcon::UiRadios => '\u{f586}',
        IpgIcon::UiRadiosGrid => '\u{f585}',
        IpgIcon::Union => '\u{f587}',
        IpgIcon::Unlock => '\u{f589}',
        IpgIcon::UnlockFill => '\u{f588}',
        IpgIcon::Upc => '\u{f58b}',
        IpgIcon::UpcScan => '\u{f58a}',
        IpgIcon::Upload => '\u{f58c}',
        IpgIcon::VectorPen => '\u{f58d}',
        IpgIcon::ViewList => '\u{f58e}',
        IpgIcon::ViewStacked => '\u{f58f}',
        IpgIcon::Vinyl => '\u{f591}',
        IpgIcon::VinylFill => '\u{f590}',
        IpgIcon::Voicemail => '\u{f592}',
        IpgIcon::VolumeDown => '\u{f594}',
        IpgIcon::VolumeDownFill => '\u{f593}',
        IpgIcon::VolumeMute => '\u{f596}',
        IpgIcon::VolumeMuteFill => '\u{f595}',
        IpgIcon::VolumeOff => '\u{f598}',
        IpgIcon::VolumeOffFill => '\u{f597}',
        IpgIcon::VolumeUp => '\u{f59a}',
        IpgIcon::VolumeUpFill => '\u{f599}',
        IpgIcon::Vr => '\u{f59b}',
        IpgIcon::Wallet => '\u{f59d}',
        IpgIcon::WalletFill => '\u{f59c}',
        IpgIcon::Wallet2 => '\u{f59e}',
        IpgIcon::Watch => '\u{f59f}',
        IpgIcon::Wifi => '\u{f5a3}',
        IpgIcon::Wifi1 => '\u{f5a0}',
        IpgIcon::Wifi2 => '\u{f5a1}',
        IpgIcon::WifiOff => '\u{f5a2}',
        IpgIcon::Window => '\u{f5a4}',
        IpgIcon::Wrench => '\u{f5a5}',
        IpgIcon::X => '\u{f5ae}',
        IpgIcon::XCircle => '\u{f5a7}',
        IpgIcon::XCircleFill => '\u{f5a6}',
        IpgIcon::XDiamond => '\u{f5a9}',
        IpgIcon::XDiamondFill => '\u{f5a8}',
        IpgIcon::XOctagon => '\u{f5ab}',
        IpgIcon::XOctagonFill => '\u{f5aa}',
        IpgIcon::XSquare => '\u{f5ad}',
        IpgIcon::XSquareFill => '\u{f5ac}',
        IpgIcon::Youtube => '\u{f5af}',
        IpgIcon::ZoomIn => '\u{f5b0}',
        IpgIcon::ZoomOut => '\u{f5b1}',
    }
}
