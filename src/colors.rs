///Following the opening in the ansi documentation - the escape code is the hexadecimal one
pub const OPENING_COLOR:&str = "\x1B[";
///Following the regular way of develpoment, despite COLOR_CLOSER being just "m", it's a constant
pub const COLOR_CLOSER:&str = "m";
#[derive(Copy, Clone)]
#[allow(dead_code)]
///Following the text related colors in the ansi documentation
pub enum TextColours{
    Black = 30 ,
    Red = 31 ,
    Green = 32 ,
    Yellow = 33 ,
    Blue = 34 ,
    Magenta = 35 ,
    Cyan = 36 ,
    Gray = 37 ,
    DarkGray = 90 ,
    LightRed = 91 ,
    LightGreen = 92 ,
    LightYellow = 93 ,
    LightBlue = 94 ,
    Pink = 95 ,
    LightCyan = 96 ,
    White = 97 ,
    NaN = 39 ,
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
///Following the background related colors in the ansi documentation
pub enum BackColours{
    Black = 40 ,
    Red = 41 ,
    Green = 42 ,
    Yellow = 43 ,
    Blue = 44 ,
    Magenta = 45 ,
    Cyan = 46 ,
    Gray = 47 ,
    DarkGray = 100 ,
    LightRed = 101 ,
    LightGreen = 102 ,
    LightYellow = 103 ,
    LightBlue = 104 ,
    Pink = 105 ,
    LightCyan = 106 ,
    White = 107 ,
    NaN = 49 ,
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
///Following the text-style related colors in the ansi documentation
pub enum Styles{
    NaN,
    Bold = 1 ,
    Dim = 2 ,
    Underline=4 ,
    Blink = 5 ,
    Reverse = 7 ,
    Hidden = 8 ,
}
#[derive(Copy, Clone,PartialEq)]
#[allow(dead_code)]
///Following the color/style/background reseting related colors in the ansi documentation
pub enum Reset{
    All = 0 ,
    Bold = 21 ,
    Dim = 22 ,
    Underline = 24 ,
    Blink = 25 ,
    Reverse = 27 ,
    Hidden = 28 ,
}
