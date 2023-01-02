mod image_reader;
mod andy_vectors;
mod green_masking;

use crate::andy_vectors::Vec3D as Vec3D;
use crate::andy_vectors::Vec2D as Vec2D;



fn main(){
    let mut image_data: Vec3D<u8> = image_reader::read_image_into_vec("plant4.jpg");

    let green_mask  : Vec2D<bool> = green_masking::plant_thresholding_mask(&image_data); 
    let asdf        : Vec2D<bool> = green_masking::bloat_mask(&green_mask);
    let bloated_mask: Vec2D<bool> = green_masking::game_of_life_mask(&asdf);
    //let bloated_mask: Vec2D<bool> = plant_thresholding_mask(&image_data); 

    for y in 0..image_data.h {
        for x in 0..image_data.w {
            if !*bloated_mask.index(y, x) {
                image_data.index_set_val(y, x, 0, 255);
                image_data.index_set_val(y, x, 1, 0);
                image_data.index_set_val(y, x, 2, 255);
            }
        }
    }

    let output = image_reader::make_image_from_vec(image_data);

    output.save("out.png").expect("failed to save");
}

