use ansi_colors::*;
fn main(){
    let mut str1 = ColouredStr::new("dsassdasa");
    str1.blue();
    println!("{}",str1.coloured_string);
}
