

pub trait Binaryable {
    fn as_binary(&self) -> Vec<u8>;

    fn as_padded_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();
        let original_binary_data = self.as_binary();

        binary_data.extend(u32_as_bytes(original_binary_data.len() as u32));
        binary_data.extend(original_binary_data);

        binary_data
    }
}


pub fn vec_as_bytes<T: Binaryable>(data: &Vec<T>) -> Vec<u8> {
    let mut binary_data: Vec<u8> = Vec::new();

    let mut vector_binary_data: Vec<u8> = Vec::new();
    for entity in data.iter() {
        vector_binary_data.extend(entity.as_padded_binary());
    }
    binary_data.extend(u32_as_bytes(vector_binary_data.len() as u32));
    binary_data.extend(vector_binary_data);

    binary_data
}


pub fn u32_as_bytes(input: u32) -> Vec<u8> {
    let mut binary_data: Vec<u8> = vec![0; 4];

    binary_data[3] = (input >> 0) as u8;
    binary_data[2] = (input >> 8) as u8;
    binary_data[1] = (input >> 16) as u8;
    binary_data[0] = (input >> 24) as u8;

    binary_data
}


pub fn i32_as_bytes(input: i32) -> Vec<u8> {
    let mut binary_data: Vec<u8> = vec![0; 4];

    binary_data[3] = (input >> 0) as u8;
    binary_data[2] = (input >> 8) as u8;
    binary_data[1] = (input >> 16) as u8;
    binary_data[0] = (input >> 24) as u8;

    binary_data
}

pub fn f32_as_bytes(input: f32) -> Vec<u8> {
    let mut binary_data: Vec<u8> = vec![0; 4];

    let raw_bytes: [u8; 4] = unsafe { std::mem::transmute(input) };
    /*
    binary_data[3] = (input >> 0) as u8;
    binary_data[2] = (input >> 8) as u8;
    binary_data[1] = (input >> 16) as u8;
    binary_data[0] = (input >> 24) as u8;
    */

    raw_bytes.to_vec()
    // binary_data
}

