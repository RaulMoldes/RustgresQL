use std::hash::{Hash, Hasher};

const ENDIANESS:bool = cfg!(target_endian = "little"); // True if little endian, false if big endian
const MAX_INT4_SIZE: usize = 4; // 4 bytes for a 32-bit integer
const MAX_FLOAT_SIZE: usize = 8; // 8 bytes for a 64-bit float
const MAX_STR_SIZE: usize = 32; // 32 bytes for a text field
const BOOLEAN_SIZE: usize = 1; // 1 byte for a boolean






#[derive(Debug, Clone, PartialEq,)]
pub enum DataType {
    /// Represents a variable-length character string with a maximum length of 255 characters.
    Varchar(String),  // Varchar with a maximum length of 255 characters
    Int32(i32),           // INT32 4 bytes
    Float64(f64),         // FLOAT64  8 bytes
    Bool(bool),           // BOOL 1 byte
    Null,                 // NULL represented as  bitmap of 1 byte
}


impl DataType {
    pub fn get_type(&self) -> u8 {
        match self {
            DataType::Varchar(_) => 0x01,
            DataType::Int32(_) => 0x02,
            DataType::Float64(_) => 0x03,
            DataType::Bool(_) => 0x04,
            DataType::Null => 0x00,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            DataType::Varchar(value) => value.clone(),
            DataType::Int32(value) => value.to_string(),
            DataType::Float64(value) => value.to_string(),
            DataType::Bool(value) => value.to_string(),
            DataType::Null => "NULL".to_string(),
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            DataType::Int32(value) => *value,
            _ => panic!("Cannot convert to integer"),
        }
    }

    pub fn as_float(&self) -> f64 {
        match self {
            DataType::Float64(value) => *value,
            _ => panic!("Cannot convert to float"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            DataType::Bool(value) => *value,
            _ => panic!("Cannot convert to boolean"),
        }
    }
}

impl Eq for DataType {}

impl Hash for DataType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_type().hash(state);
        match self {
            DataType::Varchar(value) => value.hash(state),
            DataType::Int32(value) => value.hash(state),
            DataType::Float64(value) => value.to_bits().hash(state),
            DataType::Bool(value) => value.hash(state),
            DataType::Null => 0.hash(state),
        }
    }
}

// TRAIT SERIALIZABLE
// THIS IS MY IMPLEMENTATION OF THE SERIALIZABLE TRAIT
// I WILL USE THIS TRAIT TO SERIALIZE AND DESERIALIZE DATA TYPES
// THIS WAY I WILL BE ABLE TO SERIALIZE AND DESERIALIZE DATA CATALOGS, PAGES AND EVEN AN ENTIRE DATABASE.
pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(buffer: &[u8], offset: &mut usize) -> Self where Self: Sized;

    fn serialize_list<T: Serializable>(data: &[T]) -> Vec<u8> where Self: Sized {
        let list_len = DataType::Int32(data.len() as i32);
        let mut result = list_len.serialize();
        for item in data {
            result.extend(item.serialize());
        }
        result
    }

    fn serialize_vecdeque<T: Serializable>(data: &std::collections::VecDeque<T>) -> Vec<u8> where Self: Sized {
        let list_len = DataType::Int32(data.len() as i32);
        let mut result = list_len.serialize();
        for item in data {
            result.extend(item.serialize());
        }
        result
    }

    fn deserialize_list(buffer: &[u8], offset: &mut usize) -> Vec<Self> where Self: Sized {
        // Deserializa la longitud de la lista
        let len = DataType::deserialize(buffer, offset);
        
        let mut result = Vec::new();
        for _ in 0..len.as_int() {
            result.push(Self::deserialize(buffer, offset));
        }
        result
    }

    fn deserialize_vecdeque(buffer: &[u8], offset: &mut usize) -> std::collections::VecDeque<Self> where Self: Sized {
        println!("Buffer: {:?}", buffer[*offset..*offset+10].to_vec());
        // Deserializa la longitud de la lista
        let len = DataType::deserialize(buffer, offset);
        println!("Len: {:?}", len);
        let mut result = std::collections::VecDeque::new();
        for _ in 0..len.as_int() {
            result.push_back(Self::deserialize(buffer, offset));
        }
        result
    }

    fn serialize_hashmap<T: Serializable>(data: &std::collections::HashMap<DataType, T>) -> Vec<u8> where Self: Sized {
        let list_len = DataType::Int32(data.len() as i32);
        let mut result = list_len.serialize();
        for (key, value) in data {
            result.extend(key.serialize());
            result.extend(value.serialize());
        }
        result
    }

    fn deserialize_hashmap(buffer: &[u8], offset: &mut usize) -> std::collections::HashMap<DataType, Self> where Self: Sized {
        // Deserializa la longitud de la lista
        let len = DataType::deserialize(buffer, offset);
        let mut result = std::collections::HashMap::new();
        for _ in 0..len.as_int() {
            let key = DataType::deserialize(buffer, offset);
            let value = Self::deserialize(buffer, offset);
            result.insert(key, value);
        }
        result
    }
}

// Implementation of the Serializable trait for DataType
impl Serializable for DataType {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        match self {
            DataType::Varchar(value) => {
                buffer.push(0x01); // Type marker for VARCHAR
                let len = value.len() as u8;
                if len >  MAX_STR_SIZE as u8 {
                    panic!("String length exceeds maximum length");
                }
                
                // String length (1 byte)
                buffer.push(len);
                // Convert the string to bytes and add a padding
                let mut padded_value = value.as_bytes().to_vec();
                let padding_size = MAX_STR_SIZE - len as usize;
                if padding_size > 0 {
                        padded_value.extend(vec![0u8; padding_size]); // Add zeros for padding
                }

                // The underlying buffer is extended with the bytes of the string
                buffer.extend(padded_value);
                         
                
            }

            DataType::Int32(value) => {
                buffer.push(0x02); // Type marker for INT32
                let bytes = if ENDIANESS {
                    value.to_le_bytes()
                } else {
                    value.to_be_bytes()
                };
                // Serialization of INT32 (4 bytes)
                buffer.extend(&bytes);
            }

            DataType::Float64(value) => {
                buffer.push(0x03); // Type marker for FLOAT64
                let bytes = if ENDIANESS {
                    value.to_le_bytes()
                } else {
                    value.to_be_bytes()
                };
                // Serialization of FLOAT64 (8 bytes)
                buffer.extend(&bytes);
            }

            DataType::Bool(value) => {
                buffer.push(0x04);  // Type marker for BOOL
                // Serialization of BOOL (1 byte)
                buffer.push(*value as u8);
            }

            DataType::Null => {
                buffer.push(0x00); // Type marker for NULL
                // Serialization of NULL (1 byte)
                buffer.push(0u8);  // NULL is represented as a bitmap of 1 byte
            }
        }

        buffer
    }


    // Deserializes a datatype
    // The function reads the type marker and then deserializes the data accordingly
    // the offset is updated to point to the next byte after the deserialized data
    fn deserialize(buffer: &[u8], offset: &mut usize) -> Self where Self: Sized {
        let data_type = buffer[*offset];
        *offset += 1;

        match data_type {
            0x01 => { // Varchar
                let len = buffer[*offset] as usize;
                *offset += 1;
                let value = String::from_utf8(buffer[*offset..*offset + len].to_vec()).unwrap();
                *offset += MAX_STR_SIZE;
                DataType::Varchar(value)
            }

            0x02 => { // INT32
                let mut bytes = [0u8; MAX_INT4_SIZE];
                bytes.copy_from_slice(&buffer[*offset..*offset + MAX_INT4_SIZE]);
                *offset += MAX_INT4_SIZE;
                let value = if ENDIANESS {
                    i32::from_le_bytes(bytes)
                } else {
                    i32::from_be_bytes(bytes)
                };
                DataType::Int32(value)
            }

            0x03 => { // FLOAT64
                let mut bytes = [0u8; MAX_FLOAT_SIZE];
                bytes.copy_from_slice(&buffer[*offset..*offset + MAX_FLOAT_SIZE]);
                *offset += MAX_FLOAT_SIZE;
                let value = if ENDIANESS {
                    f64::from_le_bytes(bytes)
                } else {
                    f64::from_be_bytes(bytes)
                };
                DataType::Float64(value)
            }

            0x04 => { // BOOL
                let value = buffer[*offset] != 0;
                *offset += BOOLEAN_SIZE;
                DataType::Bool(value)
            }

            _ => { 
                *offset += 1;
                DataType::Null 
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; // Import symbols from the parent module

    // Helper function to compare two DataType values
    fn assert_eq_data(expected: DataType, result: DataType) {
        assert_eq!(expected, result, "Expected {:?}, got {:?}", expected, result);
    }

    // Unit test for the DataType enum
    #[test]
    fn test_individual() {
        use DataType::*;
        let tests = vec![
            Varchar("Hello".to_string()),
            Int32(42),
            Float64(4.4849),
            Bool(true),
            Null,
        ];

        for test in tests {
            let serialized = test.serialize();
            let mut offset = 0;
            let deserialized = DataType::deserialize(&serialized, &mut offset);
            println!("Serialized: {:?}, Deserialized: {:?}", serialized, deserialized);
            assert_eq_data(test.clone(), deserialized);
        }
    }

    // Test if we can serialize and deserialize a list of data types
    #[test]
    fn test_list() {
        use DataType::*;
        let data_list = vec![
            Varchar("Test".to_string()),
            Int32(123),
            Float64(4.54884),
            Bool(false),
            Null,
        ];

        let serialized = DataType::serialize_list(&data_list);
        let deserialized_list = Serializable::deserialize_list(&serialized, &mut 0);
        assert_eq!(data_list, deserialized_list, "Mismatch in list lengths");
        for (expected, result) in data_list.iter().zip(deserialized_list) {
            assert_eq_data(expected.clone(), result.clone());
        }
    }

    // Test edge cases for serialization and deserialization
    #[test]
    fn test_edge_cases() {
        use DataType::*;
        // String vacío
        let empty_string = Varchar("".to_string());
        let serialized_empty = empty_string.serialize();
        let mut offset = 0;
        let deserialized_empty = DataType::deserialize(&serialized_empty, &mut offset);
        assert_eq_data(empty_string, deserialized_empty);

        // Valores extremos
        let int_min = Int32(i32::MIN);
        let int_max = Int32(i32::MAX);
        let float_min = Float64(f64::MIN);
        let float_max = Float64(f64::MAX);

        for test in [int_min, int_max, float_min, float_max] {
            let serialized = test.serialize();
            let mut offset = 0;
            let deserialized = DataType::deserialize(&serialized, &mut offset);

            assert_eq_data(test.clone(), deserialized);
        }
    }
}