// module catalog
// src/catalog.rs
// This module contains the implementation of the database catalog.
use crate::storagemanager::serialization::{DataType, Serializable};
use crate::storagemanager::fileops::{ManagedFile, SmallFile};


pub type ObjectId = DataType;


/// Column of a table
#[derive(Debug)]
/// Represents a column in a database table.
struct Column {
    oid: ObjectId,
    name: DataType,
    max_value: DataType,
    min_value: DataType,
    constraints: Vec<Constraint>,

}


// A column can be serialized and deserialized
impl Serializable for Column {

    fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        serialized.extend(self.oid.serialize());
        serialized.extend(self.name.serialize());       
        serialized.extend(self.max_value.serialize());
        serialized.extend(self.min_value.serialize());
        serialized.extend(Constraint::serialize_list(&self.constraints));
        serialized
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self {
        let oid = DataType::deserialize(serialized, offset);
        let name = DataType::deserialize(serialized, offset);
        let max_value = DataType::deserialize(serialized, offset);
        let min_value = DataType::deserialize(serialized, offset);
        let constraints = Constraint::deserialize_list(serialized, offset);
        Column { oid, name, max_value, min_value, constraints }
    

}
}



// An index of a table.
// For now the attribute columns is a list of strings, but it should be a list of columns in the future. 
// As the column is Serializable, the index can be serialized and deserialized.
#[derive(Debug)]
struct Index {
    oid: ObjectId,
    name: DataType,
    columns: Vec<DataType>,
    unique: DataType,
}

impl Serializable for Index {
    fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        serialized.extend(self.oid.serialize());
        serialized.extend(self.name.serialize());
        serialized.extend(DataType::serialize_list(&self.columns));
        serialized.extend(self.unique.serialize());
        serialized
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self {
        let oid = DataType::deserialize(serialized, offset);
        let name = DataType::deserialize(serialized, offset);
        let columns = DataType::deserialize_list(serialized, offset);
        let unique = DataType::deserialize(serialized, offset);
        Index { oid, name, columns, unique }
    }
}


// A table in a database.
// A table has a name, a list of columns and a list of indexes.
// This is my implementation of a TableSchema, which would just be a list of this tables.
#[derive(Debug)]
struct Table {
    oid: ObjectId,
    name: DataType,
    columns: Vec<Column>,
    indexes: Vec<Index>,

}

impl Serializable for Table {
    fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        serialized.extend(self.oid.serialize());
        serialized.extend(self.name.serialize());
        serialized.extend(Column::serialize_list(&self.columns));
        serialized.extend(Index::serialize_list(&self.indexes));
  
        serialized
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self {
        let oid = DataType::deserialize(serialized, offset);
        let name = DataType::deserialize(serialized, offset);
        let columns = Column::deserialize_list(serialized, offset);
        let indexes = Index::deserialize_list(serialized, offset);
    
        
        Table {oid, name, columns, indexes}
    }
}


// Easy implementation of a constraint
// A constraint has a name and a type.
// This is kept easy for now as we are starting the development.
#[derive(Debug)]
struct Constraint {
    oid: ObjectId,
    name: DataType,
    dtype: DataType,

}

impl Serializable for Constraint {
    fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        serialized.extend(self.oid.serialize());
        serialized.extend(self.name.serialize());
        serialized.extend(self.dtype.serialize());
        serialized
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self {
        let oid = DataType::deserialize(serialized, offset);
        let name = DataType::deserialize(serialized, offset);
        let dtype = DataType::deserialize(serialized, offset);
        Constraint { oid, name, dtype}
    }
}


// THe data catalog is the main structure that holds all the tables in the database.
// It has a file where it is stored and a list of tables.
// The file should be a ManagedFile, which implements the SmallFile trait, therefore it can be read and written into memory.
#[derive(Debug)]
struct DataCatalog {
    file: ManagedFile, // This is the file where the data catalog is stored, it should be a ManagedFile
    // Implements the SmallFile trait so it can be read and written into memory
    tables: Vec<Table>, // Schema of the database

}


// Some utility functions for the DataCatalog
impl DataCatalog {
    fn new(path: String) -> DataCatalog {
        let file = ManagedFile::new(&path);
        DataCatalog {
            file,
            tables: Vec::new(),
        }
    }

    fn set_file(&mut self, path: String) {
        self.file = ManagedFile::new(&path);
    }

    fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }
}


// The DataCatalog can be serialized and deserialized
impl Serializable for DataCatalog {
    fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        serialized.extend(Table::serialize_list(&self.tables));
        serialized
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self {
        let tables = Table::deserialize_list(serialized, offset);
        DataCatalog {
            file: ManagedFile::new("data/catalog.db"),
            tables,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    // Test to check if I can serialize and deserialize an entire data catalog without saving to disk.
    #[test]
    fn test_data_catalog_serialization() {
        let mut data_catalog = DataCatalog::new("data/catalog.db".to_string());
        let table = Table { oid: DataType::Int32(1),
            name: DataType::Varchar("table".to_string()),
            columns: vec![Column { oid: DataType::Int32(2),
                name: DataType::Varchar("column".to_string()),
                max_value: DataType::Int32(100),
                min_value: DataType::Int32(0),
                constraints: vec![Constraint {
                    oid: DataType::Int32(3),
                    name: DataType::Varchar("constraint".to_string()),
                    dtype: DataType::Varchar("type".to_string()),
                }],
            }],
            indexes: vec![Index {
                oid: DataType::Int32(4),
                name: DataType::Varchar("index".to_string()),
                columns: vec![DataType::Varchar("column".to_string())],
                unique: DataType::Bool(true),
            }],
        };
        data_catalog.add_table(table);
        let serialized = data_catalog.serialize();
        let mut offset = 0;
        let deserialized = DataCatalog::deserialize(&serialized, &mut offset);
        assert_eq!(data_catalog.tables.len(), deserialized.tables.len());
        assert_eq!(data_catalog.tables[0].name, deserialized.tables[0].name);
    }



    // This test aims to check that I can serialize and store the data catalog in a file
    // and then deserialize it back to memory
    #[test]
    fn test_data_catalog_storage(){
        let mut data_catalog = DataCatalog::new("data/catalog.db".to_string());
        let table = Table {
            oid: DataType::Int32(1),
            name: DataType::Varchar("table".to_string()),
            columns: vec![Column {
                oid: DataType::Int32(2),
                name: DataType::Varchar("column".to_string()),
                max_value: DataType::Int32(100),
                min_value: DataType::Int32(0),
                constraints: vec![Constraint {
                    oid: DataType::Int32(3),
                    name: DataType::Varchar("constraint".to_string()),
                    dtype: DataType::Varchar("type".to_string()),
                }],
            }],
            indexes: vec![Index {
                oid: DataType::Int32(4),
                name: DataType::Varchar("index".to_string()),
                columns: vec![DataType::Varchar("column".to_string())],
                unique: DataType::Bool(true),
            }],
        };
        data_catalog.add_table(table);
        let serialized = data_catalog.serialize();
        data_catalog.file.write_all(&serialized).unwrap();
        let mut deserialized = DataCatalog::deserialize(&data_catalog.file.read_to_end().unwrap(), &mut 0);
        deserialized.set_file("data/catalog.db".to_string());
        assert_eq!(data_catalog.tables.len(), deserialized.tables.len());
        assert_eq!(data_catalog.tables[0].name, deserialized.tables[0].name);


    }
}