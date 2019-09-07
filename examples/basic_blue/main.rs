use ansi_colors::*;
fn main(){
    let mut str1 = ColouredStr::new("hello ansi");
    str1.blue();
    println!("{}",str1);
}
