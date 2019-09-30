use ansi_colors::*;

fn main(){
    let mut str1 = ColouredStr::new("ERROR!!!");
    str1.to_error();
    println!("{}",str1);
}
