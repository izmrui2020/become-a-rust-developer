use std::collections::HashMap;

use crate::reader::Reader;

pub struct MultiFormatReader {
    pub readers: Vec<Box<dyn Reader>>,
    pub hints: HashMap<String, u8>,
}

impl MultiFormatReader {
    
}