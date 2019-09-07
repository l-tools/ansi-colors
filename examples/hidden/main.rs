use ansi_colors::*;
fn main(){
    let mut pass = ColouredStr::new("ansi_coloring");
    pass.white();
    pass.bold();
    pass.hidden();
    println!("{}",pass);
}
