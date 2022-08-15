pub struct Window {
    name: String,
    width: i32,
    height: i32
}

impl Window {
    pub fn new(name: String, width: i32, height: i32) -> Self {
        Window {
            name,
            width,
            height
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}
