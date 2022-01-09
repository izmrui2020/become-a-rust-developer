
#[derive(Copy, Clone, Debug)]
pub struct ResultPoint {
    x: f64,
    y: f64,
}

pub trait ResultPointTrait {
    fn get_x(&self) -> f64;
    fn set_x(&mut self, x: f64);
    fn get_y(&self) -> f64;
    fn set_y(&mut self, y: f64);

    fn order_best_patterms();
    fn distance();
    fn cross_product_z();
}

impl ResultPointTrait for ResultPoint {

    fn get_x(&self) -> f64 {
        self.x
    }

    fn set_x(&mut self, x:f64) {
        self.x = x;
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    fn order_best_patterms() {
        todo!()
    }

    fn distance() {
        todo!()
    }

    fn cross_product_z() {
        todo!()
    }
}