use byteorder::{ByteOrder, LittleEndian, BigEndian};


pub trait Binaryable {
    fn as_binary(&self) -> Vec<u8>;
    fn from_binary(binary_data: Vec<u8>) -> Self;

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
    raw_bytes.to_vec()
}

pub fn pop_bytes_from_vec(binary_data: Vec<u8>, amount: u32) -> (Vec<u8>, Vec<u8>) {
    let (a, b) = binary_data.split_at(amount as usize);
    (
        a.to_vec(),
        b.to_vec(),
    )
}

pub fn pop_f32(binary_data: Vec<u8>) -> (f32, Vec<u8>) {
    let (value_data, binary_data) = pop_bytes_from_vec(binary_data, 4);
    (
        LittleEndian::read_f32(value_data.as_slice()),
        binary_data,
    )
}

pub fn pop_i32(binary_data: Vec<u8>) -> (i32, Vec<u8>) {
    let (value_data, binary_data) = pop_bytes_from_vec(binary_data, 4);
    (
        BigEndian::read_i32(value_data.as_slice()),
        binary_data,
    )
}

pub fn pop_u32(binary_data: Vec<u8>) -> (u32, Vec<u8>) {
    let (value_data, binary_data) = pop_bytes_from_vec(binary_data, 4);
    (
        BigEndian::read_u32(value_data.as_slice()),
        binary_data,
    )
}

pub fn pop_u8(binary_data: Vec<u8>) -> (u8, Vec<u8>) {
    let (value_data, binary_data) = pop_bytes_from_vec(binary_data, 1);
    (
        value_data[0],
        // BigEndian::read_u8(value_data.as_slice()),
        binary_data,
    )
}

pub fn pop_padded(binary_data: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let (pad_amount, binary_data) = pop_u32(binary_data);
    pop_bytes_from_vec(binary_data, pad_amount)
}

