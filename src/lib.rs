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
///let str2 = format!("{}{}{}","\u{1b}[1;4;5;34m","hello world","\u{1b}[0m");
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
///let str2 = format!("{}{}{}","\u{1b}[1;8;97m","hello world","\u{1b}[0m");
///assert_eq!(str1.coloured_string,str2);
pub struct ColouredStr<'a>{
    pub string:&'a str,
    pub coloured_string:String,
    colorer:String,
    closer:String,
    reset:Reset,
    text_color:TextColours,
    background_color:BackColours,
    styles:Vec<Styles>,
}
impl <'a> fmt::Display for ColouredStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.coloured_string)
    }
}
/*
impl <'a> Deref for ColouredStr<'a> {
    type Target = ColouredStr<'a>;
    fn deref(&self) -> &Self::Target{
        let col1 = ColouredStr{string:self.string,
        coloured_string:self.coloured_string,
        colorer:self.colorer,
        closer:self.closer,
        reset:self.reset,
        text_color:self.text_color,
        background_color:self.background_color,
        styles:self.styles};
        &col1
    }
}*/
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
        let coloured_string = format!("{}{}{}",&colorer[..],string,&closer[..]);
        let col1 = ColouredStr{
            string,coloured_string,colorer,closer,reset:Reset::All,text_color:TextColours::White,background_color:BackColours::NaN,styles
        };
        return col1;
    }
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
        let mut strrr = "".to_string();
        if self.styles.len()>1{
            for i in 0..self.styles.len(){
                strrr.push_str(&(self.styles[i] as u8).to_string()[..]);
                strrr.push(';');
            }
        }else if self.styles.len()==1{
            strrr = format!("{};",&(self.styles[0] as u8).to_string()[..]);
        }
        return format!("{}{}{}{}",OPENING_COLOR,&strrr[..],&tc[..],COLOR_CLOSER);
    }
    fn set_closer(&self)->String {
        let res = (self.reset as u8).to_string().clone();
        return format!("{}{}{}",OPENING_COLOR,&res[..],COLOR_CLOSER);
    }
    ///This functions takes self and changes coloured string color into blue
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.blue();
    ///let str2 = format!("{}{}{}","\u{1b}[34m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn blue(&mut self){
        self.text_color = TextColours::Blue; 
        self.refresh();
    }
    ///This functions takes self and changes coloured string color into red
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.red();
    ///let str2 = format!("{}{}{}","\u{1b}[31m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[32m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[33m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[35m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[36m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[37m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[90m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[91m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[92m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[93m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[94m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[95m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[96m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[97m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[39m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[1;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn bold(&mut self){
        self.styles.push(Styles::Bold); 
        self.refresh();
    }
    ///This functions takes self and adds the dim style to the string
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.dim();
    ///let str2 = format!("{}{}{}","\u{1b}[2;97m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[4;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn underline(&mut self){
        self.styles.push(Styles::Underline); 
        self.refresh();
    }
    ///This functions takes self and adds the blinking style to the string
    ///
    ///
    ///```
    ///use ansi_colors::*;
    ///let mut str1 = ColouredStr::new("hello world");
    ///str1.blink();
    ///let str2 = format!("{}{}{}","\u{1b}[5;97m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[7;97m","hello world","\u{1b}[0m");
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
    ///let str2 = format!("{}{}{}","\u{1b}[8;97m","hello world","\u{1b}[0m");
    ///assert_eq!(str1.coloured_string,str2);
    ///```
    pub fn hidden(&mut self){
        self.styles.push(Styles::Hidden); 
        self.refresh();
    }
}
