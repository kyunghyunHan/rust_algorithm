
#[derive(Default)]
pub struct Second {
    value:u64
}

impl Second {

    //->defular
    // pub fn new(value:u64)->Self {
    //     Self {value}
    // }

    pub fn value (&self)->u64 {
        self.value
    }
}

