use minifb::{Window, WindowOptions,Key};

fn main(){
let width=800;
let height=600;
let _black=0x00000000;
let _red= 0x00FF0000u32;
let _green= 0x0000FF00u32;
let _blue= 0x000000FFu32;
// let mut buffer2:Vec<u32>=vec![_blue; width*height];
// let mut _buffer: Vec<u32>= vec![_blue; width * height];
//Just dot
// let x=100;
// let y=50;
// _buffer[y*width+x]=_red;

// // Just rectangle
// let rx=200; //top-left x
// let ry=150; // top-left y
// let rw = 120; //rect width
// let rh=80;//rect height
// let color = _green ;//green

// fn fill_rect(
//     buffer:&mut [u32],
//     width:usize,
//     x0:usize, y0:usize,
//     w:usize, h:usize,
//     color:u32
// )
// {
// for y in y0..y0+h {
//     for x in x0.. x0+w{
//         buffer[y*width+x]=color;
//     }
// }
// }
// let mut buffer: Vec<u32>= vec![_blue; width * height];
// fill_rect(&mut buffer, width,200,150,120,80,0x0000FF00);

// fill_rect(&mut buffer, width,250,100,120,80,_black);


fn fill_rect(
    buffer:&mut [u32],
    width:usize,
    x0:usize, y0:usize,
    w:usize, h:usize,
    color:u32
)
{
for y in y0..y0+h {
    for x in x0.. x0+w{
        buffer[y*width+x]=color;
    }
}
}
let mut buffer: Vec<u32>= vec![_blue; width * height];
// fill_rect(&mut buffer, width,200,150,120,80,0x0000FF00);

// fill_rect(&mut buffer, width,250,100,120,80,_black);



let mut window = 
Window::new("BlackScreen",width,height,WindowOptions::default(),).unwrap_or_else(|e|{panic!("{}",e);});
window.set_target_fps(60);
while window.is_open() && !window.is_key_down(Key::Escape){
for px in buffer.iter_mut() {*px=0;}
fill_rect(&mut buffer, width,200,100,90,80,_blue);
window.update_with_buffer(&buffer,width,height).unwrap();}}
