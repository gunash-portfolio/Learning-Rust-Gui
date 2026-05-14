use minifb::{Window, WindowOptions};

fn main(){
let width=800;
let height=800;
let buffer:Vec<u32>=vec![0; width*height];
let mut window = 
Window::new("BlackScreen",width,height,WindowOptions::default(),).unwrap_or_else(|e|{panic!("{}",e);});
window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
while window.is_open() && !window.is_key_down(minifb::Key::Escape){
window.update_with_buffer(&buffer,width,height).unwrap();}}
