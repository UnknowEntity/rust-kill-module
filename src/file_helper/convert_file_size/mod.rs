use byte_unit::Byte;

pub fn size(byte: u128) -> String {
    let size = Byte::from_bytes(byte);
    size.get_appropriate_unit(true).to_string()
}