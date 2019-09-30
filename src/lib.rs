///this mod public use is for you to understand the ansi escape codes:)
pub mod colors;
use std::fmt;
use crate::colors::*;
#[derive(Clone)]
///This struct is the main way to utilize the coloring sceme in this crate.
///
///
///This struct contains a number of parameters inquiring to the strings state, such as
///style,color,base string, reset actions, background and several internal things.
///this structs, as it say's below implements the fmt::Display trait, that means that you will not
///need to do anything special in order to print after you have changed the color
///you can use only one color but you can have even all the styles together!
///
///here is a blue,bold,underlined and blinking string example(just a fromatted output) :
///```
///use ansi_colors::*;
///let mut str1 = ColouredStr::new("hello world");
///str1.blue();
///str1.bold();
///str1.underline();
///str1.blink();
///let str2 = format!("{}{}{}","\u{1b}[1;4;5;49;34m","hello world","\u{1b}[0m");
///assert_eq!(str1.coloured_string,str2);
///```
///here is a white,bold and hidden string example(good for passwords):
///
///
///```
///use ansi_colors::*;
///let mut str1 = ColouredStr::new("hello world");
///str1.white();
///str1.bold();
///str1.hidden();
///let str2 = format!("{}{}{}","\u{1b}[1;8;49;97m","hello world","\u{1b}[0m");
///assert_eq!(str1.coloured_string,str2);
pub struct ColouredStr<'a>{
    pub string:&'a str,
    pub coloured_string:String,
    colorer:String,
    closer:String,
    resets:Vec<Reset>,
    text_color:TextColours,
    background_color:BackColours,
    styles:Vec<Styles>,
}
impl <'a> fmt::Display for ColouredStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.coloured_string)
    }
}
impl <'a> ColouredStr<'a> {
    ///This functions recieves one parameter - a string and creates a defult struct for it.
    ///the color is natural(no change), as well as the styles and background.
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let str1 = ColouredStr::new("hello world");
    ///assert_eq!(str1.string,"hello world");
    ///```
    pub fn new(string:&str)->ColouredStr{
        let colorer = format!("{}{}",OPENING_COLOR,COLOR_CLOSER);
        let closer = format!("{}{}{}",OPENING_COLOR,"0",COLOR_CLOSER);
        let styles = vec![];
        let resets = vec![Reset::All];
        let coloured_string = format!("{}{}{}",&colorer[..],string,&closer[..]);
        let col1 = ColouredStr{
            string,coloured_string,colorer,closer,resets,text_color:TextColours::White,background_color:BackColours::NaN,styles
        };
        return col1;
    }
    /*
    fn cpy(self)->Self{
        let col1 = ColouredStr{
            string:self.string,
            coloured_string:self.coloured_string,
            colorer:self.colorer,
            closer:self.closer,
            resets:self.resets,
            text_color:self.text_color,
            background_color:self.,
            styles:self.styles};
        col1
        cc
    }*/
    fn refresh(&mut self){
        let clo1 = &(self.set_closer())[..];
        let col1 = &(self.set_colorer())[..];
        let strr = self.string;
        (*self).coloured_string = format!("{}{}{}",col1,strr,clo1);
    }
    fn set_colorer(&self)->String{
        let mut tc = "".to_string();
        if (self.text_color as u8) != 0{
            tc = (self.text_color as u8).to_string();
        }
        let mut bc = "".to_string();
        if (self.background_color as u8) != 0{
            bc = (self.background_color as u8).to_string();
        }
        let mut strrr = "".to_string();
        if self.styles.len()>1{
            for i in 0..self.styles.len(){
                strrr.push_str(&(self.styles[i] as u8).to_string()[..]);
                strrr.push(';');
            }
        }else if self.styles.len()==1{
            strrr = format!("{};",&(self.styles[0] as u8).to_string()[..]);
        }
        return format!("{}{}{};{}{}",OPENING_COLOR,&strrr[..],&bc[..],&tc[..],COLOR_CLOSER);
    }
    fn set_closer(&mut self)->String {
        let mut res = "".to_string();
        if self.resets[0]==Reset::All && self.resets.len()>1{
            self.resets = self.resets[1..].to_vec();
        }
        if self.resets.len()>1{
            for i in 0..self.resets.len(){
                res.push_str(&(self.resets[i] as u8).to_string()[..]);
                res.push(';');
            }
        }else if self.resets.len()==1{
            res = format!("{}",&(self.resets[0] as u8).to_string()[..]);
        }
        return format!("{}{}{}",OPENING_COLOR,&res[..],COLOR_CLOSER);
    }
    ///This functions takes self and presents the color string in error format
    pub fn to_error(&mut self){
        self.red();
        self.bold();
        self.underline();
        self.blink();
    }
    ///This functions takes self and presents the color string in success format
    pub fn to_success(&mut self){
        self.green();
        self.bold();
    }
    ///This functions takes self and presents the color string in password format
    pub fn to_password(&mut self){
        self.hidden();
    }
    ///This functions takes self and changes coloured string color into blue
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.blue();
    ///let str2 = format!("{}{}{}","\u{1b}[49;34m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn blue(&mut self)->Self{
        let mut strrr = self.clone();
        self.text_color = TextColours::Blue; 
        strrr.text_color = TextColours::Blue; 
        self.refresh();
        strrr.refresh();
        strrr
    }
    ///This functions takes self and changes coloured string color into black
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.black();
    ///let str2 = format!("{}{}{}","\u{1b}[49;30m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn black(&mut self){
        self.text_color = TextColours::Black;
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into red
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.red();
    ///let str2 = format!("{}{}{}","\u{1b}[49;31m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn red(&mut self){
        self.text_color = TextColours::Red; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into green
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.green();
    ///let str2 = format!("{}{}{}","\u{1b}[49;32m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn green(&mut self){
        self.text_color = TextColours::Green; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into yellow
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.yellow();
    ///let str2 = format!("{}{}{}","\u{1b}[49;33m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn yellow(&mut self){
        self.text_color = TextColours::Yellow; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into magenta
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.magenta();
    ///let str2 = format!("{}{}{}","\u{1b}[49;35m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn magenta(&mut self){
        self.text_color = TextColours::Magenta; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into cyan
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.cyan();
    ///let str2 = format!("{}{}{}","\u{1b}[49;36m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn cyan(&mut self){
        self.text_color = TextColours::Cyan; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into gray
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.gray();
    ///let str2 = format!("{}{}{}","\u{1b}[49;37m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn gray(&mut self){
        self.text_color = TextColours::Gray; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into dark gray
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.dark_gray();
    ///let str2 = format!("{}{}{}","\u{1b}[49;90m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn dark_gray(&mut self){
        self.text_color = TextColours::DarkGray; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into light red
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.light_red();
    ///let str2 = format!("{}{}{}","\u{1b}[49;91m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn light_red(&mut self){
        self.text_color = TextColours::LightRed; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into light green
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.light_green();
    ///let str2 = format!("{}{}{}","\u{1b}[49;92m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn light_green(&mut self){
        self.text_color = TextColours::LightGreen; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into light yellow
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.light_yellow();
    ///let str2 = format!("{}{}{}","\u{1b}[49;93m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn light_yellow(&mut self){
        self.text_color = TextColours::LightYellow; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into light blue
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.light_blue();
    ///let str2 = format!("{}{}{}","\u{1b}[49;94m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn light_blue(&mut self){
        self.text_color = TextColours::LightBlue; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into pink
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.pink();
    ///let str2 = format!("{}{}{}","\u{1b}[49;95m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn pink(&mut self){
        self.text_color = TextColours::Pink; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into light cyan
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.light_cyan();
    ///let str2 = format!("{}{}{}","\u{1b}[49;96m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn light_cyan(&mut self){
        self.text_color = TextColours::LightCyan; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into white
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.white();
    ///let str2 = format!("{}{}{}","\u{1b}[49;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn white(&mut self){
        self.text_color = TextColours::White; 
        self.refresh();
    }
    ///This functions takes self and resets the string's color
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.none();
    ///let str2 = format!("{}{}{}","\u{1b}[49;39m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn none(&mut self){
        self.text_color = TextColours::NaN; 
        self.refresh();
    }
    ///This functions takes self and adds the bold style to the string
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.bold();
    ///let str2 = format!("{}{}{}","\u{1b}[1;49;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn bold(&mut self)->Self{
        let mut strrr = (*self).clone();
        self.styles.push(Styles::Bold); 
        strrr.styles.push(Styles::Bold); 
        self.refresh();
        strrr.refresh();
        strrr
    }
    ///This functions takes self and adds the dim style to the string
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.dim();
    ///let str2 = format!("{}{}{}","\u{1b}[2;49;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn dim(&mut self){
        self.styles.push(Styles::Dim); 
        self.refresh();
    }
    ///This functions takes self and adds the underline style to the string
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.underline();
    ///let str2 = format!("{}{}{}","\u{1b}[4;49;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn underline(&mut self)->Self{
        let mut strrr = self.clone();
        self.styles.push(Styles::Underline); 
        strrr.styles.push(Styles::Bold); 
        self.refresh();
        strrr.refresh();
        strrr
    }
    ///This functions takes self and adds the blinking style to the string
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.blink();
    ///let str2 = format!("{}{}{}","\u{1b}[5;49;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn blink(&mut self){
        self.styles.push(Styles::Blink); 
        self.refresh();
    }
    ///This functions takes self and adds the reverse style to the string
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.reverse();
    ///let str2 = format!("{}{}{}","\u{1b}[7;49;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn reverse(&mut self){
        self.styles.push(Styles::Reverse); 
        self.refresh();
    }
    ///This functions takes self and adds the hidden(   ) style to the string
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.hidden();
    ///let str2 = format!("{}{}{}","\u{1b}[8;49;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn hidden(&mut self){
        self.styles.push(Styles::Hidden); 
        self.refresh();
    }
    ///This functions takes self and changes the background to be black 
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.white();
    ///str1.back_black();
    ///let str2 = format!("{}{}{}","\u{1b}[40;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_black(&mut self){
        self.background_color = BackColours::Black;
        self.refresh();
    }
    ///This functions takes self and changes the background to be red 
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.blue();
    ///str1.back_red();
    ///let str2 = format!("{}{}{}","\u{1b}[41;34m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_red(&mut self){
        self.background_color = BackColours::Red;
        self.refresh();
    }
    ///This functions takes self and changes the background to be blue 
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.red();
    ///str1.back_blue();
    ///let str2 = format!("{}{}{}","\u{1b}[44;31m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_blue(&mut self){
        self.background_color = BackColours::Blue;
        self.refresh();
    }
    ///This functions takes self and changes the background to be green 
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.red();
    ///str1.back_green();
    ///let str2 = format!("{}{}{}","\u{1b}[42;31m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_green(&mut self){
        self.background_color = BackColours::Green;
        self.refresh();
    }
    ///This functions takes self and changes the background to be yellow
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.magenta();
    ///str1.back_yellow();
    ///let str2 = format!("{}{}{}","\u{1b}[43;35m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_yellow(&mut self){
        self.background_color = BackColours::Yellow;
        self.refresh();
    }
    ///This functions takes self and changes the background to be magenta
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.black();
    ///str1.back_magenta();
    ///let str2 = format!("{}{}{}","\u{1b}[45;30m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_magenta(&mut self){
        self.background_color = BackColours::Magenta;
        self.refresh();
    }
    ///This functions takes self and changes the background to be cyan  
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.red();
    ///str1.back_cyan();
    ///let str2 = format!("{}{}{}","\u{1b}[46;31m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_cyan(&mut self){
        self.background_color = BackColours::Cyan;
        self.refresh();
    }
    ///This functions takes self and changes the background to be gray  
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.yellow();
    ///str1.back_gray();
    ///let str2 = format!("{}{}{}","\u{1b}[47;33m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_gray(&mut self){
        self.background_color = BackColours::Gray;
        self.refresh();
    }
    ///This functions takes self and changes the background to be dark gray
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.white();
    ///str1.back_dark_gray();
    ///let str2 = format!("{}{}{}","\u{1b}[100;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_dark_gray(&mut self){
        self.background_color = BackColours::DarkGray;
        self.refresh();
    }
    ///This functions takes self and changes the background to be light red
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.black();
    ///str1.back_light_red();
    ///let str2 = format!("{}{}{}","\u{1b}[101;30m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_light_red(&mut self){
        self.background_color = BackColours::LightRed;
        self.refresh();
    }
    ///This functions takes self and changes the background to be light green 
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.red();
    ///str1.back_light_green();
    ///let str2 = format!("{}{}{}","\u{1b}[102;31m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_light_green(&mut self){
        self.background_color = BackColours::LightGreen;
        self.refresh();
    }
    ///This functions takes self and changes the background to be light yellow
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.blue();
    ///str1.back_light_yellow();
    ///let str2 = format!("{}{}{}","\u{1b}[103;34m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_light_yellow(&mut self){
        self.background_color = BackColours::LightYellow;
        self.refresh();
    }
    ///This functions takes self and changes the background to be light blue 
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.black();
    ///str1.back_light_blue();
    ///let str2 = format!("{}{}{}","\u{1b}[104;30m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_light_blue(&mut self){
        self.background_color = BackColours::LightBlue;
        self.refresh();
    }
    ///This functions takes self and changes the background to be pink        
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.black();
    ///str1.back_pink();
    ///let str2 = format!("{}{}{}","\u{1b}[105;30m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_pink(&mut self){
        self.background_color = BackColours::Pink;
        self.refresh();
    }
    ///This functions takes self and changes the background to be light cyan 
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.red();
    ///str1.back_light_green();
    ///let str2 = format!("{}{}{}","\u{1b}[102;31m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_light_cyan(&mut self){
        self.background_color = BackColours::LightCyan;
        self.refresh();
    }
    ///This functions takes self and changes the background to be white 
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.black();
    ///str1.back_white();
    ///let str2 = format!("{}{}{}","\u{1b}[107;30m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_white(&mut self){
        self.background_color = BackColours::White;
        self.refresh();
    }
    ///This functions takes self and changes the background to be none 
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.red();
    ///str1.back_none();
    ///let str2 = format!("{}{}{}","\u{1b}[49;31m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn back_none(&mut self){
        self.background_color = BackColours::NaN;
        self.refresh();
    }
    ///This functions takes self and adds the reset all operation to the reset
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.blink();
    ///str1.reset_all();
    ///let str2 = format!("{}{}{}","\u{1b}[5;49;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn reset_all(&mut self){
        self.resets.push(Reset::All); 
        self.refresh();
    }
    ///This functions takes self and adds the reset bold operation to the reset
    ///it means it resets only the bold style, not any other active styles or colors.
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.bold();
    ///str1.red();
    ///str1.reset_bold();
    ///let str2 = format!("{}{}{}","\u{1b}[1;49;31m","hello world","\u{1b}[21m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn reset_bold(&mut self){
        self.resets.push(Reset::Bold);
        self.refresh();
    }
    ///This functions takes self and adds the reset dim operation to the reset
    ///it means it resets only the dim style, not any other active styles or colors.
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.dim();
    ///str1.bold();
    ///str1.reset_dim();
    ///let str2 = format!("{}{}{}","\u{1b}[2;1;49;97m","hello world","\u{1b}[22m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn reset_dim(&mut self){
        self.resets.push(Reset::Dim);
        self.refresh();
    }
    ///This functions takes self and adds the reset underline operation to the reset
    ///it means it resets only the underline style, not any other active styles or colors.
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.underline();
    ///str1.back_green();
    ///str1.reset_underline();
    ///let str2 = format!("{}{}{}","\u{1b}[4;42;97m","hello world","\u{1b}[24m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn reset_underline(&mut self){
        self.resets.push(Reset::Underline);
        self.refresh();
    }
    ///This functions takes self and adds the reset blink operation to the reset
    ///it means it resets only the blink style, not any other active styles or colors.
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.blink();
    ///str1.red();
    ///str1.reset_blink();
    ///let str2 = format!("{}{}{}","\u{1b}[5;49;31m","hello world","\u{1b}[25m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn reset_blink(&mut self){
        self.resets.push(Reset::Blink);
        self.refresh();
    }
    ///This functions takes self and adds the reset reverse operation to the reset
    ///it means it resets only the reverse style, not any other active styles or colors.
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.reverse();
    ///str1.back_red();
    ///str1.reset_reverse();
    ///let str2 = format!("{}{}{}","\u{1b}[7;41;97m","hello world","\u{1b}[27m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn reset_reverse(&mut self){
        self.resets.push(Reset::Reverse);
        self.refresh();
    }
    ///This functions takes self and adds the reset hidden operation to the reset
    ///it means it resets only the hidden style, not any other active styles or colors.
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.hidden();
    ///str1.back_black();
    ///str1.reset_hidden();
    ///let str2 = format!("{}{}{}","\u{1b}[8;40;97m","hello world","\u{1b}[28m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn reset_hidden(&mut self){
        self.resets.push(Reset::Hidden);
        self.refresh();
    }
}
