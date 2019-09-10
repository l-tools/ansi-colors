use ansi_colors::*;
fn main(){
    let mut str1 = ColouredStr::new("string1sdaasdsdasda\nsdasdasdasd\nadasds");
    str1.green();
    str1.bold();
    str1.back_black();
    println!("{}",str1);

}
