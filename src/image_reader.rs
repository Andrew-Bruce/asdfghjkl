use image::io::Reader as ImageReader;
pub mod andy_vectors;
use andy_vectors::Vec3D as Vec3D;

fn read_image(filename: &str) -> image::RgbaImage{
    let img = ImageReader::open(filename)
        .expect(&("failed to open image ".to_owned() + filename))
        .decode()
        .expect(&("failed to decode image ".to_owned() + filename))
        .into_rgba8();
    return img;
}

pub fn read_image_into_vec(filename: &str) -> Vec3D<u8>{
    let img = read_image(filename);

    let img_bytes: Vec<u8> = img.as_raw().to_vec();
    let (w, h) = img.dimensions();
    
    return Vec3D::new(img_bytes, h, w, 4);
}

pub fn make_image_from_vec(data: Vec3D<u8>) -> image::RgbaImage{
    assert!(data.d == 4);
    
    return image::ImageBuffer::from_raw(data.w, data.h, data.data)
        .expect("making image from data failed");
}
