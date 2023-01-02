//mod image_reader;
use crate::andy_vectors::Vec3D as Vec3D;
use crate::andy_vectors::Vec2D as Vec2D;

//https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
fn rgb_to_hsv(rgb: (i32, i32, i32)) -> (i32, i32, i32){
    let r: f32 = (rgb.0 as f32)/255.0;
    let g: f32 = (rgb.1 as f32)/255.0;
    let b: f32 = (rgb.2 as f32)/255.0;
    let v: f32 = r.max(g).max(b);
    
    let c: f32 = v - r.min(g).min(b);

    let h: f32 =
        if c == 0.0 {
            0.0
        }else if v == r {
            60.0*(0.0+((g-b)/c))
        }else if v == g {
            60.0*(2.0+((b-r)/c))
        }else if v == b {
            60.0*(4.0+((r-g)/c))
        }else{
            unreachable!();
        };
    let s: f32 = if v == 0.0 {0.0} else {c/v};

    return (h as i32, (s*255.0) as i32, (v*255.0) as i32);
}


fn pixel_green_enough(rgba_slice: &[u8]) -> bool{
    assert!(rgba_slice.len() == 4);

    let r: i32 = rgba_slice[0] as i32;
    let g: i32 = rgba_slice[1] as i32;
    let b: i32 = rgba_slice[2] as i32;

    let (h, _s, _v) = rgb_to_hsv((r, g, b));

    let how_far_from_green:i32 = (h - 120).abs();
    return how_far_from_green < 80;
}

fn game_of_life_mask(mask: &Vec2D<bool>) -> Vec2D<bool>{
    let mut output: Vec2D<bool> = Vec2D::new(vec![false; (mask.w*mask.h) as usize], mask.h, mask.w);
    for y in 0..mask.h {
        for x in 0..mask.w {
            let mut num_neighbors: u32 = 0;
            for dy in -1i32..=1i32 {
                for dx in -1i32..=1i32{
                    let rx: i32 = x as i32 + dx;
                    let ry: i32 = y as i32 + dy;
                    if (rx > 0) && (ry > 0) {
                        if mask.check_in_range(ry as u32, rx as u32) {
                            if *mask.index(ry as u32, rx as u32) {
                                num_neighbors += 1;
                            }
                        }
                    }
                }
            }
            output.index_set_val(y, x, num_neighbors >= 4);
        }
    }
    return output;
}

fn bloat_mask(mask: &Vec2D<bool>) -> Vec2D<bool>{
    let mut output: Vec2D<bool> = Vec2D::new(vec![false; (mask.w*mask.h) as usize], mask.h, mask.w);
    for y in 0..mask.h {
        for x in 0..mask.w {
            'check_neighbors: for dy in -1i32..1i32 {
                for dx in -1i32..1i32{
                    let rx: i32 = x as i32 + dx;
                    let ry: i32 = y as i32 + dy;
                    if (rx > 0) && (ry > 0) {
                        if mask.check_in_range(ry as u32, rx as u32) {
                            if *mask.index(ry as u32, rx as u32) {
                                output.index_set_val(y, x, true);
                                break 'check_neighbors;
                            }
                        }
                    }
                }
            }
        }
    }
    return output;
}

fn plant_thresholding_mask(rgba_bytes: &Vec3D<u8>) -> Vec2D<bool>{
    let mut mask: Vec2D<bool> = Vec2D::new(vec![false; (rgba_bytes.w*rgba_bytes.h) as usize], rgba_bytes.h, rgba_bytes.w);

    for y in 0..rgba_bytes.h{
        for x in 0..rgba_bytes.w{
            if pixel_green_enough(rgba_bytes.index_2d(y, x)){
                mask.index_set_val(y, x, true);
            }
        }
    }

    return mask;
}


