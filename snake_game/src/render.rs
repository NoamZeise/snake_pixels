use crate::shapes;

#[derive(Copy, Clone)]
pub struct FrameInfo {
    pub width: u32,
    pub height: u32,
}

pub fn background(frame: &mut [u8], colour: &[u8;4]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(colour);
    }
}

pub fn pixel(frame_info: FrameInfo, frame: &mut [u8], x: u32, y: u32, colour: &[u8;4]) {
    if x >= frame_info.width ||
       y >= frame_info.height {
           return;
    }
    for i in 0..3 { frame[((frame_info.width * y * 4) + x*4) as usize + i] = colour[i]; }
}

pub fn rect(frame_info: FrameInfo, frame: &mut [u8], rect: shapes::Rect, colour: &[u8;4]) {
    if rect.pos.x > (frame_info.width as f64) ||
       rect.pos.x + rect.dim.x < 0.0          ||
       rect.pos.y > (frame_info.height as f64)||
       rect.pos.y + rect.dim.y < 0.0            {
           return;
     }
    for x in 0..(rect.dim.x) as u32 {
        for y in 0..(rect.dim.y) as u32 {
            pixel(frame_info, frame, (rect.pos.x + 0.5)as u32 + x,(rect.pos.y + 0.5)as u32 + y, colour);
        }
    }
    /*
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        if rect.contains(
            shapes::Vector2 {
                x: (i % frame_info.width as usize) as f64 + 0.5,
                y: (i / frame_info.width as usize) as f64 + 0.5} ) {
            pixel.copy_from_slice(colour);
        }
    }*/
}

/*
pub fn circ(frame_info: FrameInfo, frame: &mut [u8], circ: shapes::Circ, colour: &[u8;4]) {
    if circ.pos.x - circ.r > (frame_info.width as f64) ||
       circ.pos.x + circ.r < 0.0          ||
       circ.pos.y - circ.r > (frame_info.height as f64)||
       circ.pos.y + circ.r < 0.0            {
           return;
     }

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        if circ.contains(
            shapes::Vector2 {
                x: (i % frame_info.width as usize) as f64 + 0.5,
                y: (i / frame_info.width as usize) as f64 + 0.5} ) {
            pixel.copy_from_slice(colour);
        }
    }
}
*/
