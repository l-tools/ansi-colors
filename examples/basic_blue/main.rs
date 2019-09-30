use ansi_colors::*;
fn main(){
    let mut str1 = ColouredStr::new("hello ansi");
    str1.blue()
        .bold().underline();
    println!("{}",&str1.coloured_string[1..]);
}
