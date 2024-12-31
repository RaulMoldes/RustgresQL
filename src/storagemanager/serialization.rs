
const ENDIANESS:bool = cfg!(target_endian = "little"); // True if little endian, false if big endian
const MAX_INT4_SIZE: usize = 4; // 4 bytes for a 32-bit integer
const MAX_FLOAT_SIZE: usize = 8; // 8 bytes for a 64-bit float
const MAX_STR_SIZE: usize = 32; // 32 bytes for a text field
const BOOLEAN_SIZE: usize = 1; // 1 byte for a boolean

#[derive(Debug, Clone, PartialEq)]
enum DataType {
    Varchar(String),  // Varchar con longitud máxima de n (< 255) y el contenido de la cadena
    Int32(i32),           // INT32 ocupa 4 bytes
    Float64(f64),         // FLOAT64 ocupa 8 bytes
    Bool(bool),           // BOOL ocupa 1 byte
    Null,                 // NULL representado por un bitmap de 1 byte
}


// TRAITS
pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(buffer: &[u8], offset: &mut usize) -> Self where Self: Sized;
    // Serializa una lista de tipos de datos
    fn serialize_list(data: &[Self]) -> Vec<u8> where Self: Sized {
        let mut result = Vec::new();
        for item in data {
            result.extend(item.serialize());
        }
        result
    }

    fn deserialize_list(data: &[u8]) -> Vec<Self> where Self: Sized {
        let mut offset = 0;
        let mut result = Vec::new();
        while offset < data.len() {
            result.push(Self::deserialize(data, &mut offset));
        }
        result
    }
}

// Implementación de serialización para cada tipo de dato.
impl Serializable for DataType {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        match self {
            DataType::Varchar(value) => {
                buffer.push(0x01); // Marca de tipo para Varchar
                // Se asegura que el string no exceda el tamaño máximo
                let len = value.len() as u8;
                if len >  MAX_STR_SIZE as u8 {
                    panic!("String length exceeds maximum length");
                }
                
                // Primer byte: longitud del string (1 byte)
                buffer.push(len);
                // Convertir el string a bytes
                let mut padded_value = value.as_bytes().to_vec();
                let padding_size = MAX_STR_SIZE - len as usize;
                if padding_size > 0 {
                        padded_value.extend(vec![0u8; padding_size]); // Añadir ceros de padding
                }

                // Los bytes restantes: el contenido del string con padding
                buffer.extend(padded_value);
                         
                
            }

            DataType::Int32(value) => {
                buffer.push(0x02); // Marca de tipo para INT32
                let bytes = if ENDIANESS {
                    value.to_le_bytes()
                } else {
                    value.to_be_bytes()
                };
                // Serialización de INT32 (4 bytes)
                buffer.extend(&bytes);
            }

            DataType::Float64(value) => {
                buffer.push(0x03); // Marca de tipo para FLOAT64
                let bytes = if ENDIANESS {
                    value.to_le_bytes()
                } else {
                    value.to_be_bytes()
                };
                // Serialización de FLOAT64 (8 bytes)
                buffer.extend(&bytes);
            }

            DataType::Bool(value) => {
                buffer.push(0x04);  // Marca de tipo para BOOL
                // Serialización de BOOL (1 byte)
                buffer.push(*value as u8);
            }

            DataType::Null => {
                buffer.push(0x00); // Marca de tipo para NULL
                // Serialización de NULL (bitmap de 1 byte)
                buffer.push(0u8);  // Representamos NULL como un byte 0
            }
        }

        buffer
    }

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
    use super::*; // Importa todo lo necesario desde el módulo principal

    // Helper para comprobar igualdad y mostrar errores
    fn assert_eq_data(expected: DataType, result: DataType) {
        assert_eq!(expected, result, "Expected {:?}, got {:?}", expected, result);
    }

    // Prueba de serialización/deserialización individual
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

    // Prueba de serialización/deserialización de listas
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

        let serialized = Serializable::serialize_list(&data_list);
        let deserialized_list = Serializable::deserialize_list(&serialized);
        assert_eq!(data_list, deserialized_list, "Mismatch in list lengths");
        for (expected, result) in data_list.iter().zip(deserialized_list) {
            assert_eq_data(expected.clone(), result.clone());
        }
    }

    // Prueba de casos límite
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