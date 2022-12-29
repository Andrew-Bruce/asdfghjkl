pub struct Vec3D<T>{
    pub data: Vec<T>,
    pub h: u32,
    pub w: u32,
    pub d: u32,
}

impl<T> Vec3D<T>{
    pub fn new(data: Vec<T>, h: u32, w: u32, d: u32) -> Vec3D<T>{
        assert!(data.len() == ((h*w*d) as usize), "data dimentions mismatch");
        return Vec3D{
            data: data,
            h: h,
            w: w,
            d: d,
        }
    }
    
    fn check_in_range(&self, h: u32, w: u32, d: u32) -> bool{
        return (h < self.h)
            && (w < self.w)
            && (d < self.d);
    }

    fn assert_in_range(&self, h: u32, w: u32, d: u32) -> (){
        assert!(self.check_in_range(h, w, d), "Vec3D out of bounds");
    }

    pub fn index(&self, h: u32, w: u32, d:u32) -> &T{
        self.assert_in_range(h, w, d);
        return &self.data[(d + (w*self.d) + (h*self.d*self.w)) as usize];
    }

    pub fn index_2d(&self, h: u32, w: u32) -> &[T]{
        self.assert_in_range(h, w, 0);

        let start_slice:usize = ((w*self.d) + (h*self.d*self.w)) as usize;
        return &self.data[start_slice..start_slice+(self.d as usize)];
    }
    
    pub fn index_set_val(&mut self, h: u32, w: u32, d: u32, val: T) -> (){
        self.assert_in_range(h, w, d);
        self.data[(d + (w*self.d) + (h*self.d*self.w)) as usize] = val;
    }
 
}

pub struct Vec2D<T>{
    pub data: Vec<T>,
    pub h: u32,
    pub w: u32,
}

impl<T> Vec2D<T>{
    pub fn new(data: Vec<T>, h: u32, w: u32) -> Vec2D<T>{
        assert!(data.len() == ((h*w) as usize), "data dimentions mismatch");
        return Vec2D{
            data: data,
            h: h,
            w: w,
        }
    }
    
    pub fn check_in_range(&self, h: u32, w: u32) -> bool{
        return (h < self.h)
            && (w < self.w);
    }

    fn assert_in_range(&self, h: u32, w: u32) -> (){
        assert!(self.check_in_range(h, w), "Vec2D out of bounds");
    }

    pub fn index(&self, h: u32, w: u32) -> &T{
        self.assert_in_range(h, w);
        return &self.data[(w + (h*self.w)) as usize];
    }

    pub fn index_set_val(&mut self, h: u32, w: u32, val: T) -> (){
        self.assert_in_range(h, w);
        self.data[(w + (h*self.w)) as usize] = val;
    }
}


