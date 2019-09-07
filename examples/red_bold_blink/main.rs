use ansi_colors::*;
fn main(){
    let mut str1 = ColouredStr::new("hello world");
    str1.red();
    str1.bold();
    str1.blink();
    println!("{}",str1);
}
