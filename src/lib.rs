mod colors;
use crate::colors::*;
#[derive(Clone)]
pub struct ColouredStr<'a>{
    string:&'a str,
    pub coloured_string:String,
    colorer:String,
    closer:String,
    reset:Reset,
    text_color:TextColours,
    background_color:BackColours,
    styles:Vec<Styles>,
}
impl <'a> ColouredStr<'a> {
    pub fn new(string:&str)->ColouredStr{
        let colorer = format!("{}{}{}",OPENING_COLOR,&(TextColours::White as u8).to_string()[..],COLOR_CLOSER);
        let closer = format!("{}{}{}",OPENING_COLOR,"0",COLOR_CLOSER);
        let styles = vec![Styles::NaN];
        let coloured_string = format!("{}{}{}",&colorer[..],string,&closer[..]);
        let col1 = ColouredStr{
            string,coloured_string,colorer,closer,reset:Reset::All,text_color:TextColours::White,background_color:BackColours::NaN,styles
        };
        return col1;
    }
    fn refresh(&mut self){
        let clo2 = self.clone();
        let clo1 = &(clo2.set_closer())[..];
        let col2 = self.clone();
        let col1 = &(col2.set_colorer())[..];
        let strr = self.string;
        (*self).coloured_string = format!("{}{}{}",col1,strr,clo1);
}
    fn set_colorer(self)->String{
        let tc = (self.text_color as u8).to_string();
        return format!("{}{}{}",OPENING_COLOR,&tc[..],COLOR_CLOSER);
    }
    fn set_closer(self)->String {
        let res = (self.reset as u8).to_string().clone();
        return format!("{}{}{}",OPENING_COLOR,&res[..],COLOR_CLOSER);
    }
    pub fn blue(&mut self){
        self.text_color = TextColours::Blue; 
        self.refresh();
    }
    pub fn red(&mut self){
        self.text_color = TextColours::Red; 
        self.refresh();
    }
    pub fn green(&mut self){
        self.text_color = TextColours::Green; 
        self.refresh();
    }
    pub fn yellow(&mut self){
        self.text_color = TextColours::Yellow; 
        self.refresh();
    }
    pub fn magenta(&mut self){
        self.text_color = TextColours::Magenta; 
        self.refresh();
    }
    pub fn cyan(&mut self){
        self.text_color = TextColours::Cyan; 
        self.refresh();
    }
    pub fn gray(&mut self){
        self.text_color = TextColours::Gray; 
        self.refresh();
    }
    pub fn dark_gray(&mut self){
        self.text_color = TextColours::DarkGray; 
        self.refresh();
    }
    pub fn light_red(&mut self){
        self.text_color = TextColours::LightRed; 
        self.refresh();
    }
    pub fn light_green(&mut self){
        self.text_color = TextColours::LightGreen; 
        self.refresh();
    }
    pub fn light_yellow(&mut self){
        self.text_color = TextColours::LightYellow; 
        self.refresh();
    }
    pub fn light_blue(&mut self){
        self.text_color = TextColours::LightBlue; 
        self.refresh();
    }
    pub fn pink(&mut self){
        self.text_color = TextColours::Pink; 
        self.refresh();
    }
    pub fn light_cyan(&mut self){
        self.text_color = TextColours::LightCyan; 
        self.refresh();
    }
    pub fn white(&mut self){
        self.text_color = TextColours::White; 
        self.refresh();
    }
    pub fn none(&mut self){
        self.text_color = TextColours::NaN; 
        self.refresh();
    }/*
    pub fn bold(&'a mut self){
        self.styles.push(Styles::Bold); 
        self.refresh();
    }
    pub fn dim(&'a mut self){
        self.styles.push(Styles::Dim); 
        self.refresh();
    }
    pub fn underline(&'a mut self){
        self.styles.push(Styles::Underline); 
        self.refresh();
    }
    pub fn blink(&'a mut self){
        self.styles.push(Styles::Blink); 
        self.refresh();
    }
    pub fn reverse(&'a mut self){
        self.styles.push(Styles::Reverse); 
        self.refresh();
    }
    pub fn hidden(&'a mut self){
        self.styles.push(Styles::Hidden); 
        self.refresh();
    }*/
    pub fn printable(self)->String{
        self.coloured_string
    }
}
