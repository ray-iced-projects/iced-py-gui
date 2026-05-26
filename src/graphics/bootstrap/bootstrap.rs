//! Bootstrap

#![allow(clippy::doc_markdown, clippy::enum_variant_names, dead_code)]


use std::{
    fmt::{Display, Formatter, Result},
    string::String,
};

use iced::widget::{text, Text};

use crate::graphics::BOOTSTRAP_FONT;

/// Holds all glyphs of the Bootstrap font
#[derive(Debug, Clone, Copy)]
pub enum Bootstrap {
    /// Activity 
    Activity,
    /// Airplane 
    Airplane,
    /// AirplaneEngines 
    AirplaneEngines,
    /// AirplaneEnginesFill 
    AirplaneEnginesFill,
    /// AirplaneFill 
    AirplaneFill,
    /// Alarm 
    Alarm,
    /// AlarmFill 
    AlarmFill,
    /// Alexa 
    Alexa,
    /// AlignBottom 
    AlignBottom,
    /// AlignCenter 
    AlignCenter,
    /// AlignEnd 
    AlignEnd,
    /// AlignMiddle 
    AlignMiddle,
    /// AlignStart 
    AlignStart,
    /// AlignTop 
    AlignTop,
    /// Alipay 
    Alipay,
    /// Alphabet 
    Alphabet,
    /// AlphabetUppercase 
    AlphabetUppercase,
    /// Alt 
    Alt,
    /// Amazon 
    Amazon,
    /// Amd 
    Amd,
    /// Android 
    Android,
    /// Androidtwo 
    Androidtwo,
    /// App 
    App,
    /// AppIndicator 
    AppIndicator,
    /// Apple 
    Apple,
    /// Archive 
    Archive,
    /// ArchiveFill 
    ArchiveFill,
    /// ArrowBarDown 
    ArrowBarDown,
    /// ArrowBarLeft 
    ArrowBarLeft,
    /// ArrowBarRight 
    ArrowBarRight,
    /// ArrowBarUp 
    ArrowBarUp,
    /// ArrowClockwise 
    ArrowClockwise,
    /// ArrowCounterclockwise 
    ArrowCounterclockwise,
    /// ArrowDown 
    ArrowDown,
    /// ArrowDownCircle 
    ArrowDownCircle,
    /// ArrowDownCircleFill 
    ArrowDownCircleFill,
    /// ArrowDownLeft 
    ArrowDownLeft,
    /// ArrowDownLeftCircle 
    ArrowDownLeftCircle,
    /// ArrowDownLeftCircleFill 
    ArrowDownLeftCircleFill,
    /// ArrowDownLeftSquare 
    ArrowDownLeftSquare,
    /// ArrowDownLeftSquareFill 
    ArrowDownLeftSquareFill,
    /// ArrowDownRight 
    ArrowDownRight,
    /// ArrowDownRightCircle 
    ArrowDownRightCircle,
    /// ArrowDownRightCircleFill 
    ArrowDownRightCircleFill,
    /// ArrowDownRightSquare 
    ArrowDownRightSquare,
    /// ArrowDownRightSquareFill 
    ArrowDownRightSquareFill,
    /// ArrowDownShort 
    ArrowDownShort,
    /// ArrowDownSquare 
    ArrowDownSquare,
    /// ArrowDownSquareFill 
    ArrowDownSquareFill,
    /// ArrowDownUp 
    ArrowDownUp,
    /// ArrowLeft 
    ArrowLeft,
    /// ArrowLeftCircle 
    ArrowLeftCircle,
    /// ArrowLeftCircleFill 
    ArrowLeftCircleFill,
    /// ArrowLeftRight 
    ArrowLeftRight,
    /// ArrowLeftShort 
    ArrowLeftShort,
    /// ArrowLeftSquare 
    ArrowLeftSquare,
    /// ArrowLeftSquareFill 
    ArrowLeftSquareFill,
    /// ArrowNinezerodegDown 
    ArrowNinezerodegDown,
    /// ArrowNinezerodegLeft 
    ArrowNinezerodegLeft,
    /// ArrowNinezerodegRight 
    ArrowNinezerodegRight,
    /// ArrowNinezerodegUp 
    ArrowNinezerodegUp,
    /// ArrowRepeat 
    ArrowRepeat,
    /// ArrowReturnLeft 
    ArrowReturnLeft,
    /// ArrowReturnRight 
    ArrowReturnRight,
    /// ArrowRight 
    ArrowRight,
    /// ArrowRightCircle 
    ArrowRightCircle,
    /// ArrowRightCircleFill 
    ArrowRightCircleFill,
    /// ArrowRightShort 
    ArrowRightShort,
    /// ArrowRightSquare 
    ArrowRightSquare,
    /// ArrowRightSquareFill 
    ArrowRightSquareFill,
    /// ArrowThroughHeart 
    ArrowThroughHeart,
    /// ArrowThroughHeartFill 
    ArrowThroughHeartFill,
    /// ArrowUp 
    ArrowUp,
    /// ArrowUpCircle 
    ArrowUpCircle,
    /// ArrowUpCircleFill 
    ArrowUpCircleFill,
    /// ArrowUpLeft 
    ArrowUpLeft,
    /// ArrowUpLeftCircle 
    ArrowUpLeftCircle,
    /// ArrowUpLeftCircleFill 
    ArrowUpLeftCircleFill,
    /// ArrowUpLeftSquare 
    ArrowUpLeftSquare,
    /// ArrowUpLeftSquareFill 
    ArrowUpLeftSquareFill,
    /// ArrowUpRight 
    ArrowUpRight,
    /// ArrowUpRightCircle 
    ArrowUpRightCircle,
    /// ArrowUpRightCircleFill 
    ArrowUpRightCircleFill,
    /// ArrowUpRightSquare 
    ArrowUpRightSquare,
    /// ArrowUpRightSquareFill 
    ArrowUpRightSquareFill,
    /// ArrowUpShort 
    ArrowUpShort,
    /// ArrowUpSquare 
    ArrowUpSquare,
    /// ArrowUpSquareFill 
    ArrowUpSquareFill,
    /// Arrows 
    Arrows,
    /// ArrowsAngleContract 
    ArrowsAngleContract,
    /// ArrowsAngleExpand 
    ArrowsAngleExpand,
    /// ArrowsCollapse 
    ArrowsCollapse,
    /// ArrowsCollapseVertical 
    ArrowsCollapseVertical,
    /// ArrowsExpand 
    ArrowsExpand,
    /// ArrowsExpandVertical 
    ArrowsExpandVertical,
    /// ArrowsFullscreen 
    ArrowsFullscreen,
    /// ArrowsMove 
    ArrowsMove,
    /// ArrowsVertical 
    ArrowsVertical,
    /// AspectRatio 
    AspectRatio,
    /// AspectRatioFill 
    AspectRatioFill,
    /// Asterisk 
    Asterisk,
    /// At 
    At,
    /// Award 
    Award,
    /// AwardFill 
    AwardFill,
    /// Back 
    Back,
    /// Backpack 
    Backpack,
    /// BackpackFill 
    BackpackFill,
    /// Backpackfour 
    Backpackfour,
    /// BackpackfourFill 
    BackpackfourFill,
    /// Backpackthree 
    Backpackthree,
    /// BackpackthreeFill 
    BackpackthreeFill,
    /// Backpacktwo 
    Backpacktwo,
    /// BackpacktwoFill 
    BackpacktwoFill,
    /// Backspace 
    Backspace,
    /// BackspaceFill 
    BackspaceFill,
    /// BackspaceReverse 
    BackspaceReverse,
    /// BackspaceReverseFill 
    BackspaceReverseFill,
    /// BadgeAd 
    BadgeAd,
    /// BadgeAdFill 
    BadgeAdFill,
    /// BadgeAr 
    BadgeAr,
    /// BadgeArFill 
    BadgeArFill,
    /// BadgeCc 
    BadgeCc,
    /// BadgeCcFill 
    BadgeCcFill,
    /// BadgeEightk 
    BadgeEightk,
    /// BadgeEightkFill 
    BadgeEightkFill,
    /// BadgeFourk 
    BadgeFourk,
    /// BadgeFourkFill 
    BadgeFourkFill,
    /// BadgeHd 
    BadgeHd,
    /// BadgeHdFill 
    BadgeHdFill,
    /// BadgeSd 
    BadgeSd,
    /// BadgeSdFill 
    BadgeSdFill,
    /// BadgeThreed 
    BadgeThreed,
    /// BadgeThreedFill 
    BadgeThreedFill,
    /// BadgeTm 
    BadgeTm,
    /// BadgeTmFill 
    BadgeTmFill,
    /// BadgeVo 
    BadgeVo,
    /// BadgeVoFill 
    BadgeVoFill,
    /// BadgeVr 
    BadgeVr,
    /// BadgeVrFill 
    BadgeVrFill,
    /// BadgeWc 
    BadgeWc,
    /// BadgeWcFill 
    BadgeWcFill,
    /// Bag 
    Bag,
    /// BagCheck 
    BagCheck,
    /// BagCheckFill 
    BagCheckFill,
    /// BagDash 
    BagDash,
    /// BagDashFill 
    BagDashFill,
    /// BagFill 
    BagFill,
    /// BagHeart 
    BagHeart,
    /// BagHeartFill 
    BagHeartFill,
    /// BagPlus 
    BagPlus,
    /// BagPlusFill 
    BagPlusFill,
    /// BagX 
    BagX,
    /// BagXFill 
    BagXFill,
    /// Balloon 
    Balloon,
    /// BalloonFill 
    BalloonFill,
    /// BalloonHeart 
    BalloonHeart,
    /// BalloonHeartFill 
    BalloonHeartFill,
    /// Ban 
    Ban,
    /// BanFill 
    BanFill,
    /// Bandaid 
    Bandaid,
    /// BandaidFill 
    BandaidFill,
    /// Bank 
    Bank,
    /// Banktwo 
    Banktwo,
    /// BarChart 
    BarChart,
    /// BarChartFill 
    BarChartFill,
    /// BarChartLine 
    BarChartLine,
    /// BarChartLineFill 
    BarChartLineFill,
    /// BarChartSteps 
    BarChartSteps,
    /// Basket 
    Basket,
    /// BasketFill 
    BasketFill,
    /// Basketthree 
    Basketthree,
    /// BasketthreeFill 
    BasketthreeFill,
    /// Baskettwo 
    Baskettwo,
    /// BaskettwoFill 
    BaskettwoFill,
    /// Battery 
    Battery,
    /// BatteryCharging 
    BatteryCharging,
    /// BatteryFull 
    BatteryFull,
    /// BatteryHalf 
    BatteryHalf,
    /// Behance 
    Behance,
    /// Bell 
    Bell,
    /// BellFill 
    BellFill,
    /// BellSlash 
    BellSlash,
    /// BellSlashFill 
    BellSlashFill,
    /// Bezier 
    Bezier,
    /// Beziertwo 
    Beziertwo,
    /// Bicycle 
    Bicycle,
    /// Bing 
    Bing,
    /// Binoculars 
    Binoculars,
    /// BinocularsFill 
    BinocularsFill,
    /// BlockquoteLeft 
    BlockquoteLeft,
    /// BlockquoteRight 
    BlockquoteRight,
    /// Bluetooth 
    Bluetooth,
    /// BodyText 
    BodyText,
    /// Book 
    Book,
    /// BookFill 
    BookFill,
    /// BookHalf 
    BookHalf,
    /// Bookmark 
    Bookmark,
    /// BookmarkCheck 
    BookmarkCheck,
    /// BookmarkCheckFill 
    BookmarkCheckFill,
    /// BookmarkDash 
    BookmarkDash,
    /// BookmarkDashFill 
    BookmarkDashFill,
    /// BookmarkFill 
    BookmarkFill,
    /// BookmarkHeart 
    BookmarkHeart,
    /// BookmarkHeartFill 
    BookmarkHeartFill,
    /// BookmarkPlus 
    BookmarkPlus,
    /// BookmarkPlusFill 
    BookmarkPlusFill,
    /// BookmarkStar 
    BookmarkStar,
    /// BookmarkStarFill 
    BookmarkStarFill,
    /// BookmarkX 
    BookmarkX,
    /// BookmarkXFill 
    BookmarkXFill,
    /// Bookmarks 
    Bookmarks,
    /// BookmarksFill 
    BookmarksFill,
    /// Bookshelf 
    Bookshelf,
    /// Boombox 
    Boombox,
    /// BoomboxFill 
    BoomboxFill,
    /// Bootstrap 
    Bootstrap,
    /// BootstrapFill 
    BootstrapFill,
    /// BootstrapReboot 
    BootstrapReboot,
    /// Border 
    Border,
    /// BorderAll 
    BorderAll,
    /// BorderBottom 
    BorderBottom,
    /// BorderCenter 
    BorderCenter,
    /// BorderInner 
    BorderInner,
    /// BorderLeft 
    BorderLeft,
    /// BorderMiddle 
    BorderMiddle,
    /// BorderOuter 
    BorderOuter,
    /// BorderRight 
    BorderRight,
    /// BorderStyle 
    BorderStyle,
    /// BorderTop 
    BorderTop,
    /// BorderWidth 
    BorderWidth,
    /// BoundingBox 
    BoundingBox,
    /// BoundingBoxCircles 
    BoundingBoxCircles,
    /// Box 
    Box,
    /// BoxArrowDown 
    BoxArrowDown,
    /// BoxArrowDownLeft 
    BoxArrowDownLeft,
    /// BoxArrowDownRight 
    BoxArrowDownRight,
    /// BoxArrowInDown 
    BoxArrowInDown,
    /// BoxArrowInDownLeft 
    BoxArrowInDownLeft,
    /// BoxArrowInDownRight 
    BoxArrowInDownRight,
    /// BoxArrowInLeft 
    BoxArrowInLeft,
    /// BoxArrowInRight 
    BoxArrowInRight,
    /// BoxArrowInUp 
    BoxArrowInUp,
    /// BoxArrowInUpLeft 
    BoxArrowInUpLeft,
    /// BoxArrowInUpRight 
    BoxArrowInUpRight,
    /// BoxArrowLeft 
    BoxArrowLeft,
    /// BoxArrowRight 
    BoxArrowRight,
    /// BoxArrowUp 
    BoxArrowUp,
    /// BoxArrowUpLeft 
    BoxArrowUpLeft,
    /// BoxArrowUpRight 
    BoxArrowUpRight,
    /// BoxFill 
    BoxFill,
    /// BoxSeam 
    BoxSeam,
    /// BoxSeamFill 
    BoxSeamFill,
    /// Boxes 
    Boxes,
    /// Boxtwo 
    Boxtwo,
    /// BoxtwoFill 
    BoxtwoFill,
    /// BoxtwoHeart 
    BoxtwoHeart,
    /// BoxtwoHeartFill 
    BoxtwoHeartFill,
    /// Braces 
    Braces,
    /// BracesAsterisk 
    BracesAsterisk,
    /// Bricks 
    Bricks,
    /// Briefcase 
    Briefcase,
    /// BriefcaseFill 
    BriefcaseFill,
    /// BrightnessAltHigh 
    BrightnessAltHigh,
    /// BrightnessAltHighFill 
    BrightnessAltHighFill,
    /// BrightnessAltLow 
    BrightnessAltLow,
    /// BrightnessAltLowFill 
    BrightnessAltLowFill,
    /// BrightnessHigh 
    BrightnessHigh,
    /// BrightnessHighFill 
    BrightnessHighFill,
    /// BrightnessLow 
    BrightnessLow,
    /// BrightnessLowFill 
    BrightnessLowFill,
    /// Brilliance 
    Brilliance,
    /// Broadcast 
    Broadcast,
    /// BroadcastPin 
    BroadcastPin,
    /// BrowserChrome 
    BrowserChrome,
    /// BrowserEdge 
    BrowserEdge,
    /// BrowserFirefox 
    BrowserFirefox,
    /// BrowserSafari 
    BrowserSafari,
    /// Brush 
    Brush,
    /// BrushFill 
    BrushFill,
    /// Bucket 
    Bucket,
    /// BucketFill 
    BucketFill,
    /// Bug 
    Bug,
    /// BugFill 
    BugFill,
    /// Building 
    Building,
    /// BuildingAdd 
    BuildingAdd,
    /// BuildingCheck 
    BuildingCheck,
    /// BuildingDash 
    BuildingDash,
    /// BuildingDown 
    BuildingDown,
    /// BuildingExclamation 
    BuildingExclamation,
    /// BuildingFill 
    BuildingFill,
    /// BuildingFillAdd 
    BuildingFillAdd,
    /// BuildingFillCheck 
    BuildingFillCheck,
    /// BuildingFillDash 
    BuildingFillDash,
    /// BuildingFillDown 
    BuildingFillDown,
    /// BuildingFillExclamation 
    BuildingFillExclamation,
    /// BuildingFillGear 
    BuildingFillGear,
    /// BuildingFillLock 
    BuildingFillLock,
    /// BuildingFillSlash 
    BuildingFillSlash,
    /// BuildingFillUp 
    BuildingFillUp,
    /// BuildingFillX 
    BuildingFillX,
    /// BuildingGear 
    BuildingGear,
    /// BuildingLock 
    BuildingLock,
    /// BuildingSlash 
    BuildingSlash,
    /// BuildingUp 
    BuildingUp,
    /// BuildingX 
    BuildingX,
    /// Buildings 
    Buildings,
    /// BuildingsFill 
    BuildingsFill,
    /// Bullseye 
    Bullseye,
    /// BusFront 
    BusFront,
    /// BusFrontFill 
    BusFrontFill,
    /// CCircle 
    CCircle,
    /// CCircleFill 
    CCircleFill,
    /// CSquare 
    CSquare,
    /// CSquareFill 
    CSquareFill,
    /// Cake 
    Cake,
    /// CakeFill 
    CakeFill,
    /// Caketwo 
    Caketwo,
    /// CaketwoFill 
    CaketwoFill,
    /// Calculator 
    Calculator,
    /// CalculatorFill 
    CalculatorFill,
    /// Calendar 
    Calendar,
    /// CalendarCheck 
    CalendarCheck,
    /// CalendarCheckFill 
    CalendarCheckFill,
    /// CalendarDate 
    CalendarDate,
    /// CalendarDateFill 
    CalendarDateFill,
    /// CalendarDay 
    CalendarDay,
    /// CalendarDayFill 
    CalendarDayFill,
    /// CalendarEvent 
    CalendarEvent,
    /// CalendarEventFill 
    CalendarEventFill,
    /// CalendarFill 
    CalendarFill,
    /// CalendarHeart 
    CalendarHeart,
    /// CalendarHeartFill 
    CalendarHeartFill,
    /// CalendarMinus 
    CalendarMinus,
    /// CalendarMinusFill 
    CalendarMinusFill,
    /// CalendarMonth 
    CalendarMonth,
    /// CalendarMonthFill 
    CalendarMonthFill,
    /// CalendarPlus 
    CalendarPlus,
    /// CalendarPlusFill 
    CalendarPlusFill,
    /// CalendarRange 
    CalendarRange,
    /// CalendarRangeFill 
    CalendarRangeFill,
    /// CalendarWeek 
    CalendarWeek,
    /// CalendarWeekFill 
    CalendarWeekFill,
    /// CalendarX 
    CalendarX,
    /// CalendarXFill 
    CalendarXFill,
    /// Calendarfour 
    Calendarfour,
    /// CalendarfourEvent 
    CalendarfourEvent,
    /// CalendarfourRange 
    CalendarfourRange,
    /// CalendarfourWeek 
    CalendarfourWeek,
    /// Calendarthree 
    Calendarthree,
    /// CalendarthreeEvent 
    CalendarthreeEvent,
    /// CalendarthreeEventFill 
    CalendarthreeEventFill,
    /// CalendarthreeFill 
    CalendarthreeFill,
    /// CalendarthreeRange 
    CalendarthreeRange,
    /// CalendarthreeRangeFill 
    CalendarthreeRangeFill,
    /// CalendarthreeWeek 
    CalendarthreeWeek,
    /// CalendarthreeWeekFill 
    CalendarthreeWeekFill,
    /// Calendartwo 
    Calendartwo,
    /// CalendartwoCheck 
    CalendartwoCheck,
    /// CalendartwoCheckFill 
    CalendartwoCheckFill,
    /// CalendartwoDate 
    CalendartwoDate,
    /// CalendartwoDateFill 
    CalendartwoDateFill,
    /// CalendartwoDay 
    CalendartwoDay,
    /// CalendartwoDayFill 
    CalendartwoDayFill,
    /// CalendartwoEvent 
    CalendartwoEvent,
    /// CalendartwoEventFill 
    CalendartwoEventFill,
    /// CalendartwoFill 
    CalendartwoFill,
    /// CalendartwoHeart 
    CalendartwoHeart,
    /// CalendartwoHeartFill 
    CalendartwoHeartFill,
    /// CalendartwoMinus 
    CalendartwoMinus,
    /// CalendartwoMinusFill 
    CalendartwoMinusFill,
    /// CalendartwoMonth 
    CalendartwoMonth,
    /// CalendartwoMonthFill 
    CalendartwoMonthFill,
    /// CalendartwoPlus 
    CalendartwoPlus,
    /// CalendartwoPlusFill 
    CalendartwoPlusFill,
    /// CalendartwoRange 
    CalendartwoRange,
    /// CalendartwoRangeFill 
    CalendartwoRangeFill,
    /// CalendartwoWeek 
    CalendartwoWeek,
    /// CalendartwoWeekFill 
    CalendartwoWeekFill,
    /// CalendartwoX 
    CalendartwoX,
    /// CalendartwoXFill 
    CalendartwoXFill,
    /// Camera 
    Camera,
    /// CameraFill 
    CameraFill,
    /// CameraReels 
    CameraReels,
    /// CameraReelsFill 
    CameraReelsFill,
    /// CameraVideo 
    CameraVideo,
    /// CameraVideoFill 
    CameraVideoFill,
    /// CameraVideoOff 
    CameraVideoOff,
    /// CameraVideoOffFill 
    CameraVideoOffFill,
    /// Cameratwo 
    Cameratwo,
    /// Capslock 
    Capslock,
    /// CapslockFill 
    CapslockFill,
    /// Capsule 
    Capsule,
    /// CapsulePill 
    CapsulePill,
    /// CarFront 
    CarFront,
    /// CarFrontFill 
    CarFrontFill,
    /// CardChecklist 
    CardChecklist,
    /// CardHeading 
    CardHeading,
    /// CardImage 
    CardImage,
    /// CardList 
    CardList,
    /// CardText 
    CardText,
    /// CaretDown 
    CaretDown,
    /// CaretDownFill 
    CaretDownFill,
    /// CaretDownSquare 
    CaretDownSquare,
    /// CaretDownSquareFill 
    CaretDownSquareFill,
    /// CaretLeft 
    CaretLeft,
    /// CaretLeftFill 
    CaretLeftFill,
    /// CaretLeftSquare 
    CaretLeftSquare,
    /// CaretLeftSquareFill 
    CaretLeftSquareFill,
    /// CaretRight 
    CaretRight,
    /// CaretRightFill 
    CaretRightFill,
    /// CaretRightSquare 
    CaretRightSquare,
    /// CaretRightSquareFill 
    CaretRightSquareFill,
    /// CaretUp 
    CaretUp,
    /// CaretUpFill 
    CaretUpFill,
    /// CaretUpSquare 
    CaretUpSquare,
    /// CaretUpSquareFill 
    CaretUpSquareFill,
    /// Cart 
    Cart,
    /// CartCheck 
    CartCheck,
    /// CartCheckFill 
    CartCheckFill,
    /// CartDash 
    CartDash,
    /// CartDashFill 
    CartDashFill,
    /// CartFill 
    CartFill,
    /// CartPlus 
    CartPlus,
    /// CartPlusFill 
    CartPlusFill,
    /// CartX 
    CartX,
    /// CartXFill 
    CartXFill,
    /// Cartfour 
    Cartfour,
    /// Cartthree 
    Cartthree,
    /// Carttwo 
    Carttwo,
    /// Cash 
    Cash,
    /// CashCoin 
    CashCoin,
    /// CashStack 
    CashStack,
    /// Cassette 
    Cassette,
    /// CassetteFill 
    CassetteFill,
    /// Cast 
    Cast,
    /// CcCircle 
    CcCircle,
    /// CcCircleFill 
    CcCircleFill,
    /// CcSquare 
    CcSquare,
    /// CcSquareFill 
    CcSquareFill,
    /// Chat 
    Chat,
    /// ChatDots 
    ChatDots,
    /// ChatDotsFill 
    ChatDotsFill,
    /// ChatFill 
    ChatFill,
    /// ChatHeart 
    ChatHeart,
    /// ChatHeartFill 
    ChatHeartFill,
    /// ChatLeft 
    ChatLeft,
    /// ChatLeftDots 
    ChatLeftDots,
    /// ChatLeftDotsFill 
    ChatLeftDotsFill,
    /// ChatLeftFill 
    ChatLeftFill,
    /// ChatLeftHeart 
    ChatLeftHeart,
    /// ChatLeftHeartFill 
    ChatLeftHeartFill,
    /// ChatLeftQuote 
    ChatLeftQuote,
    /// ChatLeftQuoteFill 
    ChatLeftQuoteFill,
    /// ChatLeftText 
    ChatLeftText,
    /// ChatLeftTextFill 
    ChatLeftTextFill,
    /// ChatQuote 
    ChatQuote,
    /// ChatQuoteFill 
    ChatQuoteFill,
    /// ChatRight 
    ChatRight,
    /// ChatRightDots 
    ChatRightDots,
    /// ChatRightDotsFill 
    ChatRightDotsFill,
    /// ChatRightFill 
    ChatRightFill,
    /// ChatRightHeart 
    ChatRightHeart,
    /// ChatRightHeartFill 
    ChatRightHeartFill,
    /// ChatRightQuote 
    ChatRightQuote,
    /// ChatRightQuoteFill 
    ChatRightQuoteFill,
    /// ChatRightText 
    ChatRightText,
    /// ChatRightTextFill 
    ChatRightTextFill,
    /// ChatSquare 
    ChatSquare,
    /// ChatSquareDots 
    ChatSquareDots,
    /// ChatSquareDotsFill 
    ChatSquareDotsFill,
    /// ChatSquareFill 
    ChatSquareFill,
    /// ChatSquareHeart 
    ChatSquareHeart,
    /// ChatSquareHeartFill 
    ChatSquareHeartFill,
    /// ChatSquareQuote 
    ChatSquareQuote,
    /// ChatSquareQuoteFill 
    ChatSquareQuoteFill,
    /// ChatSquareText 
    ChatSquareText,
    /// ChatSquareTextFill 
    ChatSquareTextFill,
    /// ChatText 
    ChatText,
    /// ChatTextFill 
    ChatTextFill,
    /// Check 
    Check,
    /// CheckAll 
    CheckAll,
    /// CheckCircle 
    CheckCircle,
    /// CheckCircleFill 
    CheckCircleFill,
    /// CheckLg 
    CheckLg,
    /// CheckSquare 
    CheckSquare,
    /// CheckSquareFill 
    CheckSquareFill,
    /// Checktwo 
    Checktwo,
    /// ChecktwoAll 
    ChecktwoAll,
    /// ChecktwoCircle 
    ChecktwoCircle,
    /// ChecktwoSquare 
    ChecktwoSquare,
    /// ChevronBarContract 
    ChevronBarContract,
    /// ChevronBarDown 
    ChevronBarDown,
    /// ChevronBarExpand 
    ChevronBarExpand,
    /// ChevronBarLeft 
    ChevronBarLeft,
    /// ChevronBarRight 
    ChevronBarRight,
    /// ChevronBarUp 
    ChevronBarUp,
    /// ChevronCompactDown 
    ChevronCompactDown,
    /// ChevronCompactLeft 
    ChevronCompactLeft,
    /// ChevronCompactRight 
    ChevronCompactRight,
    /// ChevronCompactUp 
    ChevronCompactUp,
    /// ChevronContract 
    ChevronContract,
    /// ChevronDoubleDown 
    ChevronDoubleDown,
    /// ChevronDoubleLeft 
    ChevronDoubleLeft,
    /// ChevronDoubleRight 
    ChevronDoubleRight,
    /// ChevronDoubleUp 
    ChevronDoubleUp,
    /// ChevronDown 
    ChevronDown,
    /// ChevronExpand 
    ChevronExpand,
    /// ChevronLeft 
    ChevronLeft,
    /// ChevronRight 
    ChevronRight,
    /// ChevronUp 
    ChevronUp,
    /// Circle 
    Circle,
    /// CircleFill 
    CircleFill,
    /// CircleHalf 
    CircleHalf,
    /// CircleSquare 
    CircleSquare,
    /// Clipboard 
    Clipboard,
    /// ClipboardCheck 
    ClipboardCheck,
    /// ClipboardCheckFill 
    ClipboardCheckFill,
    /// ClipboardData 
    ClipboardData,
    /// ClipboardDataFill 
    ClipboardDataFill,
    /// ClipboardFill 
    ClipboardFill,
    /// ClipboardHeart 
    ClipboardHeart,
    /// ClipboardHeartFill 
    ClipboardHeartFill,
    /// ClipboardMinus 
    ClipboardMinus,
    /// ClipboardMinusFill 
    ClipboardMinusFill,
    /// ClipboardPlus 
    ClipboardPlus,
    /// ClipboardPlusFill 
    ClipboardPlusFill,
    /// ClipboardPulse 
    ClipboardPulse,
    /// ClipboardX 
    ClipboardX,
    /// ClipboardXFill 
    ClipboardXFill,
    /// Clipboardtwo 
    Clipboardtwo,
    /// ClipboardtwoCheck 
    ClipboardtwoCheck,
    /// ClipboardtwoCheckFill 
    ClipboardtwoCheckFill,
    /// ClipboardtwoData 
    ClipboardtwoData,
    /// ClipboardtwoDataFill 
    ClipboardtwoDataFill,
    /// ClipboardtwoFill 
    ClipboardtwoFill,
    /// ClipboardtwoHeart 
    ClipboardtwoHeart,
    /// ClipboardtwoHeartFill 
    ClipboardtwoHeartFill,
    /// ClipboardtwoMinus 
    ClipboardtwoMinus,
    /// ClipboardtwoMinusFill 
    ClipboardtwoMinusFill,
    /// ClipboardtwoPlus 
    ClipboardtwoPlus,
    /// ClipboardtwoPlusFill 
    ClipboardtwoPlusFill,
    /// ClipboardtwoPulse 
    ClipboardtwoPulse,
    /// ClipboardtwoPulseFill 
    ClipboardtwoPulseFill,
    /// ClipboardtwoX 
    ClipboardtwoX,
    /// ClipboardtwoXFill 
    ClipboardtwoXFill,
    /// Clock 
    Clock,
    /// ClockFill 
    ClockFill,
    /// ClockHistory 
    ClockHistory,
    /// Cloud 
    Cloud,
    /// CloudArrowDown 
    CloudArrowDown,
    /// CloudArrowDownFill 
    CloudArrowDownFill,
    /// CloudArrowUp 
    CloudArrowUp,
    /// CloudArrowUpFill 
    CloudArrowUpFill,
    /// CloudCheck 
    CloudCheck,
    /// CloudCheckFill 
    CloudCheckFill,
    /// CloudDownload 
    CloudDownload,
    /// CloudDownloadFill 
    CloudDownloadFill,
    /// CloudDrizzle 
    CloudDrizzle,
    /// CloudDrizzleFill 
    CloudDrizzleFill,
    /// CloudFill 
    CloudFill,
    /// CloudFog 
    CloudFog,
    /// CloudFogFill 
    CloudFogFill,
    /// CloudFogtwo 
    CloudFogtwo,
    /// CloudFogtwoFill 
    CloudFogtwoFill,
    /// CloudHail 
    CloudHail,
    /// CloudHailFill 
    CloudHailFill,
    /// CloudHaze 
    CloudHaze,
    /// CloudHazeFill 
    CloudHazeFill,
    /// CloudHazetwo 
    CloudHazetwo,
    /// CloudHazetwoFill 
    CloudHazetwoFill,
    /// CloudLightning 
    CloudLightning,
    /// CloudLightningFill 
    CloudLightningFill,
    /// CloudLightningRain 
    CloudLightningRain,
    /// CloudLightningRainFill 
    CloudLightningRainFill,
    /// CloudMinus 
    CloudMinus,
    /// CloudMinusFill 
    CloudMinusFill,
    /// CloudMoon 
    CloudMoon,
    /// CloudMoonFill 
    CloudMoonFill,
    /// CloudPlus 
    CloudPlus,
    /// CloudPlusFill 
    CloudPlusFill,
    /// CloudRain 
    CloudRain,
    /// CloudRainFill 
    CloudRainFill,
    /// CloudRainHeavy 
    CloudRainHeavy,
    /// CloudRainHeavyFill 
    CloudRainHeavyFill,
    /// CloudSlash 
    CloudSlash,
    /// CloudSlashFill 
    CloudSlashFill,
    /// CloudSleet 
    CloudSleet,
    /// CloudSleetFill 
    CloudSleetFill,
    /// CloudSnow 
    CloudSnow,
    /// CloudSnowFill 
    CloudSnowFill,
    /// CloudSun 
    CloudSun,
    /// CloudSunFill 
    CloudSunFill,
    /// CloudUpload 
    CloudUpload,
    /// CloudUploadFill 
    CloudUploadFill,
    /// Clouds 
    Clouds,
    /// CloudsFill 
    CloudsFill,
    /// Cloudy 
    Cloudy,
    /// CloudyFill 
    CloudyFill,
    /// Code 
    Code,
    /// CodeSlash 
    CodeSlash,
    /// CodeSquare 
    CodeSquare,
    /// Coin 
    Coin,
    /// Collection 
    Collection,
    /// CollectionFill 
    CollectionFill,
    /// CollectionPlay 
    CollectionPlay,
    /// CollectionPlayFill 
    CollectionPlayFill,
    /// Columns 
    Columns,
    /// ColumnsGap 
    ColumnsGap,
    /// Command 
    Command,
    /// Compass 
    Compass,
    /// CompassFill 
    CompassFill,
    /// Cone 
    Cone,
    /// ConeStriped 
    ConeStriped,
    /// Controller 
    Controller,
    /// Cookie 
    Cookie,
    /// Copy 
    Copy,
    /// Cpu 
    Cpu,
    /// CpuFill 
    CpuFill,
    /// CreditCard 
    CreditCard,
    /// CreditCardFill 
    CreditCardFill,
    /// CreditCardTwoBack 
    CreditCardTwoBack,
    /// CreditCardTwoBackFill 
    CreditCardTwoBackFill,
    /// CreditCardTwoFront 
    CreditCardTwoFront,
    /// CreditCardTwoFrontFill 
    CreditCardTwoFrontFill,
    /// Crop 
    Crop,
    /// Crosshair 
    Crosshair,
    /// Crosshairtwo 
    Crosshairtwo,
    /// Cup 
    Cup,
    /// CupFill 
    CupFill,
    /// CupHot 
    CupHot,
    /// CupHotFill 
    CupHotFill,
    /// CupStraw 
    CupStraw,
    /// CurrencyBitcoin 
    CurrencyBitcoin,
    /// CurrencyDollar 
    CurrencyDollar,
    /// CurrencyEuro 
    CurrencyEuro,
    /// CurrencyExchange 
    CurrencyExchange,
    /// CurrencyPound 
    CurrencyPound,
    /// CurrencyRupee 
    CurrencyRupee,
    /// CurrencyYen 
    CurrencyYen,
    /// Cursor 
    Cursor,
    /// CursorFill 
    CursorFill,
    /// CursorText 
    CursorText,
    /// Dash 
    Dash,
    /// DashCircle 
    DashCircle,
    /// DashCircleDotted 
    DashCircleDotted,
    /// DashCircleFill 
    DashCircleFill,
    /// DashLg 
    DashLg,
    /// DashSquare 
    DashSquare,
    /// DashSquareDotted 
    DashSquareDotted,
    /// DashSquareFill 
    DashSquareFill,
    /// Database 
    Database,
    /// DatabaseAdd 
    DatabaseAdd,
    /// DatabaseCheck 
    DatabaseCheck,
    /// DatabaseDash 
    DatabaseDash,
    /// DatabaseDown 
    DatabaseDown,
    /// DatabaseExclamation 
    DatabaseExclamation,
    /// DatabaseFill 
    DatabaseFill,
    /// DatabaseFillAdd 
    DatabaseFillAdd,
    /// DatabaseFillCheck 
    DatabaseFillCheck,
    /// DatabaseFillDash 
    DatabaseFillDash,
    /// DatabaseFillDown 
    DatabaseFillDown,
    /// DatabaseFillExclamation 
    DatabaseFillExclamation,
    /// DatabaseFillGear 
    DatabaseFillGear,
    /// DatabaseFillLock 
    DatabaseFillLock,
    /// DatabaseFillSlash 
    DatabaseFillSlash,
    /// DatabaseFillUp 
    DatabaseFillUp,
    /// DatabaseFillX 
    DatabaseFillX,
    /// DatabaseGear 
    DatabaseGear,
    /// DatabaseLock 
    DatabaseLock,
    /// DatabaseSlash 
    DatabaseSlash,
    /// DatabaseUp 
    DatabaseUp,
    /// DatabaseX 
    DatabaseX,
    /// DeviceHdd 
    DeviceHdd,
    /// DeviceHddFill 
    DeviceHddFill,
    /// DeviceSsd 
    DeviceSsd,
    /// DeviceSsdFill 
    DeviceSsdFill,
    /// DiagramThree 
    DiagramThree,
    /// DiagramThreeFill 
    DiagramThreeFill,
    /// DiagramTwo 
    DiagramTwo,
    /// DiagramTwoFill 
    DiagramTwoFill,
    /// Diamond 
    Diamond,
    /// DiamondFill 
    DiamondFill,
    /// DiamondHalf 
    DiamondHalf,
    /// DiceFive 
    DiceFive,
    /// DiceFiveFill 
    DiceFiveFill,
    /// DiceFour 
    DiceFour,
    /// DiceFourFill 
    DiceFourFill,
    /// DiceOne 
    DiceOne,
    /// DiceOneFill 
    DiceOneFill,
    /// DiceSix 
    DiceSix,
    /// DiceSixFill 
    DiceSixFill,
    /// DiceThree 
    DiceThree,
    /// DiceThreeFill 
    DiceThreeFill,
    /// DiceTwo 
    DiceTwo,
    /// DiceTwoFill 
    DiceTwoFill,
    /// Disc 
    Disc,
    /// DiscFill 
    DiscFill,
    /// Discord 
    Discord,
    /// Display 
    Display,
    /// DisplayFill 
    DisplayFill,
    /// Displayport 
    Displayport,
    /// DisplayportFill 
    DisplayportFill,
    /// DistributeHorizontal 
    DistributeHorizontal,
    /// DistributeVertical 
    DistributeVertical,
    /// DoorClosed 
    DoorClosed,
    /// DoorClosedFill 
    DoorClosedFill,
    /// DoorOpen 
    DoorOpen,
    /// DoorOpenFill 
    DoorOpenFill,
    /// Dot 
    Dot,
    /// Download 
    Download,
    /// Dpad 
    Dpad,
    /// DpadFill 
    DpadFill,
    /// Dribbble 
    Dribbble,
    /// Dropbox 
    Dropbox,
    /// Droplet 
    Droplet,
    /// DropletFill 
    DropletFill,
    /// DropletHalf 
    DropletHalf,
    /// Duffle 
    Duffle,
    /// DuffleFill 
    DuffleFill,
    /// Ear 
    Ear,
    /// EarFill 
    EarFill,
    /// Earbuds 
    Earbuds,
    /// Easel 
    Easel,
    /// EaselFill 
    EaselFill,
    /// Easelthree 
    Easelthree,
    /// EaselthreeFill 
    EaselthreeFill,
    /// Easeltwo 
    Easeltwo,
    /// EaseltwoFill 
    EaseltwoFill,
    /// Egg 
    Egg,
    /// EggFill 
    EggFill,
    /// EggFried 
    EggFried,
    /// EightCircle 
    EightCircle,
    /// EightCircleFill 
    EightCircleFill,
    /// EightSquare 
    EightSquare,
    /// EightSquareFill 
    EightSquareFill,
    /// Eject 
    Eject,
    /// EjectFill 
    EjectFill,
    /// EmojiAngry 
    EmojiAngry,
    /// EmojiAngryFill 
    EmojiAngryFill,
    /// EmojiAstonished 
    EmojiAstonished,
    /// EmojiAstonishedFill 
    EmojiAstonishedFill,
    /// EmojiDizzy 
    EmojiDizzy,
    /// EmojiDizzyFill 
    EmojiDizzyFill,
    /// EmojiExpressionless 
    EmojiExpressionless,
    /// EmojiExpressionlessFill 
    EmojiExpressionlessFill,
    /// EmojiFrown 
    EmojiFrown,
    /// EmojiFrownFill 
    EmojiFrownFill,
    /// EmojiGrimace 
    EmojiGrimace,
    /// EmojiGrimaceFill 
    EmojiGrimaceFill,
    /// EmojiGrin 
    EmojiGrin,
    /// EmojiGrinFill 
    EmojiGrinFill,
    /// EmojiHeartEyes 
    EmojiHeartEyes,
    /// EmojiHeartEyesFill 
    EmojiHeartEyesFill,
    /// EmojiKiss 
    EmojiKiss,
    /// EmojiKissFill 
    EmojiKissFill,
    /// EmojiLaughing 
    EmojiLaughing,
    /// EmojiLaughingFill 
    EmojiLaughingFill,
    /// EmojiNeutral 
    EmojiNeutral,
    /// EmojiNeutralFill 
    EmojiNeutralFill,
    /// EmojiSmile 
    EmojiSmile,
    /// EmojiSmileFill 
    EmojiSmileFill,
    /// EmojiSmileUpsideDown 
    EmojiSmileUpsideDown,
    /// EmojiSmileUpsideDownFill 
    EmojiSmileUpsideDownFill,
    /// EmojiSunglasses 
    EmojiSunglasses,
    /// EmojiSunglassesFill 
    EmojiSunglassesFill,
    /// EmojiSurprise 
    EmojiSurprise,
    /// EmojiSurpriseFill 
    EmojiSurpriseFill,
    /// EmojiTear 
    EmojiTear,
    /// EmojiTearFill 
    EmojiTearFill,
    /// EmojiWink 
    EmojiWink,
    /// EmojiWinkFill 
    EmojiWinkFill,
    /// Envelope 
    Envelope,
    /// EnvelopeArrowDown 
    EnvelopeArrowDown,
    /// EnvelopeArrowDownFill 
    EnvelopeArrowDownFill,
    /// EnvelopeArrowUp 
    EnvelopeArrowUp,
    /// EnvelopeArrowUpFill 
    EnvelopeArrowUpFill,
    /// EnvelopeAt 
    EnvelopeAt,
    /// EnvelopeAtFill 
    EnvelopeAtFill,
    /// EnvelopeCheck 
    EnvelopeCheck,
    /// EnvelopeCheckFill 
    EnvelopeCheckFill,
    /// EnvelopeDash 
    EnvelopeDash,
    /// EnvelopeDashFill 
    EnvelopeDashFill,
    /// EnvelopeExclamation 
    EnvelopeExclamation,
    /// EnvelopeExclamationFill 
    EnvelopeExclamationFill,
    /// EnvelopeFill 
    EnvelopeFill,
    /// EnvelopeHeart 
    EnvelopeHeart,
    /// EnvelopeHeartFill 
    EnvelopeHeartFill,
    /// EnvelopeOpen 
    EnvelopeOpen,
    /// EnvelopeOpenFill 
    EnvelopeOpenFill,
    /// EnvelopeOpenHeart 
    EnvelopeOpenHeart,
    /// EnvelopeOpenHeartFill 
    EnvelopeOpenHeartFill,
    /// EnvelopePaper 
    EnvelopePaper,
    /// EnvelopePaperFill 
    EnvelopePaperFill,
    /// EnvelopePaperHeart 
    EnvelopePaperHeart,
    /// EnvelopePaperHeartFill 
    EnvelopePaperHeartFill,
    /// EnvelopePlus 
    EnvelopePlus,
    /// EnvelopePlusFill 
    EnvelopePlusFill,
    /// EnvelopeSlash 
    EnvelopeSlash,
    /// EnvelopeSlashFill 
    EnvelopeSlashFill,
    /// EnvelopeX 
    EnvelopeX,
    /// EnvelopeXFill 
    EnvelopeXFill,
    /// Eraser 
    Eraser,
    /// EraserFill 
    EraserFill,
    /// Escape 
    Escape,
    /// Ethernet 
    Ethernet,
    /// EvFront 
    EvFront,
    /// EvFrontFill 
    EvFrontFill,
    /// EvStation 
    EvStation,
    /// EvStationFill 
    EvStationFill,
    /// Exclamation 
    Exclamation,
    /// ExclamationCircle 
    ExclamationCircle,
    /// ExclamationCircleFill 
    ExclamationCircleFill,
    /// ExclamationDiamond 
    ExclamationDiamond,
    /// ExclamationDiamondFill 
    ExclamationDiamondFill,
    /// ExclamationLg 
    ExclamationLg,
    /// ExclamationOctagon 
    ExclamationOctagon,
    /// ExclamationOctagonFill 
    ExclamationOctagonFill,
    /// ExclamationSquare 
    ExclamationSquare,
    /// ExclamationSquareFill 
    ExclamationSquareFill,
    /// ExclamationTriangle 
    ExclamationTriangle,
    /// ExclamationTriangleFill 
    ExclamationTriangleFill,
    /// Exclude 
    Exclude,
    /// Explicit 
    Explicit,
    /// ExplicitFill 
    ExplicitFill,
    /// Exposure 
    Exposure,
    /// Eye 
    Eye,
    /// EyeFill 
    EyeFill,
    /// EyeSlash 
    EyeSlash,
    /// EyeSlashFill 
    EyeSlashFill,
    /// Eyedropper 
    Eyedropper,
    /// Eyeglasses 
    Eyeglasses,
    /// Facebook 
    Facebook,
    /// Fan 
    Fan,
    /// FastForward 
    FastForward,
    /// FastForwardBtn 
    FastForwardBtn,
    /// FastForwardBtnFill 
    FastForwardBtnFill,
    /// FastForwardCircle 
    FastForwardCircle,
    /// FastForwardCircleFill 
    FastForwardCircleFill,
    /// FastForwardFill 
    FastForwardFill,
    /// Feather 
    Feather,
    /// Feathertwo 
    Feathertwo,
    /// File 
    File,
    /// FileArrowDown 
    FileArrowDown,
    /// FileArrowDownFill 
    FileArrowDownFill,
    /// FileArrowUp 
    FileArrowUp,
    /// FileArrowUpFill 
    FileArrowUpFill,
    /// FileBarGraph 
    FileBarGraph,
    /// FileBarGraphFill 
    FileBarGraphFill,
    /// FileBinary 
    FileBinary,
    /// FileBinaryFill 
    FileBinaryFill,
    /// FileBreak 
    FileBreak,
    /// FileBreakFill 
    FileBreakFill,
    /// FileCheck 
    FileCheck,
    /// FileCheckFill 
    FileCheckFill,
    /// FileCode 
    FileCode,
    /// FileCodeFill 
    FileCodeFill,
    /// FileDiff 
    FileDiff,
    /// FileDiffFill 
    FileDiffFill,
    /// FileEarmark 
    FileEarmark,
    /// FileEarmarkArrowDown 
    FileEarmarkArrowDown,
    /// FileEarmarkArrowDownFill 
    FileEarmarkArrowDownFill,
    /// FileEarmarkArrowUp 
    FileEarmarkArrowUp,
    /// FileEarmarkArrowUpFill 
    FileEarmarkArrowUpFill,
    /// FileEarmarkBarGraph 
    FileEarmarkBarGraph,
    /// FileEarmarkBarGraphFill 
    FileEarmarkBarGraphFill,
    /// FileEarmarkBinary 
    FileEarmarkBinary,
    /// FileEarmarkBinaryFill 
    FileEarmarkBinaryFill,
    /// FileEarmarkBreak 
    FileEarmarkBreak,
    /// FileEarmarkBreakFill 
    FileEarmarkBreakFill,
    /// FileEarmarkCheck 
    FileEarmarkCheck,
    /// FileEarmarkCheckFill 
    FileEarmarkCheckFill,
    /// FileEarmarkCode 
    FileEarmarkCode,
    /// FileEarmarkCodeFill 
    FileEarmarkCodeFill,
    /// FileEarmarkDiff 
    FileEarmarkDiff,
    /// FileEarmarkDiffFill 
    FileEarmarkDiffFill,
    /// FileEarmarkEasel 
    FileEarmarkEasel,
    /// FileEarmarkEaselFill 
    FileEarmarkEaselFill,
    /// FileEarmarkExcel 
    FileEarmarkExcel,
    /// FileEarmarkExcelFill 
    FileEarmarkExcelFill,
    /// FileEarmarkFill 
    FileEarmarkFill,
    /// FileEarmarkFont 
    FileEarmarkFont,
    /// FileEarmarkFontFill 
    FileEarmarkFontFill,
    /// FileEarmarkImage 
    FileEarmarkImage,
    /// FileEarmarkImageFill 
    FileEarmarkImageFill,
    /// FileEarmarkLock 
    FileEarmarkLock,
    /// FileEarmarkLockFill 
    FileEarmarkLockFill,
    /// FileEarmarkLocktwo 
    FileEarmarkLocktwo,
    /// FileEarmarkLocktwoFill 
    FileEarmarkLocktwoFill,
    /// FileEarmarkMedical 
    FileEarmarkMedical,
    /// FileEarmarkMedicalFill 
    FileEarmarkMedicalFill,
    /// FileEarmarkMinus 
    FileEarmarkMinus,
    /// FileEarmarkMinusFill 
    FileEarmarkMinusFill,
    /// FileEarmarkMusic 
    FileEarmarkMusic,
    /// FileEarmarkMusicFill 
    FileEarmarkMusicFill,
    /// FileEarmarkPdf 
    FileEarmarkPdf,
    /// FileEarmarkPdfFill 
    FileEarmarkPdfFill,
    /// FileEarmarkPerson 
    FileEarmarkPerson,
    /// FileEarmarkPersonFill 
    FileEarmarkPersonFill,
    /// FileEarmarkPlay 
    FileEarmarkPlay,
    /// FileEarmarkPlayFill 
    FileEarmarkPlayFill,
    /// FileEarmarkPlus 
    FileEarmarkPlus,
    /// FileEarmarkPlusFill 
    FileEarmarkPlusFill,
    /// FileEarmarkPost 
    FileEarmarkPost,
    /// FileEarmarkPostFill 
    FileEarmarkPostFill,
    /// FileEarmarkPpt 
    FileEarmarkPpt,
    /// FileEarmarkPptFill 
    FileEarmarkPptFill,
    /// FileEarmarkRichtext 
    FileEarmarkRichtext,
    /// FileEarmarkRichtextFill 
    FileEarmarkRichtextFill,
    /// FileEarmarkRuled 
    FileEarmarkRuled,
    /// FileEarmarkRuledFill 
    FileEarmarkRuledFill,
    /// FileEarmarkSlides 
    FileEarmarkSlides,
    /// FileEarmarkSlidesFill 
    FileEarmarkSlidesFill,
    /// FileEarmarkSpreadsheet 
    FileEarmarkSpreadsheet,
    /// FileEarmarkSpreadsheetFill 
    FileEarmarkSpreadsheetFill,
    /// FileEarmarkText 
    FileEarmarkText,
    /// FileEarmarkTextFill 
    FileEarmarkTextFill,
    /// FileEarmarkWord 
    FileEarmarkWord,
    /// FileEarmarkWordFill 
    FileEarmarkWordFill,
    /// FileEarmarkX 
    FileEarmarkX,
    /// FileEarmarkXFill 
    FileEarmarkXFill,
    /// FileEarmarkZip 
    FileEarmarkZip,
    /// FileEarmarkZipFill 
    FileEarmarkZipFill,
    /// FileEasel 
    FileEasel,
    /// FileEaselFill 
    FileEaselFill,
    /// FileExcel 
    FileExcel,
    /// FileExcelFill 
    FileExcelFill,
    /// FileFill 
    FileFill,
    /// FileFont 
    FileFont,
    /// FileFontFill 
    FileFontFill,
    /// FileImage 
    FileImage,
    /// FileImageFill 
    FileImageFill,
    /// FileLock 
    FileLock,
    /// FileLockFill 
    FileLockFill,
    /// FileLocktwo 
    FileLocktwo,
    /// FileLocktwoFill 
    FileLocktwoFill,
    /// FileMedical 
    FileMedical,
    /// FileMedicalFill 
    FileMedicalFill,
    /// FileMinus 
    FileMinus,
    /// FileMinusFill 
    FileMinusFill,
    /// FileMusic 
    FileMusic,
    /// FileMusicFill 
    FileMusicFill,
    /// FilePdf 
    FilePdf,
    /// FilePdfFill 
    FilePdfFill,
    /// FilePerson 
    FilePerson,
    /// FilePersonFill 
    FilePersonFill,
    /// FilePlay 
    FilePlay,
    /// FilePlayFill 
    FilePlayFill,
    /// FilePlus 
    FilePlus,
    /// FilePlusFill 
    FilePlusFill,
    /// FilePost 
    FilePost,
    /// FilePostFill 
    FilePostFill,
    /// FilePpt 
    FilePpt,
    /// FilePptFill 
    FilePptFill,
    /// FileRichtext 
    FileRichtext,
    /// FileRichtextFill 
    FileRichtextFill,
    /// FileRuled 
    FileRuled,
    /// FileRuledFill 
    FileRuledFill,
    /// FileSlides 
    FileSlides,
    /// FileSlidesFill 
    FileSlidesFill,
    /// FileSpreadsheet 
    FileSpreadsheet,
    /// FileSpreadsheetFill 
    FileSpreadsheetFill,
    /// FileText 
    FileText,
    /// FileTextFill 
    FileTextFill,
    /// FileWord 
    FileWord,
    /// FileWordFill 
    FileWordFill,
    /// FileX 
    FileX,
    /// FileXFill 
    FileXFill,
    /// FileZip 
    FileZip,
    /// FileZipFill 
    FileZipFill,
    /// Files 
    Files,
    /// FilesAlt 
    FilesAlt,
    /// FiletypeAac 
    FiletypeAac,
    /// FiletypeAi 
    FiletypeAi,
    /// FiletypeBmp 
    FiletypeBmp,
    /// FiletypeCs 
    FiletypeCs,
    /// FiletypeCss 
    FiletypeCss,
    /// FiletypeCsv 
    FiletypeCsv,
    /// FiletypeDoc 
    FiletypeDoc,
    /// FiletypeDocx 
    FiletypeDocx,
    /// FiletypeExe 
    FiletypeExe,
    /// FiletypeGif 
    FiletypeGif,
    /// FiletypeHeic 
    FiletypeHeic,
    /// FiletypeHtml 
    FiletypeHtml,
    /// FiletypeJava 
    FiletypeJava,
    /// FiletypeJpg 
    FiletypeJpg,
    /// FiletypeJs 
    FiletypeJs,
    /// FiletypeJson 
    FiletypeJson,
    /// FiletypeJsx 
    FiletypeJsx,
    /// FiletypeKey 
    FiletypeKey,
    /// FiletypeMd 
    FiletypeMd,
    /// FiletypeMdx 
    FiletypeMdx,
    /// FiletypeMfourp 
    FiletypeMfourp,
    /// FiletypeMov 
    FiletypeMov,
    /// FiletypeMpfour 
    FiletypeMpfour,
    /// FiletypeMpthree 
    FiletypeMpthree,
    /// FiletypeOtf 
    FiletypeOtf,
    /// FiletypePdf 
    FiletypePdf,
    /// FiletypePhp 
    FiletypePhp,
    /// FiletypePng 
    FiletypePng,
    /// FiletypePpt 
    FiletypePpt,
    /// FiletypePptx 
    FiletypePptx,
    /// FiletypePsd 
    FiletypePsd,
    /// FiletypePy 
    FiletypePy,
    /// FiletypeRaw 
    FiletypeRaw,
    /// FiletypeRb 
    FiletypeRb,
    /// FiletypeSass 
    FiletypeSass,
    /// FiletypeScss 
    FiletypeScss,
    /// FiletypeSh 
    FiletypeSh,
    /// FiletypeSql 
    FiletypeSql,
    /// FiletypeSvg 
    FiletypeSvg,
    /// FiletypeTiff 
    FiletypeTiff,
    /// FiletypeTsx 
    FiletypeTsx,
    /// FiletypeTtf 
    FiletypeTtf,
    /// FiletypeTxt 
    FiletypeTxt,
    /// FiletypeWav 
    FiletypeWav,
    /// FiletypeWoff 
    FiletypeWoff,
    /// FiletypeXls 
    FiletypeXls,
    /// FiletypeXlsx 
    FiletypeXlsx,
    /// FiletypeXml 
    FiletypeXml,
    /// FiletypeYml 
    FiletypeYml,
    /// Film 
    Film,
    /// Filter 
    Filter,
    /// FilterCircle 
    FilterCircle,
    /// FilterCircleFill 
    FilterCircleFill,
    /// FilterLeft 
    FilterLeft,
    /// FilterRight 
    FilterRight,
    /// FilterSquare 
    FilterSquare,
    /// FilterSquareFill 
    FilterSquareFill,
    /// Fingerprint 
    Fingerprint,
    /// Fire 
    Fire,
    /// FiveCircle 
    FiveCircle,
    /// FiveCircleFill 
    FiveCircleFill,
    /// FiveSquare 
    FiveSquare,
    /// FiveSquareFill 
    FiveSquareFill,
    /// Flag 
    Flag,
    /// FlagFill 
    FlagFill,
    /// Floppy 
    Floppy,
    /// FloppyFill 
    FloppyFill,
    /// Floppytwo 
    Floppytwo,
    /// FloppytwoFill 
    FloppytwoFill,
    /// Flowerone 
    Flowerone,
    /// Flowerthree 
    Flowerthree,
    /// Flowertwo 
    Flowertwo,
    /// Folder 
    Folder,
    /// FolderCheck 
    FolderCheck,
    /// FolderFill 
    FolderFill,
    /// FolderMinus 
    FolderMinus,
    /// FolderPlus 
    FolderPlus,
    /// FolderSymlink 
    FolderSymlink,
    /// FolderSymlinkFill 
    FolderSymlinkFill,
    /// FolderX 
    FolderX,
    /// Foldertwo 
    Foldertwo,
    /// FoldertwoOpen 
    FoldertwoOpen,
    /// Fonts 
    Fonts,
    /// Forward 
    Forward,
    /// ForwardFill 
    ForwardFill,
    /// FourCircle 
    FourCircle,
    /// FourCircleFill 
    FourCircleFill,
    /// FourSquare 
    FourSquare,
    /// FourSquareFill 
    FourSquareFill,
    /// Front 
    Front,
    /// FuelPump 
    FuelPump,
    /// FuelPumpDiesel 
    FuelPumpDiesel,
    /// FuelPumpDieselFill 
    FuelPumpDieselFill,
    /// FuelPumpFill 
    FuelPumpFill,
    /// Fullscreen 
    Fullscreen,
    /// FullscreenExit 
    FullscreenExit,
    /// Funnel 
    Funnel,
    /// FunnelFill 
    FunnelFill,
    /// Gear 
    Gear,
    /// GearFill 
    GearFill,
    /// GearWide 
    GearWide,
    /// GearWideConnected 
    GearWideConnected,
    /// Gem 
    Gem,
    /// GenderAmbiguous 
    GenderAmbiguous,
    /// GenderFemale 
    GenderFemale,
    /// GenderMale 
    GenderMale,
    /// GenderNeuter 
    GenderNeuter,
    /// GenderTrans 
    GenderTrans,
    /// Geo 
    Geo,
    /// GeoAlt 
    GeoAlt,
    /// GeoAltFill 
    GeoAltFill,
    /// GeoFill 
    GeoFill,
    /// Gift 
    Gift,
    /// GiftFill 
    GiftFill,
    /// Git 
    Git,
    /// Github 
    Github,
    /// Gitlab 
    Gitlab,
    /// Globe 
    Globe,
    /// GlobeAmericas 
    GlobeAmericas,
    /// GlobeAsiaAustralia 
    GlobeAsiaAustralia,
    /// GlobeCentralSouthAsia 
    GlobeCentralSouthAsia,
    /// GlobeEuropeAfrica 
    GlobeEuropeAfrica,
    /// Globetwo 
    Globetwo,
    /// Google 
    Google,
    /// GooglePlay 
    GooglePlay,
    /// GpuCard 
    GpuCard,
    /// GraphDown 
    GraphDown,
    /// GraphDownArrow 
    GraphDownArrow,
    /// GraphUp 
    GraphUp,
    /// GraphUpArrow 
    GraphUpArrow,
    /// Grid 
    Grid,
    /// GridFill 
    GridFill,
    /// GridOnextwo 
    GridOnextwo,
    /// GridOnextwoFill 
    GridOnextwoFill,
    /// GridThreexthree 
    GridThreexthree,
    /// GridThreexthreeGap 
    GridThreexthreeGap,
    /// GridThreexthreeGapFill 
    GridThreexthreeGapFill,
    /// GridThreextwo 
    GridThreextwo,
    /// GridThreextwoGap 
    GridThreextwoGap,
    /// GridThreextwoGapFill 
    GridThreextwoGapFill,
    /// GripHorizontal 
    GripHorizontal,
    /// GripVertical 
    GripVertical,
    /// HCircle 
    HCircle,
    /// HCircleFill 
    HCircleFill,
    /// HSquare 
    HSquare,
    /// HSquareFill 
    HSquareFill,
    /// Hammer 
    Hammer,
    /// HandIndex 
    HandIndex,
    /// HandIndexFill 
    HandIndexFill,
    /// HandIndexThumb 
    HandIndexThumb,
    /// HandIndexThumbFill 
    HandIndexThumbFill,
    /// HandThumbsDown 
    HandThumbsDown,
    /// HandThumbsDownFill 
    HandThumbsDownFill,
    /// HandThumbsUp 
    HandThumbsUp,
    /// HandThumbsUpFill 
    HandThumbsUpFill,
    /// Handbag 
    Handbag,
    /// HandbagFill 
    HandbagFill,
    /// Hash 
    Hash,
    /// Hdd 
    Hdd,
    /// HddFill 
    HddFill,
    /// HddNetwork 
    HddNetwork,
    /// HddNetworkFill 
    HddNetworkFill,
    /// HddRack 
    HddRack,
    /// HddRackFill 
    HddRackFill,
    /// HddStack 
    HddStack,
    /// HddStackFill 
    HddStackFill,
    /// Hdmi 
    Hdmi,
    /// HdmiFill 
    HdmiFill,
    /// Headphones 
    Headphones,
    /// Headset 
    Headset,
    /// HeadsetVr 
    HeadsetVr,
    /// Heart 
    Heart,
    /// HeartArrow 
    HeartArrow,
    /// HeartFill 
    HeartFill,
    /// HeartHalf 
    HeartHalf,
    /// HeartPulse 
    HeartPulse,
    /// HeartPulseFill 
    HeartPulseFill,
    /// Heartbreak 
    Heartbreak,
    /// HeartbreakFill 
    HeartbreakFill,
    /// Hearts 
    Hearts,
    /// Heptagon 
    Heptagon,
    /// HeptagonFill 
    HeptagonFill,
    /// HeptagonHalf 
    HeptagonHalf,
    /// Hexagon 
    Hexagon,
    /// HexagonFill 
    HexagonFill,
    /// HexagonHalf 
    HexagonHalf,
    /// Highlighter 
    Highlighter,
    /// Highlights 
    Highlights,
    /// Hospital 
    Hospital,
    /// HospitalFill 
    HospitalFill,
    /// Hourglass 
    Hourglass,
    /// HourglassBottom 
    HourglassBottom,
    /// HourglassSplit 
    HourglassSplit,
    /// HourglassTop 
    HourglassTop,
    /// House 
    House,
    /// HouseAdd 
    HouseAdd,
    /// HouseAddFill 
    HouseAddFill,
    /// HouseCheck 
    HouseCheck,
    /// HouseCheckFill 
    HouseCheckFill,
    /// HouseDash 
    HouseDash,
    /// HouseDashFill 
    HouseDashFill,
    /// HouseDoor 
    HouseDoor,
    /// HouseDoorFill 
    HouseDoorFill,
    /// HouseDown 
    HouseDown,
    /// HouseDownFill 
    HouseDownFill,
    /// HouseExclamation 
    HouseExclamation,
    /// HouseExclamationFill 
    HouseExclamationFill,
    /// HouseFill 
    HouseFill,
    /// HouseGear 
    HouseGear,
    /// HouseGearFill 
    HouseGearFill,
    /// HouseHeart 
    HouseHeart,
    /// HouseHeartFill 
    HouseHeartFill,
    /// HouseLock 
    HouseLock,
    /// HouseLockFill 
    HouseLockFill,
    /// HouseSlash 
    HouseSlash,
    /// HouseSlashFill 
    HouseSlashFill,
    /// HouseUp 
    HouseUp,
    /// HouseUpFill 
    HouseUpFill,
    /// HouseX 
    HouseX,
    /// HouseXFill 
    HouseXFill,
    /// Houses 
    Houses,
    /// HousesFill 
    HousesFill,
    /// Hr 
    Hr,
    /// Hurricane 
    Hurricane,
    /// Hypnotize 
    Hypnotize,
    /// Image 
    Image,
    /// ImageAlt 
    ImageAlt,
    /// ImageFill 
    ImageFill,
    /// Images 
    Images,
    /// Inbox 
    Inbox,
    /// InboxFill 
    InboxFill,
    /// Inboxes 
    Inboxes,
    /// InboxesFill 
    InboxesFill,
    /// Incognito 
    Incognito,
    /// Indent 
    Indent,
    /// Infinity 
    Infinity,
    /// Info 
    Info,
    /// InfoCircle 
    InfoCircle,
    /// InfoCircleFill 
    InfoCircleFill,
    /// InfoLg 
    InfoLg,
    /// InfoSquare 
    InfoSquare,
    /// InfoSquareFill 
    InfoSquareFill,
    /// InputCursor 
    InputCursor,
    /// InputCursorText 
    InputCursorText,
    /// Instagram 
    Instagram,
    /// Intersect 
    Intersect,
    /// Journal 
    Journal,
    /// JournalAlbum 
    JournalAlbum,
    /// JournalArrowDown 
    JournalArrowDown,
    /// JournalArrowUp 
    JournalArrowUp,
    /// JournalBookmark 
    JournalBookmark,
    /// JournalBookmarkFill 
    JournalBookmarkFill,
    /// JournalCheck 
    JournalCheck,
    /// JournalCode 
    JournalCode,
    /// JournalMedical 
    JournalMedical,
    /// JournalMinus 
    JournalMinus,
    /// JournalPlus 
    JournalPlus,
    /// JournalRichtext 
    JournalRichtext,
    /// JournalText 
    JournalText,
    /// JournalX 
    JournalX,
    /// Journals 
    Journals,
    /// Joystick 
    Joystick,
    /// Justify 
    Justify,
    /// JustifyLeft 
    JustifyLeft,
    /// JustifyRight 
    JustifyRight,
    /// Kanban 
    Kanban,
    /// KanbanFill 
    KanbanFill,
    /// Key 
    Key,
    /// KeyFill 
    KeyFill,
    /// Keyboard 
    Keyboard,
    /// KeyboardFill 
    KeyboardFill,
    /// Ladder 
    Ladder,
    /// Lamp 
    Lamp,
    /// LampFill 
    LampFill,
    /// Laptop 
    Laptop,
    /// LaptopFill 
    LaptopFill,
    /// LayerBackward 
    LayerBackward,
    /// LayerForward 
    LayerForward,
    /// Layers 
    Layers,
    /// LayersFill 
    LayersFill,
    /// LayersHalf 
    LayersHalf,
    /// LayoutSidebar 
    LayoutSidebar,
    /// LayoutSidebarInset 
    LayoutSidebarInset,
    /// LayoutSidebarInsetReverse 
    LayoutSidebarInsetReverse,
    /// LayoutSidebarReverse 
    LayoutSidebarReverse,
    /// LayoutSplit 
    LayoutSplit,
    /// LayoutTextSidebar 
    LayoutTextSidebar,
    /// LayoutTextSidebarReverse 
    LayoutTextSidebarReverse,
    /// LayoutTextWindow 
    LayoutTextWindow,
    /// LayoutTextWindowReverse 
    LayoutTextWindowReverse,
    /// LayoutThreeColumns 
    LayoutThreeColumns,
    /// LayoutWtf 
    LayoutWtf,
    /// LifePreserver 
    LifePreserver,
    /// Lightbulb 
    Lightbulb,
    /// LightbulbFill 
    LightbulbFill,
    /// LightbulbOff 
    LightbulbOff,
    /// LightbulbOffFill 
    LightbulbOffFill,
    /// Lightning 
    Lightning,
    /// LightningCharge 
    LightningCharge,
    /// LightningChargeFill 
    LightningChargeFill,
    /// LightningFill 
    LightningFill,
    /// Line 
    Line,
    /// Link 
    Link,
    /// LinkFourfivedeg 
    LinkFourfivedeg,
    /// Linkedin 
    Linkedin,
    /// List 
    List,
    /// ListCheck 
    ListCheck,
    /// ListColumns 
    ListColumns,
    /// ListColumnsReverse 
    ListColumnsReverse,
    /// ListNested 
    ListNested,
    /// ListOl 
    ListOl,
    /// ListStars 
    ListStars,
    /// ListTask 
    ListTask,
    /// ListUl 
    ListUl,
    /// Lock 
    Lock,
    /// LockFill 
    LockFill,
    /// Luggage 
    Luggage,
    /// LuggageFill 
    LuggageFill,
    /// Lungs 
    Lungs,
    /// LungsFill 
    LungsFill,
    /// Magic 
    Magic,
    /// Magnet 
    Magnet,
    /// MagnetFill 
    MagnetFill,
    /// Mailbox 
    Mailbox,
    /// MailboxFlag 
    MailboxFlag,
    /// Mailboxtwo 
    Mailboxtwo,
    /// MailboxtwoFlag 
    MailboxtwoFlag,
    /// Map 
    Map,
    /// MapFill 
    MapFill,
    /// Markdown 
    Markdown,
    /// MarkdownFill 
    MarkdownFill,
    /// MarkerTip 
    MarkerTip,
    /// Mask 
    Mask,
    /// Mastodon 
    Mastodon,
    /// Medium 
    Medium,
    /// Megaphone 
    Megaphone,
    /// MegaphoneFill 
    MegaphoneFill,
    /// Memory 
    Memory,
    /// MenuApp 
    MenuApp,
    /// MenuAppFill 
    MenuAppFill,
    /// MenuButton 
    MenuButton,
    /// MenuButtonFill 
    MenuButtonFill,
    /// MenuButtonWide 
    MenuButtonWide,
    /// MenuButtonWideFill 
    MenuButtonWideFill,
    /// MenuDown 
    MenuDown,
    /// MenuUp 
    MenuUp,
    /// Messenger 
    Messenger,
    /// Meta 
    Meta,
    /// Mic 
    Mic,
    /// MicFill 
    MicFill,
    /// MicMute 
    MicMute,
    /// MicMuteFill 
    MicMuteFill,
    /// Microsoft 
    Microsoft,
    /// MicrosoftTeams 
    MicrosoftTeams,
    /// Minecart 
    Minecart,
    /// MinecartLoaded 
    MinecartLoaded,
    /// Modem 
    Modem,
    /// ModemFill 
    ModemFill,
    /// Moisture 
    Moisture,
    /// Moon 
    Moon,
    /// MoonFill 
    MoonFill,
    /// MoonStars 
    MoonStars,
    /// MoonStarsFill 
    MoonStarsFill,
    /// Mortarboard 
    Mortarboard,
    /// MortarboardFill 
    MortarboardFill,
    /// Motherboard 
    Motherboard,
    /// MotherboardFill 
    MotherboardFill,
    /// Mouse 
    Mouse,
    /// MouseFill 
    MouseFill,
    /// Mousethree 
    Mousethree,
    /// MousethreeFill 
    MousethreeFill,
    /// Mousetwo 
    Mousetwo,
    /// MousetwoFill 
    MousetwoFill,
    /// MusicNote 
    MusicNote,
    /// MusicNoteBeamed 
    MusicNoteBeamed,
    /// MusicNoteList 
    MusicNoteList,
    /// MusicPlayer 
    MusicPlayer,
    /// MusicPlayerFill 
    MusicPlayerFill,
    /// Newspaper 
    Newspaper,
    /// NineCircle 
    NineCircle,
    /// NineCircleFill 
    NineCircleFill,
    /// NineSquare 
    NineSquare,
    /// NineSquareFill 
    NineSquareFill,
    /// NintendoSwitch 
    NintendoSwitch,
    /// NodeMinus 
    NodeMinus,
    /// NodeMinusFill 
    NodeMinusFill,
    /// NodePlus 
    NodePlus,
    /// NodePlusFill 
    NodePlusFill,
    /// NoiseReduction 
    NoiseReduction,
    /// Nut 
    Nut,
    /// NutFill 
    NutFill,
    /// Nvidia 
    Nvidia,
    /// Nvme 
    Nvme,
    /// NvmeFill 
    NvmeFill,
    /// Octagon 
    Octagon,
    /// OctagonFill 
    OctagonFill,
    /// OctagonHalf 
    OctagonHalf,
    /// OneCircle 
    OneCircle,
    /// OneCircleFill 
    OneCircleFill,
    /// OneSquare 
    OneSquare,
    /// OneSquareFill 
    OneSquareFill,
    /// Onetwothree 
    Onetwothree,
    /// Opencollective 
    Opencollective,
    /// OpticalAudio 
    OpticalAudio,
    /// OpticalAudioFill 
    OpticalAudioFill,
    /// Option 
    Option,
    /// Outlet 
    Outlet,
    /// PCircle 
    PCircle,
    /// PCircleFill 
    PCircleFill,
    /// PSquare 
    PSquare,
    /// PSquareFill 
    PSquareFill,
    /// PaintBucket 
    PaintBucket,
    /// Palette 
    Palette,
    /// PaletteFill 
    PaletteFill,
    /// Palettetwo 
    Palettetwo,
    /// Paperclip 
    Paperclip,
    /// Paragraph 
    Paragraph,
    /// Pass 
    Pass,
    /// PassFill 
    PassFill,
    /// Passport 
    Passport,
    /// PassportFill 
    PassportFill,
    /// PatchCheck 
    PatchCheck,
    /// PatchCheckFill 
    PatchCheckFill,
    /// PatchExclamation 
    PatchExclamation,
    /// PatchExclamationFill 
    PatchExclamationFill,
    /// PatchMinus 
    PatchMinus,
    /// PatchMinusFill 
    PatchMinusFill,
    /// PatchPlus 
    PatchPlus,
    /// PatchPlusFill 
    PatchPlusFill,
    /// PatchQuestion 
    PatchQuestion,
    /// PatchQuestionFill 
    PatchQuestionFill,
    /// Pause 
    Pause,
    /// PauseBtn 
    PauseBtn,
    /// PauseBtnFill 
    PauseBtnFill,
    /// PauseCircle 
    PauseCircle,
    /// PauseCircleFill 
    PauseCircleFill,
    /// PauseFill 
    PauseFill,
    /// Paypal 
    Paypal,
    /// Pc 
    Pc,
    /// PcDisplay 
    PcDisplay,
    /// PcDisplayHorizontal 
    PcDisplayHorizontal,
    /// PcHorizontal 
    PcHorizontal,
    /// PciCard 
    PciCard,
    /// PciCardNetwork 
    PciCardNetwork,
    /// PciCardSound 
    PciCardSound,
    /// Peace 
    Peace,
    /// PeaceFill 
    PeaceFill,
    /// Pen 
    Pen,
    /// PenFill 
    PenFill,
    /// Pencil 
    Pencil,
    /// PencilFill 
    PencilFill,
    /// PencilSquare 
    PencilSquare,
    /// Pentagon 
    Pentagon,
    /// PentagonFill 
    PentagonFill,
    /// PentagonHalf 
    PentagonHalf,
    /// People 
    People,
    /// PeopleFill 
    PeopleFill,
    /// Percent 
    Percent,
    /// Person 
    Person,
    /// PersonAdd 
    PersonAdd,
    /// PersonArmsUp 
    PersonArmsUp,
    /// PersonBadge 
    PersonBadge,
    /// PersonBadgeFill 
    PersonBadgeFill,
    /// PersonBoundingBox 
    PersonBoundingBox,
    /// PersonCheck 
    PersonCheck,
    /// PersonCheckFill 
    PersonCheckFill,
    /// PersonCircle 
    PersonCircle,
    /// PersonDash 
    PersonDash,
    /// PersonDashFill 
    PersonDashFill,
    /// PersonDown 
    PersonDown,
    /// PersonExclamation 
    PersonExclamation,
    /// PersonFill 
    PersonFill,
    /// PersonFillAdd 
    PersonFillAdd,
    /// PersonFillCheck 
    PersonFillCheck,
    /// PersonFillDash 
    PersonFillDash,
    /// PersonFillDown 
    PersonFillDown,
    /// PersonFillExclamation 
    PersonFillExclamation,
    /// PersonFillGear 
    PersonFillGear,
    /// PersonFillLock 
    PersonFillLock,
    /// PersonFillSlash 
    PersonFillSlash,
    /// PersonFillUp 
    PersonFillUp,
    /// PersonFillX 
    PersonFillX,
    /// PersonGear 
    PersonGear,
    /// PersonHeart 
    PersonHeart,
    /// PersonHearts 
    PersonHearts,
    /// PersonLinesFill 
    PersonLinesFill,
    /// PersonLock 
    PersonLock,
    /// PersonPlus 
    PersonPlus,
    /// PersonPlusFill 
    PersonPlusFill,
    /// PersonRaisedHand 
    PersonRaisedHand,
    /// PersonRolodex 
    PersonRolodex,
    /// PersonSlash 
    PersonSlash,
    /// PersonSquare 
    PersonSquare,
    /// PersonStanding 
    PersonStanding,
    /// PersonStandingDress 
    PersonStandingDress,
    /// PersonUp 
    PersonUp,
    /// PersonVcard 
    PersonVcard,
    /// PersonVcardFill 
    PersonVcardFill,
    /// PersonVideo 
    PersonVideo,
    /// PersonVideothree 
    PersonVideothree,
    /// PersonVideotwo 
    PersonVideotwo,
    /// PersonWalking 
    PersonWalking,
    /// PersonWheelchair 
    PersonWheelchair,
    /// PersonWorkspace 
    PersonWorkspace,
    /// PersonX 
    PersonX,
    /// PersonXFill 
    PersonXFill,
    /// Phone 
    Phone,
    /// PhoneFill 
    PhoneFill,
    /// PhoneFlip 
    PhoneFlip,
    /// PhoneLandscape 
    PhoneLandscape,
    /// PhoneLandscapeFill 
    PhoneLandscapeFill,
    /// PhoneVibrate 
    PhoneVibrate,
    /// PhoneVibrateFill 
    PhoneVibrateFill,
    /// PieChart 
    PieChart,
    /// PieChartFill 
    PieChartFill,
    /// PiggyBank 
    PiggyBank,
    /// PiggyBankFill 
    PiggyBankFill,
    /// Pin 
    Pin,
    /// PinAngle 
    PinAngle,
    /// PinAngleFill 
    PinAngleFill,
    /// PinFill 
    PinFill,
    /// PinMap 
    PinMap,
    /// PinMapFill 
    PinMapFill,
    /// Pinterest 
    Pinterest,
    /// Pip 
    Pip,
    /// PipFill 
    PipFill,
    /// Play 
    Play,
    /// PlayBtn 
    PlayBtn,
    /// PlayBtnFill 
    PlayBtnFill,
    /// PlayCircle 
    PlayCircle,
    /// PlayCircleFill 
    PlayCircleFill,
    /// PlayFill 
    PlayFill,
    /// Playstation 
    Playstation,
    /// Plug 
    Plug,
    /// PlugFill 
    PlugFill,
    /// Plugin 
    Plugin,
    /// Plus 
    Plus,
    /// PlusCircle 
    PlusCircle,
    /// PlusCircleDotted 
    PlusCircleDotted,
    /// PlusCircleFill 
    PlusCircleFill,
    /// PlusLg 
    PlusLg,
    /// PlusSlashMinus 
    PlusSlashMinus,
    /// PlusSquare 
    PlusSquare,
    /// PlusSquareDotted 
    PlusSquareDotted,
    /// PlusSquareFill 
    PlusSquareFill,
    /// Postage 
    Postage,
    /// PostageFill 
    PostageFill,
    /// PostageHeart 
    PostageHeart,
    /// PostageHeartFill 
    PostageHeartFill,
    /// Postcard 
    Postcard,
    /// PostcardFill 
    PostcardFill,
    /// PostcardHeart 
    PostcardHeart,
    /// PostcardHeartFill 
    PostcardHeartFill,
    /// Power 
    Power,
    /// Prescription 
    Prescription,
    /// Prescriptiontwo 
    Prescriptiontwo,
    /// Printer 
    Printer,
    /// PrinterFill 
    PrinterFill,
    /// Projector 
    Projector,
    /// ProjectorFill 
    ProjectorFill,
    /// Puzzle 
    Puzzle,
    /// PuzzleFill 
    PuzzleFill,
    /// QrCode 
    QrCode,
    /// QrCodeScan 
    QrCodeScan,
    /// Question 
    Question,
    /// QuestionCircle 
    QuestionCircle,
    /// QuestionCircleFill 
    QuestionCircleFill,
    /// QuestionDiamond 
    QuestionDiamond,
    /// QuestionDiamondFill 
    QuestionDiamondFill,
    /// QuestionLg 
    QuestionLg,
    /// QuestionOctagon 
    QuestionOctagon,
    /// QuestionOctagonFill 
    QuestionOctagonFill,
    /// QuestionSquare 
    QuestionSquare,
    /// QuestionSquareFill 
    QuestionSquareFill,
    /// Quora 
    Quora,
    /// Quote 
    Quote,
    /// RCircle 
    RCircle,
    /// RCircleFill 
    RCircleFill,
    /// RSquare 
    RSquare,
    /// RSquareFill 
    RSquareFill,
    /// Radar 
    Radar,
    /// Radioactive 
    Radioactive,
    /// Rainbow 
    Rainbow,
    /// Receipt 
    Receipt,
    /// ReceiptCutoff 
    ReceiptCutoff,
    /// ReceptionFour 
    ReceptionFour,
    /// ReceptionOne 
    ReceptionOne,
    /// ReceptionThree 
    ReceptionThree,
    /// ReceptionTwo 
    ReceptionTwo,
    /// ReceptionZero 
    ReceptionZero,
    /// Record 
    Record,
    /// RecordBtn 
    RecordBtn,
    /// RecordBtnFill 
    RecordBtnFill,
    /// RecordCircle 
    RecordCircle,
    /// RecordCircleFill 
    RecordCircleFill,
    /// RecordFill 
    RecordFill,
    /// Recordtwo 
    Recordtwo,
    /// RecordtwoFill 
    RecordtwoFill,
    /// Recycle 
    Recycle,
    /// Reddit 
    Reddit,
    /// Regex 
    Regex,
    /// Repeat 
    Repeat,
    /// RepeatOne 
    RepeatOne,
    /// Reply 
    Reply,
    /// ReplyAll 
    ReplyAll,
    /// ReplyAllFill 
    ReplyAllFill,
    /// ReplyFill 
    ReplyFill,
    /// Rewind 
    Rewind,
    /// RewindBtn 
    RewindBtn,
    /// RewindBtnFill 
    RewindBtnFill,
    /// RewindCircle 
    RewindCircle,
    /// RewindCircleFill 
    RewindCircleFill,
    /// RewindFill 
    RewindFill,
    /// Robot 
    Robot,
    /// Rocket 
    Rocket,
    /// RocketFill 
    RocketFill,
    /// RocketTakeoff 
    RocketTakeoff,
    /// RocketTakeoffFill 
    RocketTakeoffFill,
    /// Router 
    Router,
    /// RouterFill 
    RouterFill,
    /// Rss 
    Rss,
    /// RssFill 
    RssFill,
    /// Rulers 
    Rulers,
    /// Safe 
    Safe,
    /// SafeFill 
    SafeFill,
    /// Safetwo 
    Safetwo,
    /// SafetwoFill 
    SafetwoFill,
    /// Save 
    Save,
    /// SaveFill 
    SaveFill,
    /// Savetwo 
    Savetwo,
    /// SavetwoFill 
    SavetwoFill,
    /// Scissors 
    Scissors,
    /// Scooter 
    Scooter,
    /// Screwdriver 
    Screwdriver,
    /// SdCard 
    SdCard,
    /// SdCardFill 
    SdCardFill,
    /// Search 
    Search,
    /// SearchHeart 
    SearchHeart,
    /// SearchHeartFill 
    SearchHeartFill,
    /// SegmentedNav 
    SegmentedNav,
    /// Send 
    Send,
    /// SendArrowDown 
    SendArrowDown,
    /// SendArrowDownFill 
    SendArrowDownFill,
    /// SendArrowUp 
    SendArrowUp,
    /// SendArrowUpFill 
    SendArrowUpFill,
    /// SendCheck 
    SendCheck,
    /// SendCheckFill 
    SendCheckFill,
    /// SendDash 
    SendDash,
    /// SendDashFill 
    SendDashFill,
    /// SendExclamation 
    SendExclamation,
    /// SendExclamationFill 
    SendExclamationFill,
    /// SendFill 
    SendFill,
    /// SendPlus 
    SendPlus,
    /// SendPlusFill 
    SendPlusFill,
    /// SendSlash 
    SendSlash,
    /// SendSlashFill 
    SendSlashFill,
    /// SendX 
    SendX,
    /// SendXFill 
    SendXFill,
    /// Server 
    Server,
    /// SevenCircle 
    SevenCircle,
    /// SevenCircleFill 
    SevenCircleFill,
    /// SevenSquare 
    SevenSquare,
    /// SevenSquareFill 
    SevenSquareFill,
    /// Shadows 
    Shadows,
    /// Share 
    Share,
    /// ShareFill 
    ShareFill,
    /// Shield 
    Shield,
    /// ShieldCheck 
    ShieldCheck,
    /// ShieldExclamation 
    ShieldExclamation,
    /// ShieldFill 
    ShieldFill,
    /// ShieldFillCheck 
    ShieldFillCheck,
    /// ShieldFillExclamation 
    ShieldFillExclamation,
    /// ShieldFillMinus 
    ShieldFillMinus,
    /// ShieldFillPlus 
    // ShieldFillPlus,
    /// ShieldFillX 
    ShieldFillX,
    /// ShieldLock 
    ShieldLock,
    /// ShieldLockFill 
    ShieldLockFill,
    /// ShieldMinus 
    ShieldMinus,
    /// ShieldPlus 
    ShieldPlus,
    /// ShieldShaded 
    ShieldShaded,
    /// ShieldSlash 
    ShieldSlash,
    /// ShieldSlashFill 
    ShieldSlashFill,
    /// ShieldX 
    ShieldX,
    /// Shift 
    Shift,
    /// ShiftFill 
    ShiftFill,
    /// Shop 
    Shop,
    /// ShopWindow 
    ShopWindow,
    /// Shuffle 
    Shuffle,
    /// SignDeadEnd 
    SignDeadEnd,
    /// SignDeadEndFill 
    SignDeadEndFill,
    /// SignDoNotEnter 
    SignDoNotEnter,
    /// SignDoNotEnterFill 
    SignDoNotEnterFill,
    /// SignIntersection 
    SignIntersection,
    /// SignIntersectionFill 
    SignIntersectionFill,
    /// SignIntersectionSide 
    SignIntersectionSide,
    /// SignIntersectionSideFill 
    SignIntersectionSideFill,
    /// SignIntersectionT 
    SignIntersectionT,
    /// SignIntersectionTFill 
    SignIntersectionTFill,
    /// SignIntersectionY 
    SignIntersectionY,
    /// SignIntersectionYFill 
    SignIntersectionYFill,
    /// SignMergeLeft 
    SignMergeLeft,
    /// SignMergeLeftFill 
    SignMergeLeftFill,
    /// SignMergeRight 
    SignMergeRight,
    /// SignMergeRightFill 
    SignMergeRightFill,
    /// SignNoLeftTurn 
    SignNoLeftTurn,
    /// SignNoLeftTurnFill 
    SignNoLeftTurnFill,
    /// SignNoParking 
    SignNoParking,
    /// SignNoParkingFill 
    SignNoParkingFill,
    /// SignNoRightTurn 
    SignNoRightTurn,
    /// SignNoRightTurnFill 
    SignNoRightTurnFill,
    /// SignRailroad 
    SignRailroad,
    /// SignRailroadFill 
    SignRailroadFill,
    /// SignStop 
    SignStop,
    /// SignStopFill 
    SignStopFill,
    /// SignStopLights 
    SignStopLights,
    /// SignStopLightsFill 
    SignStopLightsFill,
    /// SignTurnLeft 
    SignTurnLeft,
    /// SignTurnLeftFill 
    SignTurnLeftFill,
    /// SignTurnRight 
    SignTurnRight,
    /// SignTurnRightFill 
    SignTurnRightFill,
    /// SignTurnSlightLeft 
    SignTurnSlightLeft,
    /// SignTurnSlightLeftFill 
    SignTurnSlightLeftFill,
    /// SignTurnSlightRight 
    SignTurnSlightRight,
    /// SignTurnSlightRightFill 
    SignTurnSlightRightFill,
    /// SignYield 
    SignYield,
    /// SignYieldFill 
    SignYieldFill,
    /// Signal 
    Signal,
    /// Signpost 
    Signpost,
    /// SignpostFill 
    SignpostFill,
    /// SignpostSplit 
    SignpostSplit,
    /// SignpostSplitFill 
    SignpostSplitFill,
    /// SignpostTwo 
    SignpostTwo,
    /// SignpostTwoFill 
    SignpostTwoFill,
    /// Sim 
    Sim,
    /// SimFill 
    SimFill,
    /// SimSlash 
    SimSlash,
    /// SimSlashFill 
    SimSlashFill,
    /// SinaWeibo 
    SinaWeibo,
    /// SixCircle 
    SixCircle,
    /// SixCircleFill 
    SixCircleFill,
    /// SixSquare 
    SixSquare,
    /// SixSquareFill 
    SixSquareFill,
    /// SkipBackward 
    SkipBackward,
    /// SkipBackwardBtn 
    SkipBackwardBtn,
    /// SkipBackwardBtnFill 
    SkipBackwardBtnFill,
    /// SkipBackwardCircle 
    SkipBackwardCircle,
    /// SkipBackwardCircleFill 
    SkipBackwardCircleFill,
    /// SkipBackwardFill 
    SkipBackwardFill,
    /// SkipEnd 
    SkipEnd,
    /// SkipEndBtn 
    SkipEndBtn,
    /// SkipEndBtnFill 
    SkipEndBtnFill,
    /// SkipEndCircle 
    SkipEndCircle,
    /// SkipEndCircleFill 
    SkipEndCircleFill,
    /// SkipEndFill 
    SkipEndFill,
    /// SkipForward 
    SkipForward,
    /// SkipForwardBtn 
    SkipForwardBtn,
    /// SkipForwardBtnFill 
    SkipForwardBtnFill,
    /// SkipForwardCircle 
    SkipForwardCircle,
    /// SkipForwardCircleFill 
    SkipForwardCircleFill,
    /// SkipForwardFill 
    SkipForwardFill,
    /// SkipStart 
    SkipStart,
    /// SkipStartBtn 
    SkipStartBtn,
    /// SkipStartBtnFill 
    SkipStartBtnFill,
    /// SkipStartCircle 
    SkipStartCircle,
    /// SkipStartCircleFill 
    SkipStartCircleFill,
    /// SkipStartFill 
    SkipStartFill,
    /// Skype 
    Skype,
    /// Slack 
    Slack,
    /// Slash 
    Slash,
    /// SlashCircle 
    SlashCircle,
    /// SlashCircleFill 
    SlashCircleFill,
    /// SlashLg 
    SlashLg,
    /// SlashSquare 
    SlashSquare,
    /// SlashSquareFill 
    SlashSquareFill,
    /// Sliders 
    Sliders,
    /// Sliderstwo 
    Sliderstwo,
    /// SliderstwoVertical 
    SliderstwoVertical,
    /// Smartwatch 
    Smartwatch,
    /// Snapchat 
    Snapchat,
    /// Snow 
    Snow,
    /// Snowthree 
    Snowthree,
    /// Snowtwo 
    Snowtwo,
    /// SortAlphaDown 
    SortAlphaDown,
    /// SortAlphaDownAlt 
    SortAlphaDownAlt,
    /// SortAlphaUp 
    SortAlphaUp,
    /// SortAlphaUpAlt 
    SortAlphaUpAlt,
    /// SortDown 
    SortDown,
    /// SortDownAlt 
    SortDownAlt,
    /// SortNumericDown 
    SortNumericDown,
    /// SortNumericDownAlt 
    SortNumericDownAlt,
    /// SortNumericUp 
    SortNumericUp,
    /// SortNumericUpAlt 
    SortNumericUpAlt,
    /// SortUp 
    SortUp,
    /// SortUpAlt 
    SortUpAlt,
    /// Soundwave 
    Soundwave,
    /// Sourceforge 
    Sourceforge,
    /// Speaker 
    Speaker,
    /// SpeakerFill 
    SpeakerFill,
    /// Speedometer 
    Speedometer,
    /// Speedometertwo 
    Speedometertwo,
    /// Spellcheck 
    Spellcheck,
    /// Spotify 
    Spotify,
    /// Square 
    Square,
    /// SquareFill 
    SquareFill,
    /// SquareHalf 
    SquareHalf,
    /// Stack 
    Stack,
    /// StackOverflow 
    StackOverflow,
    /// Star 
    Star,
    /// StarFill 
    StarFill,
    /// StarHalf 
    StarHalf,
    /// Stars 
    Stars,
    /// Steam 
    Steam,
    /// Stickies 
    Stickies,
    /// StickiesFill 
    StickiesFill,
    /// Sticky 
    Sticky,
    /// StickyFill 
    StickyFill,
    /// Stop 
    Stop,
    /// StopBtn 
    StopBtn,
    /// StopBtnFill 
    StopBtnFill,
    /// StopCircle 
    StopCircle,
    /// StopCircleFill 
    StopCircleFill,
    /// StopFill 
    StopFill,
    /// Stoplights 
    Stoplights,
    /// StoplightsFill 
    StoplightsFill,
    /// Stopwatch 
    Stopwatch,
    /// StopwatchFill 
    StopwatchFill,
    /// Strava 
    Strava,
    /// Stripe 
    Stripe,
    /// Subscript 
    Subscript,
    /// Substack 
    Substack,
    /// SuitClub 
    SuitClub,
    /// SuitClubFill 
    SuitClubFill,
    /// SuitDiamond 
    SuitDiamond,
    /// SuitDiamondFill 
    SuitDiamondFill,
    /// SuitHeart 
    SuitHeart,
    /// SuitHeartFill 
    SuitHeartFill,
    /// SuitSpade 
    SuitSpade,
    /// SuitSpadeFill 
    SuitSpadeFill,
    /// Suitcase 豈
    Suitcase,
    /// SuitcaseFill 
    SuitcaseFill,
    /// SuitcaseLg 
    SuitcaseLg,
    /// SuitcaseLgFill 
    SuitcaseLgFill,
    /// Suitcasetwo 車
    Suitcasetwo,
    /// SuitcasetwoFill 更
    SuitcasetwoFill,
    /// SunFill 
    SunFill,
    /// Sunglasses 
    Sunglasses,
    /// Sunrise 
    Sunrise,
    /// SunriseFill 
    SunriseFill,
    /// Sunset 
    Sunset,
    /// SunsetFill 
    SunsetFill,
    /// Superscript 
    Superscript,
    /// SymmetryHorizontal 
    SymmetryHorizontal,
    /// SymmetryVertical 
    SymmetryVertical,
    /// Table 
    Table,
    /// Tablet 
    Tablet,
    /// TabletFill 
    TabletFill,
    /// TabletLandscape 
    TabletLandscape,
    /// TabletLandscapeFill 
    TabletLandscapeFill,
    /// Tag 
    Tag,
    /// TagFill 
    TagFill,
    /// Tags 
    Tags,
    /// TagsFill 
    TagsFill,
    /// TaxiFront 
    TaxiFront,
    /// TaxiFrontFill 
    TaxiFrontFill,
    /// Telegram 
    Telegram,
    /// Telephone 
    Telephone,
    /// TelephoneFill 
    TelephoneFill,
    /// TelephoneForward 
    TelephoneForward,
    /// TelephoneForwardFill 
    TelephoneForwardFill,
    /// TelephoneInbound 
    TelephoneInbound,
    /// TelephoneInboundFill 
    TelephoneInboundFill,
    /// TelephoneMinus 
    TelephoneMinus,
    /// TelephoneMinusFill 
    TelephoneMinusFill,
    /// TelephoneOutbound 
    TelephoneOutbound,
    /// TelephoneOutboundFill 
    TelephoneOutboundFill,
    /// TelephonePlus 
    TelephonePlus,
    /// TelephonePlusFill 
    TelephonePlusFill,
    /// TelephoneX 
    TelephoneX,
    /// TelephoneXFill 
    TelephoneXFill,
    /// TencentQq 
    TencentQq,
    /// Terminal 
    Terminal,
    /// TerminalDash 
    TerminalDash,
    /// TerminalFill 
    TerminalFill,
    /// TerminalPlus 
    TerminalPlus,
    /// TerminalSplit 
    TerminalSplit,
    /// TerminalX 
    TerminalX,
    /// TextCenter 
    TextCenter,
    /// TextIndentLeft 
    TextIndentLeft,
    /// TextIndentRight 
    TextIndentRight,
    /// TextLeft 
    TextLeft,
    /// TextParagraph 
    TextParagraph,
    /// TextRight 
    TextRight,
    /// TextWrap 
    TextWrap,
    /// Textarea 
    Textarea,
    /// TextareaResize 
    TextareaResize,
    /// TextareaT 
    TextareaT,
    /// Thermometer 
    Thermometer,
    /// ThermometerHalf 
    ThermometerHalf,
    /// ThermometerHigh 
    ThermometerHigh,
    /// ThermometerLow 
    ThermometerLow,
    /// ThermometerSnow 
    ThermometerSnow,
    /// ThermometerSun 
    ThermometerSun,
    /// Threads 
    Threads,
    /// ThreadsFill 
    ThreadsFill,
    /// ThreeCircle 
    ThreeCircle,
    /// ThreeCircleFill 
    ThreeCircleFill,
    /// ThreeDots 
    ThreeDots,
    /// ThreeDotsVertical 
    ThreeDotsVertical,
    /// ThreeSquare 
    ThreeSquare,
    /// ThreeSquareFill 
    ThreeSquareFill,
    /// Thunderbolt 
    Thunderbolt,
    /// ThunderboltFill 
    ThunderboltFill,
    /// Ticket 
    Ticket,
    /// TicketDetailed 
    TicketDetailed,
    /// TicketDetailedFill 
    TicketDetailedFill,
    /// TicketFill 
    TicketFill,
    /// TicketPerforated 
    TicketPerforated,
    /// TicketPerforatedFill 
    TicketPerforatedFill,
    /// Tiktok 
    Tiktok,
    /// ToggleOff 
    ToggleOff,
    /// ToggleOn 
    ToggleOn,
    /// Toggles 
    Toggles,
    /// Togglestwo 
    Togglestwo,
    /// ToggletwoOff 
    ToggletwoOff,
    /// ToggletwoOn 
    ToggletwoOn,
    /// Tools 
    Tools,
    /// Tornado 
    Tornado,
    /// TrainFreightFront 
    TrainFreightFront,
    /// TrainFreightFrontFill 
    TrainFreightFrontFill,
    /// TrainFront 
    TrainFront,
    /// TrainFrontFill 
    TrainFrontFill,
    /// TrainLightrailFront 
    TrainLightrailFront,
    /// TrainLightrailFrontFill 
    TrainLightrailFrontFill,
    /// Translate 
    Translate,
    /// Transparency 
    Transparency,
    /// Trash 
    Trash,
    /// TrashFill 
    TrashFill,
    /// Trashthree 
    Trashthree,
    /// TrashthreeFill 
    TrashthreeFill,
    /// Trashtwo 
    Trashtwo,
    /// TrashtwoFill 
    TrashtwoFill,
    /// Tree 
    Tree,
    /// TreeFill 
    TreeFill,
    /// Trello 
    Trello,
    /// Triangle 
    Triangle,
    /// TriangleFill 
    TriangleFill,
    /// TriangleHalf 
    TriangleHalf,
    /// Trophy 
    Trophy,
    /// TrophyFill 
    TrophyFill,
    /// TropicalStorm 
    TropicalStorm,
    /// Truck 
    Truck,
    /// TruckFlatbed 
    TruckFlatbed,
    /// TruckFront 
    TruckFront,
    /// TruckFrontFill 
    TruckFrontFill,
    /// Tsunami 
    Tsunami,
    /// Tv 
    Tv,
    /// TvFill 
    TvFill,
    /// Twitch 
    Twitch,
    /// Twitter 
    Twitter,
    /// TwitterX 
    TwitterX,
    /// TwoCircle 
    TwoCircle,
    /// TwoCircleFill 
    TwoCircleFill,
    /// TwoSquare 
    TwoSquare,
    /// TwoSquareFill 
    TwoSquareFill,
    /// Type 
    Type,
    /// TypeBold 
    TypeBold,
    /// TypeHfive 
    TypeHfive,
    /// TypeHfour 
    TypeHfour,
    /// TypeHone 
    TypeHone,
    /// TypeHsix 
    TypeHsix,
    /// TypeHthree 
    TypeHthree,
    /// TypeHtwo 
    TypeHtwo,
    /// TypeItalic 
    TypeItalic,
    /// TypeStrikethrough 
    TypeStrikethrough,
    /// TypeUnderline 
    TypeUnderline,
    /// Ubuntu 
    Ubuntu,
    /// UiChecks 
    UiChecks,
    /// UiChecksGrid 
    UiChecksGrid,
    /// UiRadios 
    UiRadios,
    /// UiRadiosGrid 
    UiRadiosGrid,
    /// Umbrella 
    Umbrella,
    /// UmbrellaFill 
    UmbrellaFill,
    /// Unindent 
    Unindent,
    /// Union 
    Union,
    /// Unity 
    Unity,
    /// UniversalAccess 
    UniversalAccess,
    /// UniversalAccessCircle 
    UniversalAccessCircle,
    /// Unlock 
    Unlock,
    /// UnlockFill 
    UnlockFill,
    /// Upc 
    Upc,
    /// UpcScan 
    UpcScan,
    /// Upload 
    Upload,
    /// Usb 
    Usb,
    /// UsbC 
    UsbC,
    /// UsbCFill 
    UsbCFill,
    /// UsbDrive 
    UsbDrive,
    /// UsbDriveFill 
    UsbDriveFill,
    /// UsbFill 
    UsbFill,
    /// UsbMicro 
    UsbMicro,
    /// UsbMicroFill 
    UsbMicroFill,
    /// UsbMini 
    UsbMini,
    /// UsbMiniFill 
    UsbMiniFill,
    /// UsbPlug 
    UsbPlug,
    /// UsbPlugFill 
    UsbPlugFill,
    /// UsbSymbol 
    UsbSymbol,
    /// Valentine 
    Valentine,
    /// Valentinetwo 
    Valentinetwo,
    /// VectorPen 
    VectorPen,
    /// ViewList 
    ViewList,
    /// ViewStacked 
    ViewStacked,
    /// Vignette 賈
    Vignette,
    /// Vimeo 
    Vimeo,
    /// Vinyl 
    Vinyl,
    /// VinylFill 
    VinylFill,
    /// Virus 
    Virus,
    /// Virustwo 
    Virustwo,
    /// Voicemail 
    Voicemail,
    /// VolumeDown 
    VolumeDown,
    /// VolumeDownFill 
    VolumeDownFill,
    /// VolumeMute 
    VolumeMute,
    /// VolumeMuteFill 
    VolumeMuteFill,
    /// VolumeOff 
    VolumeOff,
    /// VolumeOffFill 
    VolumeOffFill,
    /// VolumeUp 
    VolumeUp,
    /// VolumeUpFill 
    VolumeUpFill,
    /// Vr 
    Vr,
    /// Wallet 
    Wallet,
    /// WalletFill 
    WalletFill,
    /// Wallettwo 
    Wallettwo,
    /// Watch 
    Watch,
    /// Water 
    Water,
    /// Webcam 
    Webcam,
    /// WebcamFill 
    WebcamFill,
    /// Wechat 
    Wechat,
    /// Whatsapp 
    Whatsapp,
    /// Wifi 
    Wifi,
    /// WifiOff 
    WifiOff,
    /// WifiOne 
    WifiOne,
    /// WifiTwo 
    WifiTwo,
    /// Wikipedia 
    Wikipedia,
    /// Wind 
    Wind,
    /// Window 
    Window,
    /// WindowDash 
    WindowDash,
    /// WindowDesktop 
    WindowDesktop,
    /// WindowDock 
    WindowDock,
    /// WindowFullscreen 
    WindowFullscreen,
    /// WindowPlus 
    WindowPlus,
    /// WindowSidebar 
    WindowSidebar,
    /// WindowSplit 
    WindowSplit,
    /// WindowStack 
    WindowStack,
    /// WindowX 
    WindowX,
    /// Windows 
    Windows,
    /// Wordpress 
    Wordpress,
    /// Wrench 
    Wrench,
    /// WrenchAdjustable 
    WrenchAdjustable,
    /// WrenchAdjustableCircle 
    WrenchAdjustableCircle,
    /// WrenchAdjustableCircleFill 
    WrenchAdjustableCircleFill,
    /// X 
    X,
    /// XCircle 
    XCircle,
    /// XCircleFill 
    XCircleFill,
    /// XDiamond 
    XDiamond,
    /// XDiamondFill 
    XDiamondFill,
    /// XLg 
    XLg,
    /// XOctagon 
    XOctagon,
    /// XOctagonFill 
    XOctagonFill,
    /// XSquare 
    XSquare,
    /// XSquareFill 
    XSquareFill,
    /// Xbox 
    Xbox,
    /// Yelp 
    Yelp,
    /// YinYang 
    YinYang,
    /// Youtube 
    Youtube,
    /// ZeroCircle 
    ZeroCircle,
    /// ZeroCircleFill 
    ZeroCircleFill,
    /// ZeroSquare 
    ZeroSquare,
    /// ZeroSquareFill 
    ZeroSquareFill,
    /// ZoomIn 
    ZoomIn,
    /// ZoomOut 
    ZoomOut,
}

/// Converts an [`Bootstrap`] into a [`char`]
#[must_use]
pub const fn icon_to_char(icon: Bootstrap) -> char {
    match icon {
        Bootstrap::Activity => '\u{f66b}',
        Bootstrap::Airplane => '\u{f7cd}',
        Bootstrap::AirplaneEngines => '\u{f7cb}',
        Bootstrap::AirplaneEnginesFill => '\u{f7ca}',
        Bootstrap::AirplaneFill => '\u{f7cc}',
        Bootstrap::Alarm => '\u{f102}',
        Bootstrap::AlarmFill => '\u{f101}',
        Bootstrap::Alexa => '\u{f7ce}',
        Bootstrap::AlignBottom => '\u{f103}',
        Bootstrap::AlignCenter => '\u{f104}',
        Bootstrap::AlignEnd => '\u{f105}',
        Bootstrap::AlignMiddle => '\u{f106}',
        Bootstrap::AlignStart => '\u{f107}',
        Bootstrap::AlignTop => '\u{f108}',
        Bootstrap::Alipay => '\u{f7cf}',
        Bootstrap::Alphabet => '\u{f68a}',
        Bootstrap::AlphabetUppercase => '\u{f2a5}',
        Bootstrap::Alt => '\u{f109}',
        Bootstrap::Amazon => '\u{f68d}',
        Bootstrap::Amd => '\u{f8ae}',
        Bootstrap::Android => '\u{f7d0}',
        Bootstrap::Androidtwo => '\u{f7d1}',
        Bootstrap::App => '\u{f10b}',
        Bootstrap::AppIndicator => '\u{f10a}',
        Bootstrap::Apple => '\u{f65b}',
        Bootstrap::Archive => '\u{f10d}',
        Bootstrap::ArchiveFill => '\u{f10c}',
        Bootstrap::ArrowBarDown => '\u{f112}',
        Bootstrap::ArrowBarLeft => '\u{f113}',
        Bootstrap::ArrowBarRight => '\u{f114}',
        Bootstrap::ArrowBarUp => '\u{f115}',
        Bootstrap::ArrowClockwise => '\u{f116}',
        Bootstrap::ArrowCounterclockwise => '\u{f117}',
        Bootstrap::ArrowDown => '\u{f128}',
        Bootstrap::ArrowDownCircle => '\u{f119}',
        Bootstrap::ArrowDownCircleFill => '\u{f118}',
        Bootstrap::ArrowDownLeft => '\u{f11e}',
        Bootstrap::ArrowDownLeftCircle => '\u{f11b}',
        Bootstrap::ArrowDownLeftCircleFill => '\u{f11a}',
        Bootstrap::ArrowDownLeftSquare => '\u{f11d}',
        Bootstrap::ArrowDownLeftSquareFill => '\u{f11c}',
        Bootstrap::ArrowDownRight => '\u{f123}',
        Bootstrap::ArrowDownRightCircle => '\u{f120}',
        Bootstrap::ArrowDownRightCircleFill => '\u{f11f}',
        Bootstrap::ArrowDownRightSquare => '\u{f122}',
        Bootstrap::ArrowDownRightSquareFill => '\u{f121}',
        Bootstrap::ArrowDownShort => '\u{f124}',
        Bootstrap::ArrowDownSquare => '\u{f126}',
        Bootstrap::ArrowDownSquareFill => '\u{f125}',
        Bootstrap::ArrowDownUp => '\u{f127}',
        Bootstrap::ArrowLeft => '\u{f12f}',
        Bootstrap::ArrowLeftCircle => '\u{f12a}',
        Bootstrap::ArrowLeftCircleFill => '\u{f129}',
        Bootstrap::ArrowLeftRight => '\u{f12b}',
        Bootstrap::ArrowLeftShort => '\u{f12c}',
        Bootstrap::ArrowLeftSquare => '\u{f12e}',
        Bootstrap::ArrowLeftSquareFill => '\u{f12d}',
        Bootstrap::ArrowNinezerodegDown => '\u{f10e}',
        Bootstrap::ArrowNinezerodegLeft => '\u{f10f}',
        Bootstrap::ArrowNinezerodegRight => '\u{f110}',
        Bootstrap::ArrowNinezerodegUp => '\u{f111}',
        Bootstrap::ArrowRepeat => '\u{f130}',
        Bootstrap::ArrowReturnLeft => '\u{f131}',
        Bootstrap::ArrowReturnRight => '\u{f132}',
        Bootstrap::ArrowRight => '\u{f138}',
        Bootstrap::ArrowRightCircle => '\u{f134}',
        Bootstrap::ArrowRightCircleFill => '\u{f133}',
        Bootstrap::ArrowRightShort => '\u{f135}',
        Bootstrap::ArrowRightSquare => '\u{f137}',
        Bootstrap::ArrowRightSquareFill => '\u{f136}',
        Bootstrap::ArrowThroughHeart => '\u{f701}',
        Bootstrap::ArrowThroughHeartFill => '\u{f700}',
        Bootstrap::ArrowUp => '\u{f148}',
        Bootstrap::ArrowUpCircle => '\u{f13a}',
        Bootstrap::ArrowUpCircleFill => '\u{f139}',
        Bootstrap::ArrowUpLeft => '\u{f13f}',
        Bootstrap::ArrowUpLeftCircle => '\u{f13c}',
        Bootstrap::ArrowUpLeftCircleFill => '\u{f13b}',
        Bootstrap::ArrowUpLeftSquare => '\u{f13e}',
        Bootstrap::ArrowUpLeftSquareFill => '\u{f13d}',
        Bootstrap::ArrowUpRight => '\u{f144}',
        Bootstrap::ArrowUpRightCircle => '\u{f141}',
        Bootstrap::ArrowUpRightCircleFill => '\u{f140}',
        Bootstrap::ArrowUpRightSquare => '\u{f143}',
        Bootstrap::ArrowUpRightSquareFill => '\u{f142}',
        Bootstrap::ArrowUpShort => '\u{f145}',
        Bootstrap::ArrowUpSquare => '\u{f147}',
        Bootstrap::ArrowUpSquareFill => '\u{f146}',
        Bootstrap::Arrows => '\u{f6a2}',
        Bootstrap::ArrowsAngleContract => '\u{f149}',
        Bootstrap::ArrowsAngleExpand => '\u{f14a}',
        Bootstrap::ArrowsCollapse => '\u{f14b}',
        Bootstrap::ArrowsCollapseVertical => '\u{f690}',
        Bootstrap::ArrowsExpand => '\u{f14c}',
        Bootstrap::ArrowsExpandVertical => '\u{f695}',
        Bootstrap::ArrowsFullscreen => '\u{f14d}',
        Bootstrap::ArrowsMove => '\u{f14e}',
        Bootstrap::ArrowsVertical => '\u{f698}',
        Bootstrap::AspectRatio => '\u{f150}',
        Bootstrap::AspectRatioFill => '\u{f14f}',
        Bootstrap::Asterisk => '\u{f151}',
        Bootstrap::At => '\u{f152}',
        Bootstrap::Award => '\u{f154}',
        Bootstrap::AwardFill => '\u{f153}',
        Bootstrap::Back => '\u{f598}',
        Bootstrap::Backpack => '\u{f8e0}',
        Bootstrap::BackpackFill => '\u{f8df}',
        Bootstrap::Backpackfour => '\u{f8e6}',
        Bootstrap::BackpackfourFill => '\u{f8e5}',
        Bootstrap::Backpackthree => '\u{f8e4}',
        Bootstrap::BackpackthreeFill => '\u{f8e3}',
        Bootstrap::Backpacktwo => '\u{f8e2}',
        Bootstrap::BackpacktwoFill => '\u{f8e1}',
        Bootstrap::Backspace => '\u{f159}',
        Bootstrap::BackspaceFill => '\u{f156}',
        Bootstrap::BackspaceReverse => '\u{f158}',
        Bootstrap::BackspaceReverseFill => '\u{f157}',
        Bootstrap::BadgeAd => '\u{f161}',
        Bootstrap::BadgeAdFill => '\u{f160}',
        Bootstrap::BadgeAr => '\u{f163}',
        Bootstrap::BadgeArFill => '\u{f162}',
        Bootstrap::BadgeCc => '\u{f165}',
        Bootstrap::BadgeCcFill => '\u{f164}',
        Bootstrap::BadgeEightk => '\u{f15f}',
        Bootstrap::BadgeEightkFill => '\u{f15e}',
        Bootstrap::BadgeFourk => '\u{f15d}',
        Bootstrap::BadgeFourkFill => '\u{f15c}',
        Bootstrap::BadgeHd => '\u{f167}',
        Bootstrap::BadgeHdFill => '\u{f166}',
        Bootstrap::BadgeSd => '\u{f703}',
        Bootstrap::BadgeSdFill => '\u{f702}',
        Bootstrap::BadgeThreed => '\u{f15b}',
        Bootstrap::BadgeThreedFill => '\u{f15a}',
        Bootstrap::BadgeTm => '\u{f169}',
        Bootstrap::BadgeTmFill => '\u{f168}',
        Bootstrap::BadgeVo => '\u{f16b}',
        Bootstrap::BadgeVoFill => '\u{f16a}',
        Bootstrap::BadgeVr => '\u{f16d}',
        Bootstrap::BadgeVrFill => '\u{f16c}',
        Bootstrap::BadgeWc => '\u{f16f}',
        Bootstrap::BadgeWcFill => '\u{f16e}',
        Bootstrap::Bag => '\u{f179}',
        Bootstrap::BagCheck => '\u{f171}',
        Bootstrap::BagCheckFill => '\u{f170}',
        Bootstrap::BagDash => '\u{f173}',
        Bootstrap::BagDashFill => '\u{f172}',
        Bootstrap::BagFill => '\u{f174}',
        Bootstrap::BagHeart => '\u{f705}',
        Bootstrap::BagHeartFill => '\u{f704}',
        Bootstrap::BagPlus => '\u{f176}',
        Bootstrap::BagPlusFill => '\u{f175}',
        Bootstrap::BagX => '\u{f178}',
        Bootstrap::BagXFill => '\u{f177}',
        Bootstrap::Balloon => '\u{f709}',
        Bootstrap::BalloonFill => '\u{f706}',
        Bootstrap::BalloonHeart => '\u{f708}',
        Bootstrap::BalloonHeartFill => '\u{f707}',
        Bootstrap::Ban => '\u{f6b6}',
        Bootstrap::BanFill => '\u{f6a3}',
        Bootstrap::Bandaid => '\u{f681}',
        Bootstrap::BandaidFill => '\u{f680}',
        Bootstrap::Bank => '\u{f62e}',
        Bootstrap::Banktwo => '\u{f62f}',
        Bootstrap::BarChart => '\u{f17e}',
        Bootstrap::BarChartFill => '\u{f17a}',
        Bootstrap::BarChartLine => '\u{f17c}',
        Bootstrap::BarChartLineFill => '\u{f17b}',
        Bootstrap::BarChartSteps => '\u{f17d}',
        Bootstrap::Basket => '\u{f180}',
        Bootstrap::BasketFill => '\u{f17f}',
        Bootstrap::Basketthree => '\u{f184}',
        Bootstrap::BasketthreeFill => '\u{f183}',
        Bootstrap::Baskettwo => '\u{f182}',
        Bootstrap::BaskettwoFill => '\u{f181}',
        Bootstrap::Battery => '\u{f188}',
        Bootstrap::BatteryCharging => '\u{f185}',
        Bootstrap::BatteryFull => '\u{f186}',
        Bootstrap::BatteryHalf => '\u{f187}',
        Bootstrap::Behance => '\u{f65c}',
        Bootstrap::Bell => '\u{f18a}',
        Bootstrap::BellFill => '\u{f189}',
        Bootstrap::BellSlash => '\u{f631}',
        Bootstrap::BellSlashFill => '\u{f630}',
        Bootstrap::Bezier => '\u{f18b}',
        Bootstrap::Beziertwo => '\u{f18c}',
        Bootstrap::Bicycle => '\u{f18d}',
        Bootstrap::Bing => '\u{f6c2}',
        Bootstrap::Binoculars => '\u{f18f}',
        Bootstrap::BinocularsFill => '\u{f18e}',
        Bootstrap::BlockquoteLeft => '\u{f190}',
        Bootstrap::BlockquoteRight => '\u{f191}',
        Bootstrap::Bluetooth => '\u{f682}',
        Bootstrap::BodyText => '\u{f683}',
        Bootstrap::Book => '\u{f194}',
        Bootstrap::BookFill => '\u{f192}',
        Bootstrap::BookHalf => '\u{f193}',
        Bootstrap::Bookmark => '\u{f1a2}',
        Bootstrap::BookmarkCheck => '\u{f196}',
        Bootstrap::BookmarkCheckFill => '\u{f195}',
        Bootstrap::BookmarkDash => '\u{f198}',
        Bootstrap::BookmarkDashFill => '\u{f197}',
        Bootstrap::BookmarkFill => '\u{f199}',
        Bootstrap::BookmarkHeart => '\u{f19b}',
        Bootstrap::BookmarkHeartFill => '\u{f19a}',
        Bootstrap::BookmarkPlus => '\u{f19d}',
        Bootstrap::BookmarkPlusFill => '\u{f19c}',
        Bootstrap::BookmarkStar => '\u{f19f}',
        Bootstrap::BookmarkStarFill => '\u{f19e}',
        Bootstrap::BookmarkX => '\u{f1a1}',
        Bootstrap::BookmarkXFill => '\u{f1a0}',
        Bootstrap::Bookmarks => '\u{f1a4}',
        Bootstrap::BookmarksFill => '\u{f1a3}',
        Bootstrap::Bookshelf => '\u{f1a5}',
        Bootstrap::Boombox => '\u{f684}',
        Bootstrap::BoomboxFill => '\u{f6df}',
        Bootstrap::Bootstrap => '\u{f1a8}',
        Bootstrap::BootstrapFill => '\u{f1a6}',
        Bootstrap::BootstrapReboot => '\u{f1a7}',
        Bootstrap::Border => '\u{f1b4}',
        Bootstrap::BorderAll => '\u{f1a9}',
        Bootstrap::BorderBottom => '\u{f1aa}',
        Bootstrap::BorderCenter => '\u{f1ab}',
        Bootstrap::BorderInner => '\u{f1ac}',
        Bootstrap::BorderLeft => '\u{f1ad}',
        Bootstrap::BorderMiddle => '\u{f1ae}',
        Bootstrap::BorderOuter => '\u{f1af}',
        Bootstrap::BorderRight => '\u{f1b0}',
        Bootstrap::BorderStyle => '\u{f1b1}',
        Bootstrap::BorderTop => '\u{f1b2}',
        Bootstrap::BorderWidth => '\u{f1b3}',
        Bootstrap::BoundingBox => '\u{f1b6}',
        Bootstrap::BoundingBoxCircles => '\u{f1b5}',
        Bootstrap::Box => '\u{f1c8}',
        Bootstrap::BoxArrowDown => '\u{f1b9}',
        Bootstrap::BoxArrowDownLeft => '\u{f1b7}',
        Bootstrap::BoxArrowDownRight => '\u{f1b8}',
        Bootstrap::BoxArrowInDown => '\u{f1bc}',
        Bootstrap::BoxArrowInDownLeft => '\u{f1ba}',
        Bootstrap::BoxArrowInDownRight => '\u{f1bb}',
        Bootstrap::BoxArrowInLeft => '\u{f1bd}',
        Bootstrap::BoxArrowInRight => '\u{f1be}',
        Bootstrap::BoxArrowInUp => '\u{f1c1}',
        Bootstrap::BoxArrowInUpLeft => '\u{f1bf}',
        Bootstrap::BoxArrowInUpRight => '\u{f1c0}',
        Bootstrap::BoxArrowLeft => '\u{f1c2}',
        Bootstrap::BoxArrowRight => '\u{f1c3}',
        Bootstrap::BoxArrowUp => '\u{f1c6}',
        Bootstrap::BoxArrowUpLeft => '\u{f1c4}',
        Bootstrap::BoxArrowUpRight => '\u{f1c5}',
        Bootstrap::BoxFill => '\u{f7d2}',
        Bootstrap::BoxSeam => '\u{f1c7}',
        Bootstrap::BoxSeamFill => '\u{f7d3}',
        Bootstrap::Boxes => '\u{f685}',
        Bootstrap::Boxtwo => '\u{f70d}',
        Bootstrap::BoxtwoFill => '\u{f70a}',
        Bootstrap::BoxtwoHeart => '\u{f70c}',
        Bootstrap::BoxtwoHeartFill => '\u{f70b}',
        Bootstrap::Braces => '\u{f1c9}',
        Bootstrap::BracesAsterisk => '\u{f70e}',
        Bootstrap::Bricks => '\u{f1ca}',
        Bootstrap::Briefcase => '\u{f1cc}',
        Bootstrap::BriefcaseFill => '\u{f1cb}',
        Bootstrap::BrightnessAltHigh => '\u{f1ce}',
        Bootstrap::BrightnessAltHighFill => '\u{f1cd}',
        Bootstrap::BrightnessAltLow => '\u{f1d0}',
        Bootstrap::BrightnessAltLowFill => '\u{f1cf}',
        Bootstrap::BrightnessHigh => '\u{f5a2}',
        Bootstrap::BrightnessHighFill => '\u{f1d1}',
        Bootstrap::BrightnessLow => '\u{f1d4}',
        Bootstrap::BrightnessLowFill => '\u{f1d3}',
        Bootstrap::Brilliance => '\u{f8e7}',
        Bootstrap::Broadcast => '\u{f1d6}',
        Bootstrap::BroadcastPin => '\u{f1d5}',
        Bootstrap::BrowserChrome => '\u{f7d4}',
        Bootstrap::BrowserEdge => '\u{f7d5}',
        Bootstrap::BrowserFirefox => '\u{f7d6}',
        Bootstrap::BrowserSafari => '\u{f7d7}',
        Bootstrap::Brush => '\u{f1d8}',
        Bootstrap::BrushFill => '\u{f1d7}',
        Bootstrap::Bucket => '\u{f1da}',
        Bootstrap::BucketFill => '\u{f1d9}',
        Bootstrap::Bug => '\u{f1dc}',
        Bootstrap::BugFill => '\u{f1db}',
        Bootstrap::Building => '\u{f1dd}',
        Bootstrap::BuildingAdd => '\u{f867}',
        Bootstrap::BuildingCheck => '\u{f868}',
        Bootstrap::BuildingDash => '\u{f869}',
        Bootstrap::BuildingDown => '\u{f86a}',
        Bootstrap::BuildingExclamation => '\u{f86b}',
        Bootstrap::BuildingFill => '\u{f876}',
        Bootstrap::BuildingFillAdd => '\u{f86c}',
        Bootstrap::BuildingFillCheck => '\u{f86d}',
        Bootstrap::BuildingFillDash => '\u{f86e}',
        Bootstrap::BuildingFillDown => '\u{f86f}',
        Bootstrap::BuildingFillExclamation => '\u{f870}',
        Bootstrap::BuildingFillGear => '\u{f871}',
        Bootstrap::BuildingFillLock => '\u{f872}',
        Bootstrap::BuildingFillSlash => '\u{f873}',
        Bootstrap::BuildingFillUp => '\u{f874}',
        Bootstrap::BuildingFillX => '\u{f875}',
        Bootstrap::BuildingGear => '\u{f877}',
        Bootstrap::BuildingLock => '\u{f878}',
        Bootstrap::BuildingSlash => '\u{f879}',
        Bootstrap::BuildingUp => '\u{f87a}',
        Bootstrap::BuildingX => '\u{f87b}',
        Bootstrap::Buildings => '\u{f87d}',
        Bootstrap::BuildingsFill => '\u{f87c}',
        Bootstrap::Bullseye => '\u{f1de}',
        Bootstrap::BusFront => '\u{f87f}',
        Bootstrap::BusFrontFill => '\u{f87e}',
        Bootstrap::CCircle => '\u{f7db}',
        Bootstrap::CCircleFill => '\u{f7da}',
        Bootstrap::CSquare => '\u{f7dd}',
        Bootstrap::CSquareFill => '\u{f7dc}',
        Bootstrap::Cake => '\u{f6e0}',
        Bootstrap::CakeFill => '\u{f8e8}',
        Bootstrap::Caketwo => '\u{f6ed}',
        Bootstrap::CaketwoFill => '\u{f8e9}',
        Bootstrap::Calculator => '\u{f1e0}',
        Bootstrap::CalculatorFill => '\u{f1df}',
        Bootstrap::Calendar => '\u{f1f6}',
        Bootstrap::CalendarCheck => '\u{f1e2}',
        Bootstrap::CalendarCheckFill => '\u{f1e1}',
        Bootstrap::CalendarDate => '\u{f1e4}',
        Bootstrap::CalendarDateFill => '\u{f1e3}',
        Bootstrap::CalendarDay => '\u{f1e6}',
        Bootstrap::CalendarDayFill => '\u{f1e5}',
        Bootstrap::CalendarEvent => '\u{f1e8}',
        Bootstrap::CalendarEventFill => '\u{f1e7}',
        Bootstrap::CalendarFill => '\u{f1e9}',
        Bootstrap::CalendarHeart => '\u{f710}',
        Bootstrap::CalendarHeartFill => '\u{f70f}',
        Bootstrap::CalendarMinus => '\u{f1eb}',
        Bootstrap::CalendarMinusFill => '\u{f1ea}',
        Bootstrap::CalendarMonth => '\u{f1ed}',
        Bootstrap::CalendarMonthFill => '\u{f1ec}',
        Bootstrap::CalendarPlus => '\u{f1ef}',
        Bootstrap::CalendarPlusFill => '\u{f1ee}',
        Bootstrap::CalendarRange => '\u{f1f1}',
        Bootstrap::CalendarRangeFill => '\u{f1f0}',
        Bootstrap::CalendarWeek => '\u{f1f3}',
        Bootstrap::CalendarWeekFill => '\u{f1f2}',
        Bootstrap::CalendarX => '\u{f1f5}',
        Bootstrap::CalendarXFill => '\u{f1f4}',
        Bootstrap::Calendarfour => '\u{f218}',
        Bootstrap::CalendarfourEvent => '\u{f215}',
        Bootstrap::CalendarfourRange => '\u{f216}',
        Bootstrap::CalendarfourWeek => '\u{f217}',
        Bootstrap::Calendarthree => '\u{f214}',
        Bootstrap::CalendarthreeEvent => '\u{f20e}',
        Bootstrap::CalendarthreeEventFill => '\u{f20d}',
        Bootstrap::CalendarthreeFill => '\u{f20f}',
        Bootstrap::CalendarthreeRange => '\u{f211}',
        Bootstrap::CalendarthreeRangeFill => '\u{f210}',
        Bootstrap::CalendarthreeWeek => '\u{f213}',
        Bootstrap::CalendarthreeWeekFill => '\u{f212}',
        Bootstrap::Calendartwo => '\u{f20c}',
        Bootstrap::CalendartwoCheck => '\u{f1f8}',
        Bootstrap::CalendartwoCheckFill => '\u{f1f7}',
        Bootstrap::CalendartwoDate => '\u{f1fa}',
        Bootstrap::CalendartwoDateFill => '\u{f1f9}',
        Bootstrap::CalendartwoDay => '\u{f1fc}',
        Bootstrap::CalendartwoDayFill => '\u{f1fb}',
        Bootstrap::CalendartwoEvent => '\u{f1fe}',
        Bootstrap::CalendartwoEventFill => '\u{f1fd}',
        Bootstrap::CalendartwoFill => '\u{f1ff}',
        Bootstrap::CalendartwoHeart => '\u{f712}',
        Bootstrap::CalendartwoHeartFill => '\u{f711}',
        Bootstrap::CalendartwoMinus => '\u{f201}',
        Bootstrap::CalendartwoMinusFill => '\u{f200}',
        Bootstrap::CalendartwoMonth => '\u{f203}',
        Bootstrap::CalendartwoMonthFill => '\u{f202}',
        Bootstrap::CalendartwoPlus => '\u{f205}',
        Bootstrap::CalendartwoPlusFill => '\u{f204}',
        Bootstrap::CalendartwoRange => '\u{f207}',
        Bootstrap::CalendartwoRangeFill => '\u{f206}',
        Bootstrap::CalendartwoWeek => '\u{f209}',
        Bootstrap::CalendartwoWeekFill => '\u{f208}',
        Bootstrap::CalendartwoX => '\u{f20b}',
        Bootstrap::CalendartwoXFill => '\u{f20a}',
        Bootstrap::Camera => '\u{f220}',
        Bootstrap::CameraFill => '\u{f219}',
        Bootstrap::CameraReels => '\u{f21b}',
        Bootstrap::CameraReelsFill => '\u{f21a}',
        Bootstrap::CameraVideo => '\u{f21f}',
        Bootstrap::CameraVideoFill => '\u{f21c}',
        Bootstrap::CameraVideoOff => '\u{f21e}',
        Bootstrap::CameraVideoOffFill => '\u{f21d}',
        Bootstrap::Cameratwo => '\u{f221}',
        Bootstrap::Capslock => '\u{f223}',
        Bootstrap::CapslockFill => '\u{f222}',
        Bootstrap::Capsule => '\u{f7df}',
        Bootstrap::CapsulePill => '\u{f7de}',
        Bootstrap::CarFront => '\u{f7e1}',
        Bootstrap::CarFrontFill => '\u{f7e0}',
        Bootstrap::CardChecklist => '\u{f224}',
        Bootstrap::CardHeading => '\u{f225}',
        Bootstrap::CardImage => '\u{f226}',
        Bootstrap::CardList => '\u{f227}',
        Bootstrap::CardText => '\u{f228}',
        Bootstrap::CaretDown => '\u{f22c}',
        Bootstrap::CaretDownFill => '\u{f229}',
        Bootstrap::CaretDownSquare => '\u{f22b}',
        Bootstrap::CaretDownSquareFill => '\u{f22a}',
        Bootstrap::CaretLeft => '\u{f230}',
        Bootstrap::CaretLeftFill => '\u{f22d}',
        Bootstrap::CaretLeftSquare => '\u{f22f}',
        Bootstrap::CaretLeftSquareFill => '\u{f22e}',
        Bootstrap::CaretRight => '\u{f234}',
        Bootstrap::CaretRightFill => '\u{f231}',
        Bootstrap::CaretRightSquare => '\u{f233}',
        Bootstrap::CaretRightSquareFill => '\u{f232}',
        Bootstrap::CaretUp => '\u{f238}',
        Bootstrap::CaretUpFill => '\u{f235}',
        Bootstrap::CaretUpSquare => '\u{f237}',
        Bootstrap::CaretUpSquareFill => '\u{f236}',
        Bootstrap::Cart => '\u{f242}',
        Bootstrap::CartCheck => '\u{f23a}',
        Bootstrap::CartCheckFill => '\u{f239}',
        Bootstrap::CartDash => '\u{f23c}',
        Bootstrap::CartDashFill => '\u{f23b}',
        Bootstrap::CartFill => '\u{f23d}',
        Bootstrap::CartPlus => '\u{f23f}',
        Bootstrap::CartPlusFill => '\u{f23e}',
        Bootstrap::CartX => '\u{f241}',
        Bootstrap::CartXFill => '\u{f240}',
        Bootstrap::Cartfour => '\u{f245}',
        Bootstrap::Cartthree => '\u{f244}',
        Bootstrap::Carttwo => '\u{f243}',
        Bootstrap::Cash => '\u{f247}',
        Bootstrap::CashCoin => '\u{f632}',
        Bootstrap::CashStack => '\u{f246}',
        Bootstrap::Cassette => '\u{f7e3}',
        Bootstrap::CassetteFill => '\u{f7e2}',
        Bootstrap::Cast => '\u{f248}',
        Bootstrap::CcCircle => '\u{f7e7}',
        Bootstrap::CcCircleFill => '\u{f7e6}',
        Bootstrap::CcSquare => '\u{f7e9}',
        Bootstrap::CcSquareFill => '\u{f7e8}',
        Bootstrap::Chat => '\u{f268}',
        Bootstrap::ChatDots => '\u{f24a}',
        Bootstrap::ChatDotsFill => '\u{f249}',
        Bootstrap::ChatFill => '\u{f24b}',
        Bootstrap::ChatHeart => '\u{f714}',
        Bootstrap::ChatHeartFill => '\u{f713}',
        Bootstrap::ChatLeft => '\u{f253}',
        Bootstrap::ChatLeftDots => '\u{f24d}',
        Bootstrap::ChatLeftDotsFill => '\u{f24c}',
        Bootstrap::ChatLeftFill => '\u{f24e}',
        Bootstrap::ChatLeftHeart => '\u{f716}',
        Bootstrap::ChatLeftHeartFill => '\u{f715}',
        Bootstrap::ChatLeftQuote => '\u{f250}',
        Bootstrap::ChatLeftQuoteFill => '\u{f24f}',
        Bootstrap::ChatLeftText => '\u{f252}',
        Bootstrap::ChatLeftTextFill => '\u{f251}',
        Bootstrap::ChatQuote => '\u{f255}',
        Bootstrap::ChatQuoteFill => '\u{f254}',
        Bootstrap::ChatRight => '\u{f25d}',
        Bootstrap::ChatRightDots => '\u{f257}',
        Bootstrap::ChatRightDotsFill => '\u{f256}',
        Bootstrap::ChatRightFill => '\u{f258}',
        Bootstrap::ChatRightHeart => '\u{f718}',
        Bootstrap::ChatRightHeartFill => '\u{f717}',
        Bootstrap::ChatRightQuote => '\u{f25a}',
        Bootstrap::ChatRightQuoteFill => '\u{f259}',
        Bootstrap::ChatRightText => '\u{f25c}',
        Bootstrap::ChatRightTextFill => '\u{f25b}',
        Bootstrap::ChatSquare => '\u{f265}',
        Bootstrap::ChatSquareDots => '\u{f25f}',
        Bootstrap::ChatSquareDotsFill => '\u{f25e}',
        Bootstrap::ChatSquareFill => '\u{f260}',
        Bootstrap::ChatSquareHeart => '\u{f71a}',
        Bootstrap::ChatSquareHeartFill => '\u{f719}',
        Bootstrap::ChatSquareQuote => '\u{f262}',
        Bootstrap::ChatSquareQuoteFill => '\u{f261}',
        Bootstrap::ChatSquareText => '\u{f264}',
        Bootstrap::ChatSquareTextFill => '\u{f263}',
        Bootstrap::ChatText => '\u{f267}',
        Bootstrap::ChatTextFill => '\u{f266}',
        Bootstrap::Check => '\u{f26e}',
        Bootstrap::CheckAll => '\u{f269}',
        Bootstrap::CheckCircle => '\u{f26b}',
        Bootstrap::CheckCircleFill => '\u{f26a}',
        Bootstrap::CheckLg => '\u{f633}',
        Bootstrap::CheckSquare => '\u{f26d}',
        Bootstrap::CheckSquareFill => '\u{f26c}',
        Bootstrap::Checktwo => '\u{f272}',
        Bootstrap::ChecktwoAll => '\u{f26f}',
        Bootstrap::ChecktwoCircle => '\u{f270}',
        Bootstrap::ChecktwoSquare => '\u{f271}',
        Bootstrap::ChevronBarContract => '\u{f273}',
        Bootstrap::ChevronBarDown => '\u{f274}',
        Bootstrap::ChevronBarExpand => '\u{f275}',
        Bootstrap::ChevronBarLeft => '\u{f276}',
        Bootstrap::ChevronBarRight => '\u{f277}',
        Bootstrap::ChevronBarUp => '\u{f278}',
        Bootstrap::ChevronCompactDown => '\u{f279}',
        Bootstrap::ChevronCompactLeft => '\u{f27a}',
        Bootstrap::ChevronCompactRight => '\u{f27b}',
        Bootstrap::ChevronCompactUp => '\u{f27c}',
        Bootstrap::ChevronContract => '\u{f27d}',
        Bootstrap::ChevronDoubleDown => '\u{f27e}',
        Bootstrap::ChevronDoubleLeft => '\u{f27f}',
        Bootstrap::ChevronDoubleRight => '\u{f280}',
        Bootstrap::ChevronDoubleUp => '\u{f281}',
        Bootstrap::ChevronDown => '\u{f282}',
        Bootstrap::ChevronExpand => '\u{f283}',
        Bootstrap::ChevronLeft => '\u{f284}',
        Bootstrap::ChevronRight => '\u{f285}',
        Bootstrap::ChevronUp => '\u{f286}',
        Bootstrap::Circle => '\u{f28a}',
        Bootstrap::CircleFill => '\u{f287}',
        Bootstrap::CircleHalf => '\u{f288}',
        Bootstrap::CircleSquare => '\u{f289}',
        Bootstrap::Clipboard => '\u{f290}',
        Bootstrap::ClipboardCheck => '\u{f28b}',
        Bootstrap::ClipboardCheckFill => '\u{f71b}',
        Bootstrap::ClipboardData => '\u{f28c}',
        Bootstrap::ClipboardDataFill => '\u{f71c}',
        Bootstrap::ClipboardFill => '\u{f71d}',
        Bootstrap::ClipboardHeart => '\u{f71f}',
        Bootstrap::ClipboardHeartFill => '\u{f71e}',
        Bootstrap::ClipboardMinus => '\u{f28d}',
        Bootstrap::ClipboardMinusFill => '\u{f720}',
        Bootstrap::ClipboardPlus => '\u{f28e}',
        Bootstrap::ClipboardPlusFill => '\u{f721}',
        Bootstrap::ClipboardPulse => '\u{f722}',
        Bootstrap::ClipboardX => '\u{f28f}',
        Bootstrap::ClipboardXFill => '\u{f723}',
        Bootstrap::Clipboardtwo => '\u{f733}',
        Bootstrap::ClipboardtwoCheck => '\u{f725}',
        Bootstrap::ClipboardtwoCheckFill => '\u{f724}',
        Bootstrap::ClipboardtwoData => '\u{f727}',
        Bootstrap::ClipboardtwoDataFill => '\u{f726}',
        Bootstrap::ClipboardtwoFill => '\u{f728}',
        Bootstrap::ClipboardtwoHeart => '\u{f72a}',
        Bootstrap::ClipboardtwoHeartFill => '\u{f729}',
        Bootstrap::ClipboardtwoMinus => '\u{f72c}',
        Bootstrap::ClipboardtwoMinusFill => '\u{f72b}',
        Bootstrap::ClipboardtwoPlus => '\u{f72e}',
        Bootstrap::ClipboardtwoPlusFill => '\u{f72d}',
        Bootstrap::ClipboardtwoPulse => '\u{f730}',
        Bootstrap::ClipboardtwoPulseFill => '\u{f72f}',
        Bootstrap::ClipboardtwoX => '\u{f732}',
        Bootstrap::ClipboardtwoXFill => '\u{f731}',
        Bootstrap::Clock => '\u{f293}',
        Bootstrap::ClockFill => '\u{f291}',
        Bootstrap::ClockHistory => '\u{f292}',
        Bootstrap::Cloud => '\u{f2c1}',
        Bootstrap::CloudArrowDown => '\u{f295}',
        Bootstrap::CloudArrowDownFill => '\u{f294}',
        Bootstrap::CloudArrowUp => '\u{f297}',
        Bootstrap::CloudArrowUpFill => '\u{f296}',
        Bootstrap::CloudCheck => '\u{f299}',
        Bootstrap::CloudCheckFill => '\u{f298}',
        Bootstrap::CloudDownload => '\u{f29b}',
        Bootstrap::CloudDownloadFill => '\u{f29a}',
        Bootstrap::CloudDrizzle => '\u{f29d}',
        Bootstrap::CloudDrizzleFill => '\u{f29c}',
        Bootstrap::CloudFill => '\u{f29e}',
        Bootstrap::CloudFog => '\u{f2a0}',
        Bootstrap::CloudFogFill => '\u{f29f}',
        Bootstrap::CloudFogtwo => '\u{f2a2}',
        Bootstrap::CloudFogtwoFill => '\u{f2a1}',
        Bootstrap::CloudHail => '\u{f2a4}',
        Bootstrap::CloudHailFill => '\u{f2a3}',
        Bootstrap::CloudHaze => '\u{f2a7}',
        Bootstrap::CloudHazeFill => '\u{f2a6}',
        Bootstrap::CloudHazetwo => '\u{f6f7}',
        Bootstrap::CloudHazetwoFill => '\u{f2a8}',
        Bootstrap::CloudLightning => '\u{f2ac}',
        Bootstrap::CloudLightningFill => '\u{f2a9}',
        Bootstrap::CloudLightningRain => '\u{f2ab}',
        Bootstrap::CloudLightningRainFill => '\u{f2aa}',
        Bootstrap::CloudMinus => '\u{f2ae}',
        Bootstrap::CloudMinusFill => '\u{f2ad}',
        Bootstrap::CloudMoon => '\u{f2b0}',
        Bootstrap::CloudMoonFill => '\u{f2af}',
        Bootstrap::CloudPlus => '\u{f2b2}',
        Bootstrap::CloudPlusFill => '\u{f2b1}',
        Bootstrap::CloudRain => '\u{f2b6}',
        Bootstrap::CloudRainFill => '\u{f2b3}',
        Bootstrap::CloudRainHeavy => '\u{f2b5}',
        Bootstrap::CloudRainHeavyFill => '\u{f2b4}',
        Bootstrap::CloudSlash => '\u{f2b8}',
        Bootstrap::CloudSlashFill => '\u{f2b7}',
        Bootstrap::CloudSleet => '\u{f2ba}',
        Bootstrap::CloudSleetFill => '\u{f2b9}',
        Bootstrap::CloudSnow => '\u{f2bc}',
        Bootstrap::CloudSnowFill => '\u{f2bb}',
        Bootstrap::CloudSun => '\u{f2be}',
        Bootstrap::CloudSunFill => '\u{f2bd}',
        Bootstrap::CloudUpload => '\u{f2c0}',
        Bootstrap::CloudUploadFill => '\u{f2bf}',
        Bootstrap::Clouds => '\u{f2c3}',
        Bootstrap::CloudsFill => '\u{f2c2}',
        Bootstrap::Cloudy => '\u{f2c5}',
        Bootstrap::CloudyFill => '\u{f2c4}',
        Bootstrap::Code => '\u{f2c8}',
        Bootstrap::CodeSlash => '\u{f2c6}',
        Bootstrap::CodeSquare => '\u{f2c7}',
        Bootstrap::Coin => '\u{f634}',
        Bootstrap::Collection => '\u{f2cc}',
        Bootstrap::CollectionFill => '\u{f2c9}',
        Bootstrap::CollectionPlay => '\u{f2cb}',
        Bootstrap::CollectionPlayFill => '\u{f2ca}',
        Bootstrap::Columns => '\u{f2ce}',
        Bootstrap::ColumnsGap => '\u{f2cd}',
        Bootstrap::Command => '\u{f2cf}',
        Bootstrap::Compass => '\u{f2d1}',
        Bootstrap::CompassFill => '\u{f2d0}',
        Bootstrap::Cone => '\u{f2d3}',
        Bootstrap::ConeStriped => '\u{f2d2}',
        Bootstrap::Controller => '\u{f2d4}',
        Bootstrap::Cookie => '\u{f6ee}',
        Bootstrap::Copy => '\u{f759}',
        Bootstrap::Cpu => '\u{f2d6}',
        Bootstrap::CpuFill => '\u{f2d5}',
        Bootstrap::CreditCard => '\u{f2dc}',
        Bootstrap::CreditCardFill => '\u{f2db}',
        Bootstrap::CreditCardTwoBack => '\u{f2d8}',
        Bootstrap::CreditCardTwoBackFill => '\u{f2d7}',
        Bootstrap::CreditCardTwoFront => '\u{f2da}',
        Bootstrap::CreditCardTwoFrontFill => '\u{f2d9}',
        Bootstrap::Crop => '\u{f2dd}',
        Bootstrap::Crosshair => '\u{f769}',
        Bootstrap::Crosshairtwo => '\u{f794}',
        Bootstrap::Cup => '\u{f2e0}',
        Bootstrap::CupFill => '\u{f2de}',
        Bootstrap::CupHot => '\u{f7eb}',
        Bootstrap::CupHotFill => '\u{f7ea}',
        Bootstrap::CupStraw => '\u{f2df}',
        Bootstrap::CurrencyBitcoin => '\u{f635}',
        Bootstrap::CurrencyDollar => '\u{f636}',
        Bootstrap::CurrencyEuro => '\u{f637}',
        Bootstrap::CurrencyExchange => '\u{f638}',
        Bootstrap::CurrencyPound => '\u{f639}',
        Bootstrap::CurrencyRupee => '\u{f7ec}',
        Bootstrap::CurrencyYen => '\u{f63a}',
        Bootstrap::Cursor => '\u{f2e3}',
        Bootstrap::CursorFill => '\u{f2e1}',
        Bootstrap::CursorText => '\u{f2e2}',
        Bootstrap::Dash => '\u{f2ea}',
        Bootstrap::DashCircle => '\u{f2e6}',
        Bootstrap::DashCircleDotted => '\u{f2e4}',
        Bootstrap::DashCircleFill => '\u{f2e5}',
        Bootstrap::DashLg => '\u{f63b}',
        Bootstrap::DashSquare => '\u{f2e9}',
        Bootstrap::DashSquareDotted => '\u{f2e7}',
        Bootstrap::DashSquareFill => '\u{f2e8}',
        Bootstrap::Database => '\u{f8c4}',
        Bootstrap::DatabaseAdd => '\u{f8af}',
        Bootstrap::DatabaseCheck => '\u{f8b0}',
        Bootstrap::DatabaseDash => '\u{f8b1}',
        Bootstrap::DatabaseDown => '\u{f8b2}',
        Bootstrap::DatabaseExclamation => '\u{f8b3}',
        Bootstrap::DatabaseFill => '\u{f8be}',
        Bootstrap::DatabaseFillAdd => '\u{f8b4}',
        Bootstrap::DatabaseFillCheck => '\u{f8b5}',
        Bootstrap::DatabaseFillDash => '\u{f8b6}',
        Bootstrap::DatabaseFillDown => '\u{f8b7}',
        Bootstrap::DatabaseFillExclamation => '\u{f8b8}',
        Bootstrap::DatabaseFillGear => '\u{f8b9}',
        Bootstrap::DatabaseFillLock => '\u{f8ba}',
        Bootstrap::DatabaseFillSlash => '\u{f8bb}',
        Bootstrap::DatabaseFillUp => '\u{f8bc}',
        Bootstrap::DatabaseFillX => '\u{f8bd}',
        Bootstrap::DatabaseGear => '\u{f8bf}',
        Bootstrap::DatabaseLock => '\u{f8c0}',
        Bootstrap::DatabaseSlash => '\u{f8c1}',
        Bootstrap::DatabaseUp => '\u{f8c2}',
        Bootstrap::DatabaseX => '\u{f8c3}',
        Bootstrap::DeviceHdd => '\u{f6f9}',
        Bootstrap::DeviceHddFill => '\u{f6f8}',
        Bootstrap::DeviceSsd => '\u{f6fb}',
        Bootstrap::DeviceSsdFill => '\u{f6fa}',
        Bootstrap::DiagramThree => '\u{f2ee}',
        Bootstrap::DiagramThreeFill => '\u{f2ed}',
        Bootstrap::DiagramTwo => '\u{f2ec}',
        Bootstrap::DiagramTwoFill => '\u{f2eb}',
        Bootstrap::Diamond => '\u{f2f1}',
        Bootstrap::DiamondFill => '\u{f2ef}',
        Bootstrap::DiamondHalf => '\u{f2f0}',
        Bootstrap::DiceFive => '\u{f2fb}',
        Bootstrap::DiceFiveFill => '\u{f2fa}',
        Bootstrap::DiceFour => '\u{f2f9}',
        Bootstrap::DiceFourFill => '\u{f2f8}',
        Bootstrap::DiceOne => '\u{f2f3}',
        Bootstrap::DiceOneFill => '\u{f2f2}',
        Bootstrap::DiceSix => '\u{f2fd}',
        Bootstrap::DiceSixFill => '\u{f2fc}',
        Bootstrap::DiceThree => '\u{f2f7}',
        Bootstrap::DiceThreeFill => '\u{f2f6}',
        Bootstrap::DiceTwo => '\u{f2f5}',
        Bootstrap::DiceTwoFill => '\u{f2f4}',
        Bootstrap::Disc => '\u{f2ff}',
        Bootstrap::DiscFill => '\u{f2fe}',
        Bootstrap::Discord => '\u{f300}',
        Bootstrap::Display => '\u{f302}',
        Bootstrap::DisplayFill => '\u{f301}',
        Bootstrap::Displayport => '\u{f6e1}',
        Bootstrap::DisplayportFill => '\u{f6fc}',
        Bootstrap::DistributeHorizontal => '\u{f303}',
        Bootstrap::DistributeVertical => '\u{f304}',
        Bootstrap::DoorClosed => '\u{f306}',
        Bootstrap::DoorClosedFill => '\u{f305}',
        Bootstrap::DoorOpen => '\u{f308}',
        Bootstrap::DoorOpenFill => '\u{f307}',
        Bootstrap::Dot => '\u{f309}',
        Bootstrap::Download => '\u{f30a}',
        Bootstrap::Dpad => '\u{f687}',
        Bootstrap::DpadFill => '\u{f686}',
        Bootstrap::Dribbble => '\u{f65f}',
        Bootstrap::Dropbox => '\u{f7ed}',
        Bootstrap::Droplet => '\u{f30d}',
        Bootstrap::DropletFill => '\u{f30b}',
        Bootstrap::DropletHalf => '\u{f30c}',
        Bootstrap::Duffle => '\u{f8eb}',
        Bootstrap::DuffleFill => '\u{f8ea}',
        Bootstrap::Ear => '\u{f689}',
        Bootstrap::EarFill => '\u{f688}',
        Bootstrap::Earbuds => '\u{f30e}',
        Bootstrap::Easel => '\u{f310}',
        Bootstrap::EaselFill => '\u{f30f}',
        Bootstrap::Easelthree => '\u{f66f}',
        Bootstrap::EaselthreeFill => '\u{f66e}',
        Bootstrap::Easeltwo => '\u{f66d}',
        Bootstrap::EaseltwoFill => '\u{f66c}',
        Bootstrap::Egg => '\u{f313}',
        Bootstrap::EggFill => '\u{f311}',
        Bootstrap::EggFried => '\u{f312}',
        Bootstrap::EightCircle => '\u{f7c1}',
        Bootstrap::EightCircleFill => '\u{f7c0}',
        Bootstrap::EightSquare => '\u{f7c3}',
        Bootstrap::EightSquareFill => '\u{f7c2}',
        Bootstrap::Eject => '\u{f315}',
        Bootstrap::EjectFill => '\u{f314}',
        Bootstrap::EmojiAngry => '\u{f317}',
        Bootstrap::EmojiAngryFill => '\u{f316}',
        Bootstrap::EmojiAstonished => '\u{f79a}',
        Bootstrap::EmojiAstonishedFill => '\u{f795}',
        Bootstrap::EmojiDizzy => '\u{f319}',
        Bootstrap::EmojiDizzyFill => '\u{f318}',
        Bootstrap::EmojiExpressionless => '\u{f31b}',
        Bootstrap::EmojiExpressionlessFill => '\u{f31a}',
        Bootstrap::EmojiFrown => '\u{f31d}',
        Bootstrap::EmojiFrownFill => '\u{f31c}',
        Bootstrap::EmojiGrimace => '\u{f7a0}',
        Bootstrap::EmojiGrimaceFill => '\u{f79b}',
        Bootstrap::EmojiGrin => '\u{f7a6}',
        Bootstrap::EmojiGrinFill => '\u{f7a1}',
        Bootstrap::EmojiHeartEyes => '\u{f31f}',
        Bootstrap::EmojiHeartEyesFill => '\u{f31e}',
        Bootstrap::EmojiKiss => '\u{f735}',
        Bootstrap::EmojiKissFill => '\u{f734}',
        Bootstrap::EmojiLaughing => '\u{f321}',
        Bootstrap::EmojiLaughingFill => '\u{f320}',
        Bootstrap::EmojiNeutral => '\u{f323}',
        Bootstrap::EmojiNeutralFill => '\u{f322}',
        Bootstrap::EmojiSmile => '\u{f327}',
        Bootstrap::EmojiSmileFill => '\u{f324}',
        Bootstrap::EmojiSmileUpsideDown => '\u{f326}',
        Bootstrap::EmojiSmileUpsideDownFill => '\u{f325}',
        Bootstrap::EmojiSunglasses => '\u{f329}',
        Bootstrap::EmojiSunglassesFill => '\u{f328}',
        Bootstrap::EmojiSurprise => '\u{f7ac}',
        Bootstrap::EmojiSurpriseFill => '\u{f7a7}',
        Bootstrap::EmojiTear => '\u{f7b2}',
        Bootstrap::EmojiTearFill => '\u{f7ad}',
        Bootstrap::EmojiWink => '\u{f32b}',
        Bootstrap::EmojiWinkFill => '\u{f32a}',
        Bootstrap::Envelope => '\u{f32f}',
        Bootstrap::EnvelopeArrowDown => '\u{f7b8}',
        Bootstrap::EnvelopeArrowDownFill => '\u{f7b3}',
        Bootstrap::EnvelopeArrowUp => '\u{f7be}',
        Bootstrap::EnvelopeArrowUpFill => '\u{f7b9}',
        Bootstrap::EnvelopeAt => '\u{f84c}',
        Bootstrap::EnvelopeAtFill => '\u{f84b}',
        Bootstrap::EnvelopeCheck => '\u{f68c}',
        Bootstrap::EnvelopeCheckFill => '\u{f68b}',
        Bootstrap::EnvelopeDash => '\u{f68f}',
        Bootstrap::EnvelopeDashFill => '\u{f68e}',
        Bootstrap::EnvelopeExclamation => '\u{f692}',
        Bootstrap::EnvelopeExclamationFill => '\u{f691}',
        Bootstrap::EnvelopeFill => '\u{f32c}',
        Bootstrap::EnvelopeHeart => '\u{f737}',
        Bootstrap::EnvelopeHeartFill => '\u{f736}',
        Bootstrap::EnvelopeOpen => '\u{f32e}',
        Bootstrap::EnvelopeOpenFill => '\u{f32d}',
        Bootstrap::EnvelopeOpenHeart => '\u{f739}',
        Bootstrap::EnvelopeOpenHeartFill => '\u{f738}',
        Bootstrap::EnvelopePaper => '\u{f73d}',
        Bootstrap::EnvelopePaperFill => '\u{f73a}',
        Bootstrap::EnvelopePaperHeart => '\u{f73c}',
        Bootstrap::EnvelopePaperHeartFill => '\u{f73b}',
        Bootstrap::EnvelopePlus => '\u{f694}',
        Bootstrap::EnvelopePlusFill => '\u{f693}',
        Bootstrap::EnvelopeSlash => '\u{f697}',
        Bootstrap::EnvelopeSlashFill => '\u{f696}',
        Bootstrap::EnvelopeX => '\u{f69a}',
        Bootstrap::EnvelopeXFill => '\u{f699}',
        Bootstrap::Eraser => '\u{f331}',
        Bootstrap::EraserFill => '\u{f330}',
        Bootstrap::Escape => '\u{f7ee}',
        Bootstrap::Ethernet => '\u{f6d5}',
        Bootstrap::EvFront => '\u{f881}',
        Bootstrap::EvFrontFill => '\u{f880}',
        Bootstrap::EvStation => '\u{f83a}',
        Bootstrap::EvStationFill => '\u{f839}',
        Bootstrap::Exclamation => '\u{f33c}',
        Bootstrap::ExclamationCircle => '\u{f333}',
        Bootstrap::ExclamationCircleFill => '\u{f332}',
        Bootstrap::ExclamationDiamond => '\u{f335}',
        Bootstrap::ExclamationDiamondFill => '\u{f334}',
        Bootstrap::ExclamationLg => '\u{f63c}',
        Bootstrap::ExclamationOctagon => '\u{f337}',
        Bootstrap::ExclamationOctagonFill => '\u{f336}',
        Bootstrap::ExclamationSquare => '\u{f339}',
        Bootstrap::ExclamationSquareFill => '\u{f338}',
        Bootstrap::ExclamationTriangle => '\u{f33b}',
        Bootstrap::ExclamationTriangleFill => '\u{f33a}',
        Bootstrap::Exclude => '\u{f33d}',
        Bootstrap::Explicit => '\u{f69c}',
        Bootstrap::ExplicitFill => '\u{f69b}',
        Bootstrap::Exposure => '\u{f8ec}',
        Bootstrap::Eye => '\u{f341}',
        Bootstrap::EyeFill => '\u{f33e}',
        Bootstrap::EyeSlash => '\u{f340}',
        Bootstrap::EyeSlashFill => '\u{f33f}',
        Bootstrap::Eyedropper => '\u{f342}',
        Bootstrap::Eyeglasses => '\u{f343}',
        Bootstrap::Facebook => '\u{f344}',
        Bootstrap::Fan => '\u{f670}',
        Bootstrap::FastForward => '\u{f7f4}',
        Bootstrap::FastForwardBtn => '\u{f7f0}',
        Bootstrap::FastForwardBtnFill => '\u{f7ef}',
        Bootstrap::FastForwardCircle => '\u{f7f2}',
        Bootstrap::FastForwardCircleFill => '\u{f7f1}',
        Bootstrap::FastForwardFill => '\u{f7f3}',
        Bootstrap::Feather => '\u{f7bf}',
        Bootstrap::Feathertwo => '\u{f7c4}',
        Bootstrap::File => '\u{f3c0}',
        Bootstrap::FileArrowDown => '\u{f346}',
        Bootstrap::FileArrowDownFill => '\u{f345}',
        Bootstrap::FileArrowUp => '\u{f348}',
        Bootstrap::FileArrowUpFill => '\u{f347}',
        Bootstrap::FileBarGraph => '\u{f34a}',
        Bootstrap::FileBarGraphFill => '\u{f349}',
        Bootstrap::FileBinary => '\u{f34c}',
        Bootstrap::FileBinaryFill => '\u{f34b}',
        Bootstrap::FileBreak => '\u{f34e}',
        Bootstrap::FileBreakFill => '\u{f34d}',
        Bootstrap::FileCheck => '\u{f350}',
        Bootstrap::FileCheckFill => '\u{f34f}',
        Bootstrap::FileCode => '\u{f352}',
        Bootstrap::FileCodeFill => '\u{f351}',
        Bootstrap::FileDiff => '\u{f354}',
        Bootstrap::FileDiffFill => '\u{f353}',
        Bootstrap::FileEarmark => '\u{f392}',
        Bootstrap::FileEarmarkArrowDown => '\u{f356}',
        Bootstrap::FileEarmarkArrowDownFill => '\u{f355}',
        Bootstrap::FileEarmarkArrowUp => '\u{f358}',
        Bootstrap::FileEarmarkArrowUpFill => '\u{f357}',
        Bootstrap::FileEarmarkBarGraph => '\u{f35a}',
        Bootstrap::FileEarmarkBarGraphFill => '\u{f359}',
        Bootstrap::FileEarmarkBinary => '\u{f35c}',
        Bootstrap::FileEarmarkBinaryFill => '\u{f35b}',
        Bootstrap::FileEarmarkBreak => '\u{f35e}',
        Bootstrap::FileEarmarkBreakFill => '\u{f35d}',
        Bootstrap::FileEarmarkCheck => '\u{f360}',
        Bootstrap::FileEarmarkCheckFill => '\u{f35f}',
        Bootstrap::FileEarmarkCode => '\u{f362}',
        Bootstrap::FileEarmarkCodeFill => '\u{f361}',
        Bootstrap::FileEarmarkDiff => '\u{f364}',
        Bootstrap::FileEarmarkDiffFill => '\u{f363}',
        Bootstrap::FileEarmarkEasel => '\u{f366}',
        Bootstrap::FileEarmarkEaselFill => '\u{f365}',
        Bootstrap::FileEarmarkExcel => '\u{f368}',
        Bootstrap::FileEarmarkExcelFill => '\u{f367}',
        Bootstrap::FileEarmarkFill => '\u{f369}',
        Bootstrap::FileEarmarkFont => '\u{f36b}',
        Bootstrap::FileEarmarkFontFill => '\u{f36a}',
        Bootstrap::FileEarmarkImage => '\u{f36d}',
        Bootstrap::FileEarmarkImageFill => '\u{f36c}',
        Bootstrap::FileEarmarkLock => '\u{f36f}',
        Bootstrap::FileEarmarkLockFill => '\u{f36e}',
        Bootstrap::FileEarmarkLocktwo => '\u{f371}',
        Bootstrap::FileEarmarkLocktwoFill => '\u{f370}',
        Bootstrap::FileEarmarkMedical => '\u{f373}',
        Bootstrap::FileEarmarkMedicalFill => '\u{f372}',
        Bootstrap::FileEarmarkMinus => '\u{f375}',
        Bootstrap::FileEarmarkMinusFill => '\u{f374}',
        Bootstrap::FileEarmarkMusic => '\u{f377}',
        Bootstrap::FileEarmarkMusicFill => '\u{f376}',
        Bootstrap::FileEarmarkPdf => '\u{f63e}',
        Bootstrap::FileEarmarkPdfFill => '\u{f63d}',
        Bootstrap::FileEarmarkPerson => '\u{f379}',
        Bootstrap::FileEarmarkPersonFill => '\u{f378}',
        Bootstrap::FileEarmarkPlay => '\u{f37b}',
        Bootstrap::FileEarmarkPlayFill => '\u{f37a}',
        Bootstrap::FileEarmarkPlus => '\u{f37d}',
        Bootstrap::FileEarmarkPlusFill => '\u{f37c}',
        Bootstrap::FileEarmarkPost => '\u{f37f}',
        Bootstrap::FileEarmarkPostFill => '\u{f37e}',
        Bootstrap::FileEarmarkPpt => '\u{f381}',
        Bootstrap::FileEarmarkPptFill => '\u{f380}',
        Bootstrap::FileEarmarkRichtext => '\u{f383}',
        Bootstrap::FileEarmarkRichtextFill => '\u{f382}',
        Bootstrap::FileEarmarkRuled => '\u{f385}',
        Bootstrap::FileEarmarkRuledFill => '\u{f384}',
        Bootstrap::FileEarmarkSlides => '\u{f387}',
        Bootstrap::FileEarmarkSlidesFill => '\u{f386}',
        Bootstrap::FileEarmarkSpreadsheet => '\u{f389}',
        Bootstrap::FileEarmarkSpreadsheetFill => '\u{f388}',
        Bootstrap::FileEarmarkText => '\u{f38b}',
        Bootstrap::FileEarmarkTextFill => '\u{f38a}',
        Bootstrap::FileEarmarkWord => '\u{f38d}',
        Bootstrap::FileEarmarkWordFill => '\u{f38c}',
        Bootstrap::FileEarmarkX => '\u{f38f}',
        Bootstrap::FileEarmarkXFill => '\u{f38e}',
        Bootstrap::FileEarmarkZip => '\u{f391}',
        Bootstrap::FileEarmarkZipFill => '\u{f390}',
        Bootstrap::FileEasel => '\u{f394}',
        Bootstrap::FileEaselFill => '\u{f393}',
        Bootstrap::FileExcel => '\u{f396}',
        Bootstrap::FileExcelFill => '\u{f395}',
        Bootstrap::FileFill => '\u{f397}',
        Bootstrap::FileFont => '\u{f399}',
        Bootstrap::FileFontFill => '\u{f398}',
        Bootstrap::FileImage => '\u{f39b}',
        Bootstrap::FileImageFill => '\u{f39a}',
        Bootstrap::FileLock => '\u{f39d}',
        Bootstrap::FileLockFill => '\u{f39c}',
        Bootstrap::FileLocktwo => '\u{f39f}',
        Bootstrap::FileLocktwoFill => '\u{f39e}',
        Bootstrap::FileMedical => '\u{f3a1}',
        Bootstrap::FileMedicalFill => '\u{f3a0}',
        Bootstrap::FileMinus => '\u{f3a3}',
        Bootstrap::FileMinusFill => '\u{f3a2}',
        Bootstrap::FileMusic => '\u{f3a5}',
        Bootstrap::FileMusicFill => '\u{f3a4}',
        Bootstrap::FilePdf => '\u{f640}',
        Bootstrap::FilePdfFill => '\u{f63f}',
        Bootstrap::FilePerson => '\u{f3a7}',
        Bootstrap::FilePersonFill => '\u{f3a6}',
        Bootstrap::FilePlay => '\u{f3a9}',
        Bootstrap::FilePlayFill => '\u{f3a8}',
        Bootstrap::FilePlus => '\u{f3ab}',
        Bootstrap::FilePlusFill => '\u{f3aa}',
        Bootstrap::FilePost => '\u{f3ad}',
        Bootstrap::FilePostFill => '\u{f3ac}',
        Bootstrap::FilePpt => '\u{f3af}',
        Bootstrap::FilePptFill => '\u{f3ae}',
        Bootstrap::FileRichtext => '\u{f3b1}',
        Bootstrap::FileRichtextFill => '\u{f3b0}',
        Bootstrap::FileRuled => '\u{f3b3}',
        Bootstrap::FileRuledFill => '\u{f3b2}',
        Bootstrap::FileSlides => '\u{f3b5}',
        Bootstrap::FileSlidesFill => '\u{f3b4}',
        Bootstrap::FileSpreadsheet => '\u{f3b7}',
        Bootstrap::FileSpreadsheetFill => '\u{f3b6}',
        Bootstrap::FileText => '\u{f3b9}',
        Bootstrap::FileTextFill => '\u{f3b8}',
        Bootstrap::FileWord => '\u{f3bb}',
        Bootstrap::FileWordFill => '\u{f3ba}',
        Bootstrap::FileX => '\u{f3bd}',
        Bootstrap::FileXFill => '\u{f3bc}',
        Bootstrap::FileZip => '\u{f3bf}',
        Bootstrap::FileZipFill => '\u{f3be}',
        Bootstrap::Files => '\u{f3c2}',
        Bootstrap::FilesAlt => '\u{f3c1}',
        Bootstrap::FiletypeAac => '\u{f73e}',
        Bootstrap::FiletypeAi => '\u{f73f}',
        Bootstrap::FiletypeBmp => '\u{f740}',
        Bootstrap::FiletypeCs => '\u{f741}',
        Bootstrap::FiletypeCss => '\u{f742}',
        Bootstrap::FiletypeCsv => '\u{f743}',
        Bootstrap::FiletypeDoc => '\u{f744}',
        Bootstrap::FiletypeDocx => '\u{f745}',
        Bootstrap::FiletypeExe => '\u{f746}',
        Bootstrap::FiletypeGif => '\u{f747}',
        Bootstrap::FiletypeHeic => '\u{f748}',
        Bootstrap::FiletypeHtml => '\u{f749}',
        Bootstrap::FiletypeJava => '\u{f74a}',
        Bootstrap::FiletypeJpg => '\u{f74b}',
        Bootstrap::FiletypeJs => '\u{f74c}',
        Bootstrap::FiletypeJson => '\u{f791}',
        Bootstrap::FiletypeJsx => '\u{f74d}',
        Bootstrap::FiletypeKey => '\u{f74e}',
        Bootstrap::FiletypeMd => '\u{f750}',
        Bootstrap::FiletypeMdx => '\u{f751}',
        Bootstrap::FiletypeMfourp => '\u{f74f}',
        Bootstrap::FiletypeMov => '\u{f752}',
        Bootstrap::FiletypeMpfour => '\u{f754}',
        Bootstrap::FiletypeMpthree => '\u{f753}',
        Bootstrap::FiletypeOtf => '\u{f755}',
        Bootstrap::FiletypePdf => '\u{f756}',
        Bootstrap::FiletypePhp => '\u{f757}',
        Bootstrap::FiletypePng => '\u{f758}',
        Bootstrap::FiletypePpt => '\u{f75a}',
        Bootstrap::FiletypePptx => '\u{f792}',
        Bootstrap::FiletypePsd => '\u{f75b}',
        Bootstrap::FiletypePy => '\u{f75c}',
        Bootstrap::FiletypeRaw => '\u{f75d}',
        Bootstrap::FiletypeRb => '\u{f75e}',
        Bootstrap::FiletypeSass => '\u{f75f}',
        Bootstrap::FiletypeScss => '\u{f760}',
        Bootstrap::FiletypeSh => '\u{f761}',
        Bootstrap::FiletypeSql => '\u{f7f5}',
        Bootstrap::FiletypeSvg => '\u{f762}',
        Bootstrap::FiletypeTiff => '\u{f763}',
        Bootstrap::FiletypeTsx => '\u{f764}',
        Bootstrap::FiletypeTtf => '\u{f765}',
        Bootstrap::FiletypeTxt => '\u{f766}',
        Bootstrap::FiletypeWav => '\u{f767}',
        Bootstrap::FiletypeWoff => '\u{f768}',
        Bootstrap::FiletypeXls => '\u{f76a}',
        Bootstrap::FiletypeXlsx => '\u{f793}',
        Bootstrap::FiletypeXml => '\u{f76b}',
        Bootstrap::FiletypeYml => '\u{f76c}',
        Bootstrap::Film => '\u{f3c3}',
        Bootstrap::Filter => '\u{f3ca}',
        Bootstrap::FilterCircle => '\u{f3c5}',
        Bootstrap::FilterCircleFill => '\u{f3c4}',
        Bootstrap::FilterLeft => '\u{f3c6}',
        Bootstrap::FilterRight => '\u{f3c7}',
        Bootstrap::FilterSquare => '\u{f3c9}',
        Bootstrap::FilterSquareFill => '\u{f3c8}',
        Bootstrap::Fingerprint => '\u{f671}',
        Bootstrap::Fire => '\u{f7f6}',
        Bootstrap::FiveCircle => '\u{f7af}',
        Bootstrap::FiveCircleFill => '\u{f7ae}',
        Bootstrap::FiveSquare => '\u{f7b1}',
        Bootstrap::FiveSquareFill => '\u{f7b0}',
        Bootstrap::Flag => '\u{f3cc}',
        Bootstrap::FlagFill => '\u{f3cb}',
        Bootstrap::Floppy => '\u{f7d8}',
        Bootstrap::FloppyFill => '\u{f7c5}',
        Bootstrap::Floppytwo => '\u{f7e4}',
        Bootstrap::FloppytwoFill => '\u{f7d9}',
        Bootstrap::Flowerone => '\u{f3cd}',
        Bootstrap::Flowerthree => '\u{f3cf}',
        Bootstrap::Flowertwo => '\u{f3ce}',
        Bootstrap::Folder => '\u{f3d7}',
        Bootstrap::FolderCheck => '\u{f3d0}',
        Bootstrap::FolderFill => '\u{f3d1}',
        Bootstrap::FolderMinus => '\u{f3d2}',
        Bootstrap::FolderPlus => '\u{f3d3}',
        Bootstrap::FolderSymlink => '\u{f3d5}',
        Bootstrap::FolderSymlinkFill => '\u{f3d4}',
        Bootstrap::FolderX => '\u{f3d6}',
        Bootstrap::Foldertwo => '\u{f3d9}',
        Bootstrap::FoldertwoOpen => '\u{f3d8}',
        Bootstrap::Fonts => '\u{f3da}',
        Bootstrap::Forward => '\u{f3dc}',
        Bootstrap::ForwardFill => '\u{f3db}',
        Bootstrap::FourCircle => '\u{f7a9}',
        Bootstrap::FourCircleFill => '\u{f7a8}',
        Bootstrap::FourSquare => '\u{f7ab}',
        Bootstrap::FourSquareFill => '\u{f7aa}',
        Bootstrap::Front => '\u{f3dd}',
        Bootstrap::FuelPump => '\u{f83e}',
        Bootstrap::FuelPumpDiesel => '\u{f83c}',
        Bootstrap::FuelPumpDieselFill => '\u{f83b}',
        Bootstrap::FuelPumpFill => '\u{f83d}',
        Bootstrap::Fullscreen => '\u{f3df}',
        Bootstrap::FullscreenExit => '\u{f3de}',
        Bootstrap::Funnel => '\u{f3e1}',
        Bootstrap::FunnelFill => '\u{f3e0}',
        Bootstrap::Gear => '\u{f3e5}',
        Bootstrap::GearFill => '\u{f3e2}',
        Bootstrap::GearWide => '\u{f3e4}',
        Bootstrap::GearWideConnected => '\u{f3e3}',
        Bootstrap::Gem => '\u{f3e6}',
        Bootstrap::GenderAmbiguous => '\u{f641}',
        Bootstrap::GenderFemale => '\u{f642}',
        Bootstrap::GenderMale => '\u{f643}',
        Bootstrap::GenderNeuter => '\u{f8ed}',
        Bootstrap::GenderTrans => '\u{f644}',
        Bootstrap::Geo => '\u{f3ea}',
        Bootstrap::GeoAlt => '\u{f3e8}',
        Bootstrap::GeoAltFill => '\u{f3e7}',
        Bootstrap::GeoFill => '\u{f3e9}',
        Bootstrap::Gift => '\u{f3ec}',
        Bootstrap::GiftFill => '\u{f3eb}',
        Bootstrap::Git => '\u{f69d}',
        Bootstrap::Github => '\u{f3ed}',
        Bootstrap::Gitlab => '\u{f7e5}',
        Bootstrap::Globe => '\u{f3ee}',
        Bootstrap::GlobeAmericas => '\u{f882}',
        Bootstrap::GlobeAsiaAustralia => '\u{f883}',
        Bootstrap::GlobeCentralSouthAsia => '\u{f884}',
        Bootstrap::GlobeEuropeAfrica => '\u{f885}',
        Bootstrap::Globetwo => '\u{f3ef}',
        Bootstrap::Google => '\u{f3f0}',
        Bootstrap::GooglePlay => '\u{f7f7}',
        Bootstrap::GpuCard => '\u{f6e2}',
        Bootstrap::GraphDown => '\u{f3f1}',
        Bootstrap::GraphDownArrow => '\u{f672}',
        Bootstrap::GraphUp => '\u{f3f2}',
        Bootstrap::GraphUpArrow => '\u{f673}',
        Bootstrap::Grid => '\u{f3fc}',
        Bootstrap::GridFill => '\u{f3fb}',
        Bootstrap::GridOnextwo => '\u{f3f4}',
        Bootstrap::GridOnextwoFill => '\u{f3f3}',
        Bootstrap::GridThreexthree => '\u{f3fa}',
        Bootstrap::GridThreexthreeGap => '\u{f3f9}',
        Bootstrap::GridThreexthreeGapFill => '\u{f3f8}',
        Bootstrap::GridThreextwo => '\u{f3f7}',
        Bootstrap::GridThreextwoGap => '\u{f3f6}',
        Bootstrap::GridThreextwoGapFill => '\u{f3f5}',
        Bootstrap::GripHorizontal => '\u{f3fd}',
        Bootstrap::GripVertical => '\u{f3fe}',
        Bootstrap::HCircle => '\u{f7fb}',
        Bootstrap::HCircleFill => '\u{f7fa}',
        Bootstrap::HSquare => '\u{f7fd}',
        Bootstrap::HSquareFill => '\u{f7fc}',
        Bootstrap::Hammer => '\u{f3ff}',
        Bootstrap::HandIndex => '\u{f403}',
        Bootstrap::HandIndexFill => '\u{f400}',
        Bootstrap::HandIndexThumb => '\u{f402}',
        Bootstrap::HandIndexThumbFill => '\u{f401}',
        Bootstrap::HandThumbsDown => '\u{f405}',
        Bootstrap::HandThumbsDownFill => '\u{f404}',
        Bootstrap::HandThumbsUp => '\u{f407}',
        Bootstrap::HandThumbsUpFill => '\u{f406}',
        Bootstrap::Handbag => '\u{f409}',
        Bootstrap::HandbagFill => '\u{f408}',
        Bootstrap::Hash => '\u{f40a}',
        Bootstrap::Hdd => '\u{f412}',
        Bootstrap::HddFill => '\u{f40b}',
        Bootstrap::HddNetwork => '\u{f40d}',
        Bootstrap::HddNetworkFill => '\u{f40c}',
        Bootstrap::HddRack => '\u{f40f}',
        Bootstrap::HddRackFill => '\u{f40e}',
        Bootstrap::HddStack => '\u{f411}',
        Bootstrap::HddStackFill => '\u{f410}',
        Bootstrap::Hdmi => '\u{f6d7}',
        Bootstrap::HdmiFill => '\u{f6d6}',
        Bootstrap::Headphones => '\u{f413}',
        Bootstrap::Headset => '\u{f414}',
        Bootstrap::HeadsetVr => '\u{f645}',
        Bootstrap::Heart => '\u{f417}',
        Bootstrap::HeartArrow => '\u{f76d}',
        Bootstrap::HeartFill => '\u{f415}',
        Bootstrap::HeartHalf => '\u{f416}',
        Bootstrap::HeartPulse => '\u{f76f}',
        Bootstrap::HeartPulseFill => '\u{f76e}',
        Bootstrap::Heartbreak => '\u{f771}',
        Bootstrap::HeartbreakFill => '\u{f770}',
        Bootstrap::Hearts => '\u{f772}',
        Bootstrap::Heptagon => '\u{f41a}',
        Bootstrap::HeptagonFill => '\u{f418}',
        Bootstrap::HeptagonHalf => '\u{f419}',
        Bootstrap::Hexagon => '\u{f41d}',
        Bootstrap::HexagonFill => '\u{f41b}',
        Bootstrap::HexagonHalf => '\u{f41c}',
        Bootstrap::Highlighter => '\u{f7f8}',
        Bootstrap::Highlights => '\u{f8ee}',
        Bootstrap::Hospital => '\u{f774}',
        Bootstrap::HospitalFill => '\u{f773}',
        Bootstrap::Hourglass => '\u{f421}',
        Bootstrap::HourglassBottom => '\u{f41e}',
        Bootstrap::HourglassSplit => '\u{f41f}',
        Bootstrap::HourglassTop => '\u{f420}',
        Bootstrap::House => '\u{f425}',
        Bootstrap::HouseAdd => '\u{f887}',
        Bootstrap::HouseAddFill => '\u{f886}',
        Bootstrap::HouseCheck => '\u{f889}',
        Bootstrap::HouseCheckFill => '\u{f888}',
        Bootstrap::HouseDash => '\u{f88b}',
        Bootstrap::HouseDashFill => '\u{f88a}',
        Bootstrap::HouseDoor => '\u{f423}',
        Bootstrap::HouseDoorFill => '\u{f422}',
        Bootstrap::HouseDown => '\u{f88d}',
        Bootstrap::HouseDownFill => '\u{f88c}',
        Bootstrap::HouseExclamation => '\u{f88f}',
        Bootstrap::HouseExclamationFill => '\u{f88e}',
        Bootstrap::HouseFill => '\u{f424}',
        Bootstrap::HouseGear => '\u{f891}',
        Bootstrap::HouseGearFill => '\u{f890}',
        Bootstrap::HouseHeart => '\u{f776}',
        Bootstrap::HouseHeartFill => '\u{f775}',
        Bootstrap::HouseLock => '\u{f893}',
        Bootstrap::HouseLockFill => '\u{f892}',
        Bootstrap::HouseSlash => '\u{f895}',
        Bootstrap::HouseSlashFill => '\u{f894}',
        Bootstrap::HouseUp => '\u{f897}',
        Bootstrap::HouseUpFill => '\u{f896}',
        Bootstrap::HouseX => '\u{f899}',
        Bootstrap::HouseXFill => '\u{f898}',
        Bootstrap::Houses => '\u{f8c6}',
        Bootstrap::HousesFill => '\u{f8c5}',
        Bootstrap::Hr => '\u{f426}',
        Bootstrap::Hurricane => '\u{f427}',
        Bootstrap::Hypnotize => '\u{f674}',
        Bootstrap::Image => '\u{f42a}',
        Bootstrap::ImageAlt => '\u{f428}',
        Bootstrap::ImageFill => '\u{f429}',
        Bootstrap::Images => '\u{f42b}',
        Bootstrap::Inbox => '\u{f42d}',
        Bootstrap::InboxFill => '\u{f42c}',
        Bootstrap::Inboxes => '\u{f42f}',
        Bootstrap::InboxesFill => '\u{f42e}',
        Bootstrap::Incognito => '\u{f777}',
        Bootstrap::Indent => '\u{f7fe}',
        Bootstrap::Infinity => '\u{f69e}',
        Bootstrap::Info => '\u{f434}',
        Bootstrap::InfoCircle => '\u{f431}',
        Bootstrap::InfoCircleFill => '\u{f430}',
        Bootstrap::InfoLg => '\u{f646}',
        Bootstrap::InfoSquare => '\u{f433}',
        Bootstrap::InfoSquareFill => '\u{f432}',
        Bootstrap::InputCursor => '\u{f436}',
        Bootstrap::InputCursorText => '\u{f435}',
        Bootstrap::Instagram => '\u{f437}',
        Bootstrap::Intersect => '\u{f438}',
        Bootstrap::Journal => '\u{f446}',
        Bootstrap::JournalAlbum => '\u{f439}',
        Bootstrap::JournalArrowDown => '\u{f43a}',
        Bootstrap::JournalArrowUp => '\u{f43b}',
        Bootstrap::JournalBookmark => '\u{f43d}',
        Bootstrap::JournalBookmarkFill => '\u{f43c}',
        Bootstrap::JournalCheck => '\u{f43e}',
        Bootstrap::JournalCode => '\u{f43f}',
        Bootstrap::JournalMedical => '\u{f440}',
        Bootstrap::JournalMinus => '\u{f441}',
        Bootstrap::JournalPlus => '\u{f442}',
        Bootstrap::JournalRichtext => '\u{f443}',
        Bootstrap::JournalText => '\u{f444}',
        Bootstrap::JournalX => '\u{f445}',
        Bootstrap::Journals => '\u{f447}',
        Bootstrap::Joystick => '\u{f448}',
        Bootstrap::Justify => '\u{f44b}',
        Bootstrap::JustifyLeft => '\u{f449}',
        Bootstrap::JustifyRight => '\u{f44a}',
        Bootstrap::Kanban => '\u{f44d}',
        Bootstrap::KanbanFill => '\u{f44c}',
        Bootstrap::Key => '\u{f44f}',
        Bootstrap::KeyFill => '\u{f44e}',
        Bootstrap::Keyboard => '\u{f451}',
        Bootstrap::KeyboardFill => '\u{f450}',
        Bootstrap::Ladder => '\u{f452}',
        Bootstrap::Lamp => '\u{f454}',
        Bootstrap::LampFill => '\u{f453}',
        Bootstrap::Laptop => '\u{f456}',
        Bootstrap::LaptopFill => '\u{f455}',
        Bootstrap::LayerBackward => '\u{f457}',
        Bootstrap::LayerForward => '\u{f458}',
        Bootstrap::Layers => '\u{f45b}',
        Bootstrap::LayersFill => '\u{f459}',
        Bootstrap::LayersHalf => '\u{f45a}',
        Bootstrap::LayoutSidebar => '\u{f45f}',
        Bootstrap::LayoutSidebarInset => '\u{f45d}',
        Bootstrap::LayoutSidebarInsetReverse => '\u{f45c}',
        Bootstrap::LayoutSidebarReverse => '\u{f45e}',
        Bootstrap::LayoutSplit => '\u{f460}',
        Bootstrap::LayoutTextSidebar => '\u{f462}',
        Bootstrap::LayoutTextSidebarReverse => '\u{f461}',
        Bootstrap::LayoutTextWindow => '\u{f464}',
        Bootstrap::LayoutTextWindowReverse => '\u{f463}',
        Bootstrap::LayoutThreeColumns => '\u{f465}',
        Bootstrap::LayoutWtf => '\u{f466}',
        Bootstrap::LifePreserver => '\u{f467}',
        Bootstrap::Lightbulb => '\u{f46b}',
        Bootstrap::LightbulbFill => '\u{f468}',
        Bootstrap::LightbulbOff => '\u{f46a}',
        Bootstrap::LightbulbOffFill => '\u{f469}',
        Bootstrap::Lightning => '\u{f46f}',
        Bootstrap::LightningCharge => '\u{f46d}',
        Bootstrap::LightningChargeFill => '\u{f46c}',
        Bootstrap::LightningFill => '\u{f46e}',
        Bootstrap::Line => '\u{f660}',
        Bootstrap::Link => '\u{f471}',
        Bootstrap::LinkFourfivedeg => '\u{f470}',
        Bootstrap::Linkedin => '\u{f472}',
        Bootstrap::List => '\u{f479}',
        Bootstrap::ListCheck => '\u{f473}',
        Bootstrap::ListColumns => '\u{f6a0}',
        Bootstrap::ListColumnsReverse => '\u{f69f}',
        Bootstrap::ListNested => '\u{f474}',
        Bootstrap::ListOl => '\u{f475}',
        Bootstrap::ListStars => '\u{f476}',
        Bootstrap::ListTask => '\u{f477}',
        Bootstrap::ListUl => '\u{f478}',
        Bootstrap::Lock => '\u{f47b}',
        Bootstrap::LockFill => '\u{f47a}',
        Bootstrap::Luggage => '\u{f8f0}',
        Bootstrap::LuggageFill => '\u{f8ef}',
        Bootstrap::Lungs => '\u{f800}',
        Bootstrap::LungsFill => '\u{f7ff}',
        Bootstrap::Magic => '\u{f675}',
        Bootstrap::Magnet => '\u{f779}',
        Bootstrap::MagnetFill => '\u{f778}',
        Bootstrap::Mailbox => '\u{f47c}',
        Bootstrap::MailboxFlag => '\u{f8f1}',
        Bootstrap::Mailboxtwo => '\u{f47d}',
        Bootstrap::MailboxtwoFlag => '\u{f8f2}',
        Bootstrap::Map => '\u{f47f}',
        Bootstrap::MapFill => '\u{f47e}',
        Bootstrap::Markdown => '\u{f481}',
        Bootstrap::MarkdownFill => '\u{f480}',
        Bootstrap::MarkerTip => '\u{f802}',
        Bootstrap::Mask => '\u{f482}',
        Bootstrap::Mastodon => '\u{f647}',
        Bootstrap::Medium => '\u{f661}',
        Bootstrap::Megaphone => '\u{f484}',
        Bootstrap::MegaphoneFill => '\u{f483}',
        Bootstrap::Memory => '\u{f6e3}',
        Bootstrap::MenuApp => '\u{f486}',
        Bootstrap::MenuAppFill => '\u{f485}',
        Bootstrap::MenuButton => '\u{f48a}',
        Bootstrap::MenuButtonFill => '\u{f487}',
        Bootstrap::MenuButtonWide => '\u{f489}',
        Bootstrap::MenuButtonWideFill => '\u{f488}',
        Bootstrap::MenuDown => '\u{f48b}',
        Bootstrap::MenuUp => '\u{f48c}',
        Bootstrap::Messenger => '\u{f648}',
        Bootstrap::Meta => '\u{f6a1}',
        Bootstrap::Mic => '\u{f490}',
        Bootstrap::MicFill => '\u{f48d}',
        Bootstrap::MicMute => '\u{f48f}',
        Bootstrap::MicMuteFill => '\u{f48e}',
        Bootstrap::Microsoft => '\u{f65d}',
        Bootstrap::MicrosoftTeams => '\u{f801}',
        Bootstrap::Minecart => '\u{f492}',
        Bootstrap::MinecartLoaded => '\u{f491}',
        Bootstrap::Modem => '\u{f6e5}',
        Bootstrap::ModemFill => '\u{f6e4}',
        Bootstrap::Moisture => '\u{f493}',
        Bootstrap::Moon => '\u{f497}',
        Bootstrap::MoonFill => '\u{f494}',
        Bootstrap::MoonStars => '\u{f496}',
        Bootstrap::MoonStarsFill => '\u{f495}',
        Bootstrap::Mortarboard => '\u{f6fe}',
        Bootstrap::MortarboardFill => '\u{f6fd}',
        Bootstrap::Motherboard => '\u{f6e7}',
        Bootstrap::MotherboardFill => '\u{f6e6}',
        Bootstrap::Mouse => '\u{f499}',
        Bootstrap::MouseFill => '\u{f498}',
        Bootstrap::Mousethree => '\u{f49d}',
        Bootstrap::MousethreeFill => '\u{f49c}',
        Bootstrap::Mousetwo => '\u{f49b}',
        Bootstrap::MousetwoFill => '\u{f49a}',
        Bootstrap::MusicNote => '\u{f4a0}',
        Bootstrap::MusicNoteBeamed => '\u{f49e}',
        Bootstrap::MusicNoteList => '\u{f49f}',
        Bootstrap::MusicPlayer => '\u{f4a2}',
        Bootstrap::MusicPlayerFill => '\u{f4a1}',
        Bootstrap::Newspaper => '\u{f4a3}',
        Bootstrap::NineCircle => '\u{f7c7}',
        Bootstrap::NineCircleFill => '\u{f7c6}',
        Bootstrap::NineSquare => '\u{f7c9}',
        Bootstrap::NineSquareFill => '\u{f7c8}',
        Bootstrap::NintendoSwitch => '\u{f6a4}',
        Bootstrap::NodeMinus => '\u{f4a5}',
        Bootstrap::NodeMinusFill => '\u{f4a4}',
        Bootstrap::NodePlus => '\u{f4a7}',
        Bootstrap::NodePlusFill => '\u{f4a6}',
        Bootstrap::NoiseReduction => '\u{f8f3}',
        Bootstrap::Nut => '\u{f4a9}',
        Bootstrap::NutFill => '\u{f4a8}',
        Bootstrap::Nvidia => '\u{f8c7}',
        Bootstrap::Nvme => '\u{f80c}',
        Bootstrap::NvmeFill => '\u{f803}',
        Bootstrap::Octagon => '\u{f4ac}',
        Bootstrap::OctagonFill => '\u{f4aa}',
        Bootstrap::OctagonHalf => '\u{f4ab}',
        Bootstrap::OneCircle => '\u{f797}',
        Bootstrap::OneCircleFill => '\u{f796}',
        Bootstrap::OneSquare => '\u{f799}',
        Bootstrap::OneSquareFill => '\u{f798}',
        Bootstrap::Onetwothree => '\u{f67f}',
        Bootstrap::Opencollective => '\u{f80d}',
        Bootstrap::OpticalAudio => '\u{f6e9}',
        Bootstrap::OpticalAudioFill => '\u{f6e8}',
        Bootstrap::Option => '\u{f4ad}',
        Bootstrap::Outlet => '\u{f4ae}',
        Bootstrap::PCircle => '\u{f805}',
        Bootstrap::PCircleFill => '\u{f804}',
        Bootstrap::PSquare => '\u{f807}',
        Bootstrap::PSquareFill => '\u{f806}',
        Bootstrap::PaintBucket => '\u{f4af}',
        Bootstrap::Palette => '\u{f4b1}',
        Bootstrap::PaletteFill => '\u{f4b0}',
        Bootstrap::Palettetwo => '\u{f4b2}',
        Bootstrap::Paperclip => '\u{f4b3}',
        Bootstrap::Paragraph => '\u{f4b4}',
        Bootstrap::Pass => '\u{f809}',
        Bootstrap::PassFill => '\u{f808}',
        Bootstrap::Passport => '\u{f8f5}',
        Bootstrap::PassportFill => '\u{f8f4}',
        Bootstrap::PatchCheck => '\u{f4b6}',
        Bootstrap::PatchCheckFill => '\u{f4b5}',
        Bootstrap::PatchExclamation => '\u{f4b8}',
        Bootstrap::PatchExclamationFill => '\u{f4b7}',
        Bootstrap::PatchMinus => '\u{f4ba}',
        Bootstrap::PatchMinusFill => '\u{f4b9}',
        Bootstrap::PatchPlus => '\u{f4bc}',
        Bootstrap::PatchPlusFill => '\u{f4bb}',
        Bootstrap::PatchQuestion => '\u{f4be}',
        Bootstrap::PatchQuestionFill => '\u{f4bd}',
        Bootstrap::Pause => '\u{f4c4}',
        Bootstrap::PauseBtn => '\u{f4c0}',
        Bootstrap::PauseBtnFill => '\u{f4bf}',
        Bootstrap::PauseCircle => '\u{f4c2}',
        Bootstrap::PauseCircleFill => '\u{f4c1}',
        Bootstrap::PauseFill => '\u{f4c3}',
        Bootstrap::Paypal => '\u{f662}',
        Bootstrap::Pc => '\u{f6a8}',
        Bootstrap::PcDisplay => '\u{f6a6}',
        Bootstrap::PcDisplayHorizontal => '\u{f6a5}',
        Bootstrap::PcHorizontal => '\u{f6a7}',
        Bootstrap::PciCard => '\u{f6ea}',
        Bootstrap::PciCardNetwork => '\u{f8cd}',
        Bootstrap::PciCardSound => '\u{f8ce}',
        Bootstrap::Peace => '\u{f4c6}',
        Bootstrap::PeaceFill => '\u{f4c5}',
        Bootstrap::Pen => '\u{f4c8}',
        Bootstrap::PenFill => '\u{f4c7}',
        Bootstrap::Pencil => '\u{f4cb}',
        Bootstrap::PencilFill => '\u{f4c9}',
        Bootstrap::PencilSquare => '\u{f4ca}',
        Bootstrap::Pentagon => '\u{f4ce}',
        Bootstrap::PentagonFill => '\u{f4cc}',
        Bootstrap::PentagonHalf => '\u{f4cd}',
        Bootstrap::People => '\u{f4d0}',
        Bootstrap::PeopleFill => '\u{f4cf}',
        Bootstrap::Percent => '\u{f4d1}',
        Bootstrap::Person => '\u{f4e1}',
        Bootstrap::PersonAdd => '\u{f89a}',
        Bootstrap::PersonArmsUp => '\u{f8f6}',
        Bootstrap::PersonBadge => '\u{f4d3}',
        Bootstrap::PersonBadgeFill => '\u{f4d2}',
        Bootstrap::PersonBoundingBox => '\u{f4d4}',
        Bootstrap::PersonCheck => '\u{f4d6}',
        Bootstrap::PersonCheckFill => '\u{f4d5}',
        Bootstrap::PersonCircle => '\u{f4d7}',
        Bootstrap::PersonDash => '\u{f4d9}',
        Bootstrap::PersonDashFill => '\u{f4d8}',
        Bootstrap::PersonDown => '\u{f89b}',
        Bootstrap::PersonExclamation => '\u{f89c}',
        Bootstrap::PersonFill => '\u{f4da}',
        Bootstrap::PersonFillAdd => '\u{f89d}',
        Bootstrap::PersonFillCheck => '\u{f89e}',
        Bootstrap::PersonFillDash => '\u{f89f}',
        Bootstrap::PersonFillDown => '\u{f8a0}',
        Bootstrap::PersonFillExclamation => '\u{f8a1}',
        Bootstrap::PersonFillGear => '\u{f8a2}',
        Bootstrap::PersonFillLock => '\u{f8a3}',
        Bootstrap::PersonFillSlash => '\u{f8a4}',
        Bootstrap::PersonFillUp => '\u{f8a5}',
        Bootstrap::PersonFillX => '\u{f8a6}',
        Bootstrap::PersonGear => '\u{f8a7}',
        Bootstrap::PersonHeart => '\u{f77a}',
        Bootstrap::PersonHearts => '\u{f77b}',
        Bootstrap::PersonLinesFill => '\u{f4db}',
        Bootstrap::PersonLock => '\u{f8a8}',
        Bootstrap::PersonPlus => '\u{f4dd}',
        Bootstrap::PersonPlusFill => '\u{f4dc}',
        Bootstrap::PersonRaisedHand => '\u{f8f7}',
        Bootstrap::PersonRolodex => '\u{f676}',
        Bootstrap::PersonSlash => '\u{f8a9}',
        Bootstrap::PersonSquare => '\u{f4de}',
        Bootstrap::PersonStanding => '\u{f8f9}',
        Bootstrap::PersonStandingDress => '\u{f8f8}',
        Bootstrap::PersonUp => '\u{f8aa}',
        Bootstrap::PersonVcard => '\u{f8c9}',
        Bootstrap::PersonVcardFill => '\u{f8c8}',
        Bootstrap::PersonVideo => '\u{f677}',
        Bootstrap::PersonVideothree => '\u{f679}',
        Bootstrap::PersonVideotwo => '\u{f678}',
        Bootstrap::PersonWalking => '\u{f8fa}',
        Bootstrap::PersonWheelchair => '\u{f8fb}',
        Bootstrap::PersonWorkspace => '\u{f67a}',
        Bootstrap::PersonX => '\u{f4e0}',
        Bootstrap::PersonXFill => '\u{f4df}',
        Bootstrap::Phone => '\u{f4e7}',
        Bootstrap::PhoneFill => '\u{f4e2}',
        Bootstrap::PhoneFlip => '\u{f77c}',
        Bootstrap::PhoneLandscape => '\u{f4e4}',
        Bootstrap::PhoneLandscapeFill => '\u{f4e3}',
        Bootstrap::PhoneVibrate => '\u{f4e6}',
        Bootstrap::PhoneVibrateFill => '\u{f4e5}',
        Bootstrap::PieChart => '\u{f4e9}',
        Bootstrap::PieChartFill => '\u{f4e8}',
        Bootstrap::PiggyBank => '\u{f64a}',
        Bootstrap::PiggyBankFill => '\u{f649}',
        Bootstrap::Pin => '\u{f4ed}',
        Bootstrap::PinAngle => '\u{f4eb}',
        Bootstrap::PinAngleFill => '\u{f4ea}',
        Bootstrap::PinFill => '\u{f4ec}',
        Bootstrap::PinMap => '\u{f64c}',
        Bootstrap::PinMapFill => '\u{f64b}',
        Bootstrap::Pinterest => '\u{f663}',
        Bootstrap::Pip => '\u{f4ef}',
        Bootstrap::PipFill => '\u{f4ee}',
        Bootstrap::Play => '\u{f4f5}',
        Bootstrap::PlayBtn => '\u{f4f1}',
        Bootstrap::PlayBtnFill => '\u{f4f0}',
        Bootstrap::PlayCircle => '\u{f4f3}',
        Bootstrap::PlayCircleFill => '\u{f4f2}',
        Bootstrap::PlayFill => '\u{f4f4}',
        Bootstrap::Playstation => '\u{f6a9}',
        Bootstrap::Plug => '\u{f4f7}',
        Bootstrap::PlugFill => '\u{f4f6}',
        Bootstrap::Plugin => '\u{f77d}',
        Bootstrap::Plus => '\u{f4fe}',
        Bootstrap::PlusCircle => '\u{f4fa}',
        Bootstrap::PlusCircleDotted => '\u{f4f8}',
        Bootstrap::PlusCircleFill => '\u{f4f9}',
        Bootstrap::PlusLg => '\u{f64d}',
        Bootstrap::PlusSlashMinus => '\u{f6aa}',
        Bootstrap::PlusSquare => '\u{f4fd}',
        Bootstrap::PlusSquareDotted => '\u{f4fb}',
        Bootstrap::PlusSquareFill => '\u{f4fc}',
        Bootstrap::Postage => '\u{f781}',
        Bootstrap::PostageFill => '\u{f77e}',
        Bootstrap::PostageHeart => '\u{f780}',
        Bootstrap::PostageHeartFill => '\u{f77f}',
        Bootstrap::Postcard => '\u{f785}',
        Bootstrap::PostcardFill => '\u{f782}',
        Bootstrap::PostcardHeart => '\u{f784}',
        Bootstrap::PostcardHeartFill => '\u{f783}',
        Bootstrap::Power => '\u{f4ff}',
        Bootstrap::Prescription => '\u{f80a}',
        Bootstrap::Prescriptiontwo => '\u{f80b}',
        Bootstrap::Printer => '\u{f501}',
        Bootstrap::PrinterFill => '\u{f500}',
        Bootstrap::Projector => '\u{f6ac}',
        Bootstrap::ProjectorFill => '\u{f6ab}',
        Bootstrap::Puzzle => '\u{f503}',
        Bootstrap::PuzzleFill => '\u{f502}',
        Bootstrap::QrCode => '\u{f6ae}',
        Bootstrap::QrCodeScan => '\u{f6ad}',
        Bootstrap::Question => '\u{f50c}',
        Bootstrap::QuestionCircle => '\u{f505}',
        Bootstrap::QuestionCircleFill => '\u{f504}',
        Bootstrap::QuestionDiamond => '\u{f507}',
        Bootstrap::QuestionDiamondFill => '\u{f506}',
        Bootstrap::QuestionLg => '\u{f64e}',
        Bootstrap::QuestionOctagon => '\u{f509}',
        Bootstrap::QuestionOctagonFill => '\u{f508}',
        Bootstrap::QuestionSquare => '\u{f50b}',
        Bootstrap::QuestionSquareFill => '\u{f50a}',
        Bootstrap::Quora => '\u{f6af}',
        Bootstrap::Quote => '\u{f6b0}',
        Bootstrap::RCircle => '\u{f80f}',
        Bootstrap::RCircleFill => '\u{f80e}',
        Bootstrap::RSquare => '\u{f811}',
        Bootstrap::RSquareFill => '\u{f810}',
        Bootstrap::Radar => '\u{f8cf}',
        Bootstrap::Radioactive => '\u{f67b}',
        Bootstrap::Rainbow => '\u{f50d}',
        Bootstrap::Receipt => '\u{f50f}',
        Bootstrap::ReceiptCutoff => '\u{f50e}',
        Bootstrap::ReceptionFour => '\u{f514}',
        Bootstrap::ReceptionOne => '\u{f511}',
        Bootstrap::ReceptionThree => '\u{f513}',
        Bootstrap::ReceptionTwo => '\u{f512}',
        Bootstrap::ReceptionZero => '\u{f510}',
        Bootstrap::Record => '\u{f51a}',
        Bootstrap::RecordBtn => '\u{f516}',
        Bootstrap::RecordBtnFill => '\u{f515}',
        Bootstrap::RecordCircle => '\u{f518}',
        Bootstrap::RecordCircleFill => '\u{f517}',
        Bootstrap::RecordFill => '\u{f519}',
        Bootstrap::Recordtwo => '\u{f51c}',
        Bootstrap::RecordtwoFill => '\u{f51b}',
        Bootstrap::Recycle => '\u{f64f}',
        Bootstrap::Reddit => '\u{f650}',
        Bootstrap::Regex => '\u{f84d}',
        Bootstrap::Repeat => '\u{f813}',
        Bootstrap::RepeatOne => '\u{f812}',
        Bootstrap::Reply => '\u{f520}',
        Bootstrap::ReplyAll => '\u{f51e}',
        Bootstrap::ReplyAllFill => '\u{f51d}',
        Bootstrap::ReplyFill => '\u{f51f}',
        Bootstrap::Rewind => '\u{f819}',
        Bootstrap::RewindBtn => '\u{f815}',
        Bootstrap::RewindBtnFill => '\u{f814}',
        Bootstrap::RewindCircle => '\u{f817}',
        Bootstrap::RewindCircleFill => '\u{f816}',
        Bootstrap::RewindFill => '\u{f818}',
        Bootstrap::Robot => '\u{f6b1}',
        Bootstrap::Rocket => '\u{f846}',
        Bootstrap::RocketFill => '\u{f843}',
        Bootstrap::RocketTakeoff => '\u{f845}',
        Bootstrap::RocketTakeoffFill => '\u{f844}',
        Bootstrap::Router => '\u{f6ec}',
        Bootstrap::RouterFill => '\u{f6eb}',
        Bootstrap::Rss => '\u{f522}',
        Bootstrap::RssFill => '\u{f521}',
        Bootstrap::Rulers => '\u{f523}',
        Bootstrap::Safe => '\u{f65a}',
        Bootstrap::SafeFill => '\u{f651}',
        Bootstrap::Safetwo => '\u{f653}',
        Bootstrap::SafetwoFill => '\u{f652}',
        Bootstrap::Save => '\u{f525}',
        Bootstrap::SaveFill => '\u{f524}',
        Bootstrap::Savetwo => '\u{f527}',
        Bootstrap::SavetwoFill => '\u{f526}',
        Bootstrap::Scissors => '\u{f528}',
        Bootstrap::Scooter => '\u{f8ab}',
        Bootstrap::Screwdriver => '\u{f529}',
        Bootstrap::SdCard => '\u{f655}',
        Bootstrap::SdCardFill => '\u{f654}',
        Bootstrap::Search => '\u{f52a}',
        Bootstrap::SearchHeart => '\u{f787}',
        Bootstrap::SearchHeartFill => '\u{f786}',
        Bootstrap::SegmentedNav => '\u{f52b}',
        Bootstrap::Send => '\u{f6c0}',
        Bootstrap::SendArrowDown => '\u{f8d1}',
        Bootstrap::SendArrowDownFill => '\u{f8d0}',
        Bootstrap::SendArrowUp => '\u{f8d3}',
        Bootstrap::SendArrowUpFill => '\u{f8d2}',
        Bootstrap::SendCheck => '\u{f6b3}',
        Bootstrap::SendCheckFill => '\u{f6b2}',
        Bootstrap::SendDash => '\u{f6b5}',
        Bootstrap::SendDashFill => '\u{f6b4}',
        Bootstrap::SendExclamation => '\u{f6b8}',
        Bootstrap::SendExclamationFill => '\u{f6b7}',
        Bootstrap::SendFill => '\u{f6b9}',
        Bootstrap::SendPlus => '\u{f6bb}',
        Bootstrap::SendPlusFill => '\u{f6ba}',
        Bootstrap::SendSlash => '\u{f6bd}',
        Bootstrap::SendSlashFill => '\u{f6bc}',
        Bootstrap::SendX => '\u{f6bf}',
        Bootstrap::SendXFill => '\u{f6be}',
        Bootstrap::Server => '\u{f52c}',
        Bootstrap::SevenCircle => '\u{f7bb}',
        Bootstrap::SevenCircleFill => '\u{f7ba}',
        Bootstrap::SevenSquare => '\u{f7bd}',
        Bootstrap::SevenSquareFill => '\u{f7bc}',
        Bootstrap::Shadows => '\u{f8fc}',
        Bootstrap::Share => '\u{f52e}',
        Bootstrap::ShareFill => '\u{f52d}',
        Bootstrap::Shield => '\u{f53f}',
        Bootstrap::ShieldCheck => '\u{f52f}',
        Bootstrap::ShieldExclamation => '\u{f530}',
        Bootstrap::ShieldFill => '\u{f536}',
        Bootstrap::ShieldFillCheck => '\u{f531}',
        Bootstrap::ShieldFillExclamation => '\u{f532}',
        Bootstrap::ShieldFillMinus => '\u{f533}',
        // Bootspub mod bootrap::ShieldFillPlus => '\u{f534}',
        Bootstrap::ShieldFillX => '\u{f535}',
        Bootstrap::ShieldLock => '\u{f538}',
        Bootstrap::ShieldLockFill => '\u{f537}',
        Bootstrap::ShieldMinus => '\u{f539}',
        Bootstrap::ShieldPlus => '\u{f53a}',
        Bootstrap::ShieldShaded => '\u{f53b}',
        Bootstrap::ShieldSlash => '\u{f53d}',
        Bootstrap::ShieldSlashFill => '\u{f53c}',
        Bootstrap::ShieldX => '\u{f53e}',
        Bootstrap::Shift => '\u{f541}',
        Bootstrap::ShiftFill => '\u{f540}',
        Bootstrap::Shop => '\u{f543}',
        Bootstrap::ShopWindow => '\u{f542}',
        Bootstrap::Shuffle => '\u{f544}',
        Bootstrap::SignDeadEnd => '\u{f850}',
        Bootstrap::SignDeadEndFill => '\u{f84f}',
        Bootstrap::SignDoNotEnter => '\u{f852}',
        Bootstrap::SignDoNotEnterFill => '\u{f851}',
        Bootstrap::SignIntersection => '\u{f85a}',
        Bootstrap::SignIntersectionFill => '\u{f853}',
        Bootstrap::SignIntersectionSide => '\u{f855}',
        Bootstrap::SignIntersectionSideFill => '\u{f854}',
        Bootstrap::SignIntersectionT => '\u{f857}',
        Bootstrap::SignIntersectionTFill => '\u{f856}',
        Bootstrap::SignIntersectionY => '\u{f859}',
        Bootstrap::SignIntersectionYFill => '\u{f858}',
        Bootstrap::SignMergeLeft => '\u{f85c}',
        Bootstrap::SignMergeLeftFill => '\u{f85b}',
        Bootstrap::SignMergeRight => '\u{f85e}',
        Bootstrap::SignMergeRightFill => '\u{f85d}',
        Bootstrap::SignNoLeftTurn => '\u{f860}',
        Bootstrap::SignNoLeftTurnFill => '\u{f85f}',
        Bootstrap::SignNoParking => '\u{f862}',
        Bootstrap::SignNoParkingFill => '\u{f861}',
        Bootstrap::SignNoRightTurn => '\u{f864}',
        Bootstrap::SignNoRightTurnFill => '\u{f863}',
        Bootstrap::SignRailroad => '\u{f866}',
        Bootstrap::SignRailroadFill => '\u{f865}',
        Bootstrap::SignStop => '\u{f82e}',
        Bootstrap::SignStopFill => '\u{f82b}',
        Bootstrap::SignStopLights => '\u{f82d}',
        Bootstrap::SignStopLightsFill => '\u{f82c}',
        Bootstrap::SignTurnLeft => '\u{f830}',
        Bootstrap::SignTurnLeftFill => '\u{f82f}',
        Bootstrap::SignTurnRight => '\u{f832}',
        Bootstrap::SignTurnRightFill => '\u{f831}',
        Bootstrap::SignTurnSlightLeft => '\u{f834}',
        Bootstrap::SignTurnSlightLeftFill => '\u{f833}',
        Bootstrap::SignTurnSlightRight => '\u{f836}',
        Bootstrap::SignTurnSlightRightFill => '\u{f835}',
        Bootstrap::SignYield => '\u{f838}',
        Bootstrap::SignYieldFill => '\u{f837}',
        Bootstrap::Signal => '\u{f664}',
        Bootstrap::Signpost => '\u{f54a}',
        Bootstrap::SignpostFill => '\u{f547}',
        Bootstrap::SignpostSplit => '\u{f549}',
        Bootstrap::SignpostSplitFill => '\u{f548}',
        Bootstrap::SignpostTwo => '\u{f546}',
        Bootstrap::SignpostTwoFill => '\u{f545}',
        Bootstrap::Sim => '\u{f54c}',
        Bootstrap::SimFill => '\u{f54b}',
        Bootstrap::SimSlash => '\u{f8d5}',
        Bootstrap::SimSlashFill => '\u{f8d4}',
        Bootstrap::SinaWeibo => '\u{f8ca}',
        Bootstrap::SixCircle => '\u{f7b5}',
        Bootstrap::SixCircleFill => '\u{f7b4}',
        Bootstrap::SixSquare => '\u{f7b7}',
        Bootstrap::SixSquareFill => '\u{f7b6}',
        Bootstrap::SkipBackward => '\u{f552}',
        Bootstrap::SkipBackwardBtn => '\u{f54e}',
        Bootstrap::SkipBackwardBtnFill => '\u{f54d}',
        Bootstrap::SkipBackwardCircle => '\u{f550}',
        Bootstrap::SkipBackwardCircleFill => '\u{f54f}',
        Bootstrap::SkipBackwardFill => '\u{f551}',
        Bootstrap::SkipEnd => '\u{f558}',
        Bootstrap::SkipEndBtn => '\u{f554}',
        Bootstrap::SkipEndBtnFill => '\u{f553}',
        Bootstrap::SkipEndCircle => '\u{f556}',
        Bootstrap::SkipEndCircleFill => '\u{f555}',
        Bootstrap::SkipEndFill => '\u{f557}',
        Bootstrap::SkipForward => '\u{f55e}',
        Bootstrap::SkipForwardBtn => '\u{f55a}',
        Bootstrap::SkipForwardBtnFill => '\u{f559}',
        Bootstrap::SkipForwardCircle => '\u{f55c}',
        Bootstrap::SkipForwardCircleFill => '\u{f55b}',
        Bootstrap::SkipForwardFill => '\u{f55d}',
        Bootstrap::SkipStart => '\u{f564}',
        Bootstrap::SkipStartBtn => '\u{f560}',
        Bootstrap::SkipStartBtnFill => '\u{f55f}',
        Bootstrap::SkipStartCircle => '\u{f562}',
        Bootstrap::SkipStartCircleFill => '\u{f561}',
        Bootstrap::SkipStartFill => '\u{f563}',
        Bootstrap::Skype => '\u{f656}',
        Bootstrap::Slack => '\u{f565}',
        Bootstrap::Slash => '\u{f56a}',
        Bootstrap::SlashCircle => '\u{f567}',
        Bootstrap::SlashCircleFill => '\u{f566}',
        Bootstrap::SlashLg => '\u{f657}',
        Bootstrap::SlashSquare => '\u{f569}',
        Bootstrap::SlashSquareFill => '\u{f568}',
        Bootstrap::Sliders => '\u{f56b}',
        Bootstrap::Sliderstwo => '\u{f789}',
        Bootstrap::SliderstwoVertical => '\u{f788}',
        Bootstrap::Smartwatch => '\u{f56c}',
        Bootstrap::Snapchat => '\u{f665}',
        Bootstrap::Snow => '\u{f56d}',
        Bootstrap::Snowthree => '\u{f56f}',
        Bootstrap::Snowtwo => '\u{f56e}',
        Bootstrap::SortAlphaDown => '\u{f571}',
        Bootstrap::SortAlphaDownAlt => '\u{f570}',
        Bootstrap::SortAlphaUp => '\u{f573}',
        Bootstrap::SortAlphaUpAlt => '\u{f572}',
        Bootstrap::SortDown => '\u{f575}',
        Bootstrap::SortDownAlt => '\u{f574}',
        Bootstrap::SortNumericDown => '\u{f577}',
        Bootstrap::SortNumericDownAlt => '\u{f576}',
        Bootstrap::SortNumericUp => '\u{f579}',
        Bootstrap::SortNumericUpAlt => '\u{f578}',
        Bootstrap::SortUp => '\u{f57b}',
        Bootstrap::SortUpAlt => '\u{f57a}',
        Bootstrap::Soundwave => '\u{f57c}',
        Bootstrap::Sourceforge => '\u{f8d6}',
        Bootstrap::Speaker => '\u{f57e}',
        Bootstrap::SpeakerFill => '\u{f57d}',
        Bootstrap::Speedometer => '\u{f57f}',
        Bootstrap::Speedometertwo => '\u{f580}',
        Bootstrap::Spellcheck => '\u{f581}',
        Bootstrap::Spotify => '\u{f666}',
        Bootstrap::Square => '\u{f584}',
        Bootstrap::SquareFill => '\u{f582}',
        Bootstrap::SquareHalf => '\u{f583}',
        Bootstrap::Stack => '\u{f585}',
        Bootstrap::StackOverflow => '\u{f667}',
        Bootstrap::Star => '\u{f588}',
        Bootstrap::StarFill => '\u{f586}',
        Bootstrap::StarHalf => '\u{f587}',
        Bootstrap::Stars => '\u{f589}',
        Bootstrap::Steam => '\u{f6c1}',
        Bootstrap::Stickies => '\u{f58b}',
        Bootstrap::StickiesFill => '\u{f58a}',
        Bootstrap::Sticky => '\u{f58d}',
        Bootstrap::StickyFill => '\u{f58c}',
        Bootstrap::Stop => '\u{f593}',
        Bootstrap::StopBtn => '\u{f58f}',
        Bootstrap::StopBtnFill => '\u{f58e}',
        Bootstrap::StopCircle => '\u{f591}',
        Bootstrap::StopCircleFill => '\u{f590}',
        Bootstrap::StopFill => '\u{f592}',
        Bootstrap::Stoplights => '\u{f595}',
        Bootstrap::StoplightsFill => '\u{f594}',
        Bootstrap::Stopwatch => '\u{f597}',
        Bootstrap::StopwatchFill => '\u{f596}',
        Bootstrap::Strava => '\u{f668}',
        Bootstrap::Stripe => '\u{f847}',
        Bootstrap::Subscript => '\u{f848}',
        Bootstrap::Substack => '\u{f8d7}',
        Bootstrap::SuitClub => '\u{f59a}',
        Bootstrap::SuitClubFill => '\u{f599}',
        Bootstrap::SuitDiamond => '\u{f59c}',
        Bootstrap::SuitDiamondFill => '\u{f59b}',
        Bootstrap::SuitHeart => '\u{f59e}',
        Bootstrap::SuitHeartFill => '\u{f59d}',
        Bootstrap::SuitSpade => '\u{f5a0}',
        Bootstrap::SuitSpadeFill => '\u{f59f}',
        Bootstrap::Suitcase => '\u{f900}',
        Bootstrap::SuitcaseFill => '\u{f8fd}',
        Bootstrap::SuitcaseLg => '\u{f8ff}',
        Bootstrap::SuitcaseLgFill => '\u{f8fe}',
        Bootstrap::Suitcasetwo => '\u{f902}',
        Bootstrap::SuitcasetwoFill => '\u{f901}',
        Bootstrap::SunFill => '\u{f5a1}',
        Bootstrap::Sunglasses => '\u{f5a3}',
        Bootstrap::Sunrise => '\u{f5a5}',
        Bootstrap::SunriseFill => '\u{f5a4}',
        Bootstrap::Sunset => '\u{f5a7}',
        Bootstrap::SunsetFill => '\u{f5a6}',
        Bootstrap::Superscript => '\u{f849}',
        Bootstrap::SymmetryHorizontal => '\u{f5a8}',
        Bootstrap::SymmetryVertical => '\u{f5a9}',
        Bootstrap::Table => '\u{f5aa}',
        Bootstrap::Tablet => '\u{f5ae}',
        Bootstrap::TabletFill => '\u{f5ab}',
        Bootstrap::TabletLandscape => '\u{f5ad}',
        Bootstrap::TabletLandscapeFill => '\u{f5ac}',
        Bootstrap::Tag => '\u{f5b0}',
        Bootstrap::TagFill => '\u{f5af}',
        Bootstrap::Tags => '\u{f5b2}',
        Bootstrap::TagsFill => '\u{f5b1}',
        Bootstrap::TaxiFront => '\u{f8ad}',
        Bootstrap::TaxiFrontFill => '\u{f8ac}',
        Bootstrap::Telegram => '\u{f5b3}',
        Bootstrap::Telephone => '\u{f5c1}',
        Bootstrap::TelephoneFill => '\u{f5b4}',
        Bootstrap::TelephoneForward => '\u{f5b6}',
        Bootstrap::TelephoneForwardFill => '\u{f5b5}',
        Bootstrap::TelephoneInbound => '\u{f5b8}',
        Bootstrap::TelephoneInboundFill => '\u{f5b7}',
        Bootstrap::TelephoneMinus => '\u{f5ba}',
        Bootstrap::TelephoneMinusFill => '\u{f5b9}',
        Bootstrap::TelephoneOutbound => '\u{f5bc}',
        Bootstrap::TelephoneOutboundFill => '\u{f5bb}',
        Bootstrap::TelephonePlus => '\u{f5be}',
        Bootstrap::TelephonePlusFill => '\u{f5bd}',
        Bootstrap::TelephoneX => '\u{f5c0}',
        Bootstrap::TelephoneXFill => '\u{f5bf}',
        Bootstrap::TencentQq => '\u{f8cb}',
        Bootstrap::Terminal => '\u{f5c3}',
        Bootstrap::TerminalDash => '\u{f6c3}',
        Bootstrap::TerminalFill => '\u{f5c2}',
        Bootstrap::TerminalPlus => '\u{f6c4}',
        Bootstrap::TerminalSplit => '\u{f6c5}',
        Bootstrap::TerminalX => '\u{f6ff}',
        Bootstrap::TextCenter => '\u{f5c4}',
        Bootstrap::TextIndentLeft => '\u{f5c5}',
        Bootstrap::TextIndentRight => '\u{f5c6}',
        Bootstrap::TextLeft => '\u{f5c7}',
        Bootstrap::TextParagraph => '\u{f5c8}',
        Bootstrap::TextRight => '\u{f5c9}',
        Bootstrap::TextWrap => '\u{f84e}',
        Bootstrap::Textarea => '\u{f5cc}',
        Bootstrap::TextareaResize => '\u{f5ca}',
        Bootstrap::TextareaT => '\u{f5cb}',
        Bootstrap::Thermometer => '\u{f5d2}',
        Bootstrap::ThermometerHalf => '\u{f5cd}',
        Bootstrap::ThermometerHigh => '\u{f5ce}',
        Bootstrap::ThermometerLow => '\u{f5cf}',
        Bootstrap::ThermometerSnow => '\u{f5d0}',
        Bootstrap::ThermometerSun => '\u{f5d1}',
        Bootstrap::Threads => '\u{f8d9}',
        Bootstrap::ThreadsFill => '\u{f8d8}',
        Bootstrap::ThreeCircle => '\u{f7a3}',
        Bootstrap::ThreeCircleFill => '\u{f7a2}',
        Bootstrap::ThreeDots => '\u{f5d4}',
        Bootstrap::ThreeDotsVertical => '\u{f5d3}',
        Bootstrap::ThreeSquare => '\u{f7a5}',
        Bootstrap::ThreeSquareFill => '\u{f7a4}',
        Bootstrap::Thunderbolt => '\u{f6f0}',
        Bootstrap::ThunderboltFill => '\u{f6ef}',
        Bootstrap::Ticket => '\u{f6cb}',
        Bootstrap::TicketDetailed => '\u{f6c7}',
        Bootstrap::TicketDetailedFill => '\u{f6c6}',
        Bootstrap::TicketFill => '\u{f6c8}',
        Bootstrap::TicketPerforated => '\u{f6ca}',
        Bootstrap::TicketPerforatedFill => '\u{f6c9}',
        Bootstrap::Tiktok => '\u{f6cc}',
        Bootstrap::ToggleOff => '\u{f5d5}',
        Bootstrap::ToggleOn => '\u{f5d6}',
        Bootstrap::Toggles => '\u{f5d9}',
        Bootstrap::Togglestwo => '\u{f5da}',
        Bootstrap::ToggletwoOff => '\u{f5d7}',
        Bootstrap::ToggletwoOn => '\u{f5d8}',
        Bootstrap::Tools => '\u{f5db}',
        Bootstrap::Tornado => '\u{f5dc}',
        Bootstrap::TrainFreightFront => '\u{f81b}',
        Bootstrap::TrainFreightFrontFill => '\u{f81a}',
        Bootstrap::TrainFront => '\u{f81d}',
        Bootstrap::TrainFrontFill => '\u{f81c}',
        Bootstrap::TrainLightrailFront => '\u{f81f}',
        Bootstrap::TrainLightrailFrontFill => '\u{f81e}',
        Bootstrap::Translate => '\u{f658}',
        Bootstrap::Transparency => '\u{f8da}',
        Bootstrap::Trash => '\u{f5de}',
        Bootstrap::TrashFill => '\u{f5dd}',
        Bootstrap::Trashthree => '\u{f78b}',
        Bootstrap::TrashthreeFill => '\u{f78a}',
        Bootstrap::Trashtwo => '\u{f5e0}',
        Bootstrap::TrashtwoFill => '\u{f5df}',
        Bootstrap::Tree => '\u{f5e2}',
        Bootstrap::TreeFill => '\u{f5e1}',
        Bootstrap::Trello => '\u{f84a}',
        Bootstrap::Triangle => '\u{f5e5}',
        Bootstrap::TriangleFill => '\u{f5e3}',
        Bootstrap::TriangleHalf => '\u{f5e4}',
        Bootstrap::Trophy => '\u{f5e7}',
        Bootstrap::TrophyFill => '\u{f5e6}',
        Bootstrap::TropicalStorm => '\u{f5e8}',
        Bootstrap::Truck => '\u{f5ea}',
        Bootstrap::TruckFlatbed => '\u{f5e9}',
        Bootstrap::TruckFront => '\u{f821}',
        Bootstrap::TruckFrontFill => '\u{f820}',
        Bootstrap::Tsunami => '\u{f5eb}',
        Bootstrap::Tv => '\u{f5ed}',
        Bootstrap::TvFill => '\u{f5ec}',
        Bootstrap::Twitch => '\u{f5ee}',
        Bootstrap::Twitter => '\u{f5ef}',
        Bootstrap::TwitterX => '\u{f8db}',
        Bootstrap::TwoCircle => '\u{f79d}',
        Bootstrap::TwoCircleFill => '\u{f79c}',
        Bootstrap::TwoSquare => '\u{f79f}',
        Bootstrap::TwoSquareFill => '\u{f79e}',
        Bootstrap::Type => '\u{f5f7}',
        Bootstrap::TypeBold => '\u{f5f0}',
        Bootstrap::TypeHfive => '\u{f8dd}',
        Bootstrap::TypeHfour => '\u{f8dc}',
        Bootstrap::TypeHone => '\u{f5f1}',
        Bootstrap::TypeHsix => '\u{f8de}',
        Bootstrap::TypeHthree => '\u{f5f3}',
        Bootstrap::TypeHtwo => '\u{f5f2}',
        Bootstrap::TypeItalic => '\u{f5f4}',
        Bootstrap::TypeStrikethrough => '\u{f5f5}',
        Bootstrap::TypeUnderline => '\u{f5f6}',
        Bootstrap::Ubuntu => '\u{f822}',
        Bootstrap::UiChecks => '\u{f5f9}',
        Bootstrap::UiChecksGrid => '\u{f5f8}',
        Bootstrap::UiRadios => '\u{f5fb}',
        Bootstrap::UiRadiosGrid => '\u{f5fa}',
        Bootstrap::Umbrella => '\u{f5fd}',
        Bootstrap::UmbrellaFill => '\u{f5fc}',
        Bootstrap::Unindent => '\u{f823}',
        Bootstrap::Union => '\u{f5fe}',
        Bootstrap::Unity => '\u{f824}',
        Bootstrap::UniversalAccess => '\u{f826}',
        Bootstrap::UniversalAccessCircle => '\u{f825}',
        Bootstrap::Unlock => '\u{f600}',
        Bootstrap::UnlockFill => '\u{f5ff}',
        Bootstrap::Upc => '\u{f602}',
        Bootstrap::UpcScan => '\u{f601}',
        Bootstrap::Upload => '\u{f603}',
        Bootstrap::Usb => '\u{f6de}',
        Bootstrap::UsbC => '\u{f6d9}',
        Bootstrap::UsbCFill => '\u{f6d8}',
        Bootstrap::UsbDrive => '\u{f6f2}',
        Bootstrap::UsbDriveFill => '\u{f6f1}',
        Bootstrap::UsbFill => '\u{f6da}',
        Bootstrap::UsbMicro => '\u{f6f4}',
        Bootstrap::UsbMicroFill => '\u{f6f3}',
        Bootstrap::UsbMini => '\u{f6f6}',
        Bootstrap::UsbMiniFill => '\u{f6f5}',
        Bootstrap::UsbPlug => '\u{f6dc}',
        Bootstrap::UsbPlugFill => '\u{f6db}',
        Bootstrap::UsbSymbol => '\u{f6dd}',
        Bootstrap::Valentine => '\u{f78c}',
        Bootstrap::Valentinetwo => '\u{f78d}',
        Bootstrap::VectorPen => '\u{f604}',
        Bootstrap::ViewList => '\u{f605}',
        Bootstrap::ViewStacked => '\u{f606}',
        Bootstrap::Vignette => '\u{f903}',
        Bootstrap::Vimeo => '\u{f66a}',
        Bootstrap::Vinyl => '\u{f608}',
        Bootstrap::VinylFill => '\u{f607}',
        Bootstrap::Virus => '\u{f827}',
        Bootstrap::Virustwo => '\u{f828}',
        Bootstrap::Voicemail => '\u{f609}',
        Bootstrap::VolumeDown => '\u{f60b}',
        Bootstrap::VolumeDownFill => '\u{f60a}',
        Bootstrap::VolumeMute => '\u{f60d}',
        Bootstrap::VolumeMuteFill => '\u{f60c}',
        Bootstrap::VolumeOff => '\u{f60f}',
        Bootstrap::VolumeOffFill => '\u{f60e}',
        Bootstrap::VolumeUp => '\u{f611}',
        Bootstrap::VolumeUpFill => '\u{f610}',
        Bootstrap::Vr => '\u{f612}',
        Bootstrap::Wallet => '\u{f614}',
        Bootstrap::WalletFill => '\u{f613}',
        Bootstrap::Wallettwo => '\u{f615}',
        Bootstrap::Watch => '\u{f616}',
        Bootstrap::Water => '\u{f617}',
        Bootstrap::Webcam => '\u{f67d}',
        Bootstrap::WebcamFill => '\u{f67c}',
        Bootstrap::Wechat => '\u{f829}',
        Bootstrap::Whatsapp => '\u{f618}',
        Bootstrap::Wifi => '\u{f61c}',
        Bootstrap::WifiOff => '\u{f61b}',
        Bootstrap::WifiOne => '\u{f619}',
        Bootstrap::WifiTwo => '\u{f61a}',
        Bootstrap::Wikipedia => '\u{f8cc}',
        Bootstrap::Wind => '\u{f61d}',
        Bootstrap::Window => '\u{f620}',
        Bootstrap::WindowDash => '\u{f6cd}',
        Bootstrap::WindowDesktop => '\u{f6ce}',
        Bootstrap::WindowDock => '\u{f61e}',
        Bootstrap::WindowFullscreen => '\u{f6cf}',
        Bootstrap::WindowPlus => '\u{f6d0}',
        Bootstrap::WindowSidebar => '\u{f61f}',
        Bootstrap::WindowSplit => '\u{f6d1}',
        Bootstrap::WindowStack => '\u{f6d2}',
        Bootstrap::WindowX => '\u{f6d3}',
        Bootstrap::Windows => '\u{f65e}',
        Bootstrap::Wordpress => '\u{f669}',
        Bootstrap::Wrench => '\u{f621}',
        Bootstrap::WrenchAdjustable => '\u{f790}',
        Bootstrap::WrenchAdjustableCircle => '\u{f78f}',
        Bootstrap::WrenchAdjustableCircleFill => '\u{f78e}',
        Bootstrap::X => '\u{f62a}',
        Bootstrap::XCircle => '\u{f623}',
        Bootstrap::XCircleFill => '\u{f622}',
        Bootstrap::XDiamond => '\u{f625}',
        Bootstrap::XDiamondFill => '\u{f624}',
        Bootstrap::XLg => '\u{f659}',
        Bootstrap::XOctagon => '\u{f627}',
        Bootstrap::XOctagonFill => '\u{f626}',
        Bootstrap::XSquare => '\u{f629}',
        Bootstrap::XSquareFill => '\u{f628}',
        Bootstrap::Xbox => '\u{f6d4}',
        Bootstrap::Yelp => '\u{f82a}',
        Bootstrap::YinYang => '\u{f67e}',
        Bootstrap::Youtube => '\u{f62b}',
        Bootstrap::ZeroCircle => '\u{f840}',
        Bootstrap::ZeroCircleFill => '\u{f83f}',
        Bootstrap::ZeroSquare => '\u{f842}',
        Bootstrap::ZeroSquareFill => '\u{f841}',
        Bootstrap::ZoomIn => '\u{f62c}',
        Bootstrap::ZoomOut => '\u{f62d}',
    }
}

/// Converts an [`Bootstrap`] into a [`char`]
pub fn icon_to_string(icon: Bootstrap) -> String {
    icon_to_char(icon).to_string()
}

impl Display for Bootstrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", icon_to_char(*self))
    }
}

impl From<Bootstrap> for char {
    fn from(icon: Bootstrap) -> Self {
        icon_to_char(icon)
    }
}

impl From<Bootstrap> for String {
    fn from(icon: Bootstrap) -> Self {
        format!("{}", icon_to_char(icon))
    }
}

/// Converts an [`Bootstrap`] into a [`Text`] with the font.
pub fn icon_to_text(icon: Bootstrap) -> Text<'static> {
    text(icon_to_char(icon)).font(BOOTSTRAP_FONT)
}
