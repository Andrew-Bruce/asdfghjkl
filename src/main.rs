mod image_reader;
use image_reader::andy_vectors::Vec3D as Vec3D;
use image_reader::andy_vectors::Vec2D as Vec2D;

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
    return how_far_from_green < 50;
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

fn main(){
    let mut image_data: Vec3D<u8> = image_reader::read_image_into_vec("plant4.jpg");
    
    let green_mask: Vec2D<bool> = plant_thresholding_mask(&image_data);

    for y in 0..image_data.h {
        for x in 0..image_data.w {
            if !*green_mask.index(y, x) {
                image_data.index_set_val(y, x, 0, 0);
                image_data.index_set_val(y, x, 1, 0);
                image_data.index_set_val(y, x, 2, 0);
            }

        }
    }

    let output = image_reader::make_image_from_vec(image_data);

    output.save("out.png").expect("failed to save");
}

