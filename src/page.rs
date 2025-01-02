// MOD page
// src/page.rs
// This module contains the implementation of the page module.
// A page is a unit of storage in the database.
// A page has a header, a list of slots and a list of tuples.
use std::collections::VecDeque;
use crate::storagemanager::serialization::{Serializable, DataType};


const MAX_PAGE_SIZE: u16 = 4096; // SELECTED MAX PAGE SIZE


// CUSTOM TYPES
pub type PageId = DataType;
type TupleId = DataType;


// STRUCT HEADER
// The header of the page contains metadata about the page
#[derive(Debug)]
struct Header{
    page_type: PageType,
    free_space: DataType, // AMOUNT OF FREE SPACE IN THE PAGE 
    page_number: PageId, // PAGE NUMBER
    next_page: PageId,
    last_slot: TupleId, // POINTER TO THE TUPLE ID OF THE LAST SLOT
    offset: DataType, // OFFSET WHERE THE LAST TUPLE STARTS
}

impl Header{
    fn new(page_type: PageType, page_number: PageId, next_page: PageId, free_space: Option<DataType>) -> Self{
        Header{
            page_type,
            free_space: free_space.unwrap_or(DataType::Int32(MAX_PAGE_SIZE as i32)),
            page_number,
            next_page,
            last_slot: DataType::Int32(0), // INITIALLY NO SLOTS
            offset: DataType::Int32(MAX_PAGE_SIZE as i32 - 1), // INITIALLY NO OFFSET
        }
    }

    fn set_free_space(&mut self, free_space: DataType){
        self.free_space = free_space;
    }
}

impl Serializable for Header {
    fn serialize(&self) -> Vec<u8>{
        let mut serialized = Vec::new();
        serialized.extend(self.page_type.serialize());
        serialized.extend(self.free_space.serialize());
        serialized.extend(self.page_number.serialize());
        serialized.extend(self.next_page.serialize());
        serialized
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self{
        let page_type = PageType::deserialize(serialized, offset);
        let free_space = DataType::deserialize(serialized, offset);
        let page_number = DataType::deserialize(serialized, offset);
        let next_page = DataType::deserialize(serialized, offset);
        Header::new(page_type,  page_number, next_page, Some(free_space))
    }
}

#[derive(Debug)]
pub enum PageType{
    Data(DataType), // Varchar:: 'DATA'
    Index(DataType), // Varchar:: 'INDEX'
}

impl Serializable for PageType{
    fn serialize(&self) -> Vec<u8>{
        match self{
            PageType::Data(data_type) => data_type.serialize(),
            PageType::Index(data_type) => data_type.serialize(),
        }
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self{
        let data_type = DataType::deserialize(serialized, offset);
        match data_type.as_string().as_str(){
            "DATA" => PageType::Data(data_type),
            "INDEX" => PageType::Index(data_type),
            _ => panic!("Invalid page type"),
        }
    }
}


#[derive(Debug)]
struct Slot{
    tuple_id: TupleId, // TUPLE ID
    offset: DataType, // OFFSET WHERE THE TUPLE STARTS
    length: DataType, // LENGTH OF THE TUPLE

}

impl Slot{
    fn new(tuple_id: TupleId, offset: DataType, length: DataType) -> Self{
        Slot{
            tuple_id,
            offset,
            length,
        }
    }
}

impl Serializable for Slot {
    fn serialize(&self) -> Vec<u8>{
        let mut serialized = Vec::new();
        serialized.extend(self.tuple_id.serialize());
        serialized.extend(self.offset.serialize());
        serialized.extend(self.length.serialize());
        serialized
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self{
        let tuple_id = DataType::deserialize(serialized, offset);
        let tuple_offset = DataType::deserialize(serialized, offset);
        let length = DataType::deserialize(serialized, offset);
        Slot::new(tuple_id, tuple_offset, length)
    }
}



#[derive(Debug, Clone)]
struct Tuple{
    tuple_id: TupleId,
    data: Vec<DataType>,
}

impl Tuple{
    fn new(tuple_id: TupleId, data: Vec<DataType>) -> Self{
        Tuple{
            tuple_id,
            data,
        }
    }
}

impl Serializable for Tuple {
    fn serialize(&self) -> Vec<u8>{
        let mut serialized = Vec::new();
        serialized.extend(self.tuple_id.serialize());
        serialized.extend(DataType::serialize_list(&self.data));
        serialized
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self{
        let tuple_id = DataType::deserialize(serialized, offset);
        let data = DataType::deserialize_list(serialized, offset);
        Tuple::new(tuple_id, data)
    }
}



// STRUCT PAGE
// A page is a unit of storage in the database
// A page has a header, a list of slots and a list of tuples
// MAX PAGE SIZE is 4096 bytes
// The page is serialized as follows:
// 1. Serialize the header  
// 2. Serialize the slots
// 3. Serialize the tuples
// The slots are stored at the beginning of the page and grow towards the end
// The tuples are stored at the end of the page and grow towards the beginning
#[derive(Debug)]
pub struct Page{
    header: Header,
    slots: VecDeque<Slot>,    
    data: VecDeque<Tuple>,
    

}

impl Page{
    fn new(header: Header, slots: Option<VecDeque<Slot>>, data: Option<VecDeque<Tuple>>) -> Self{

        
        
        Page{
            header,
            slots: slots.unwrap_or_default(),
            data: data.unwrap_or_default(),
        }
    }

    fn reduce_free_space(&mut self, reduce_by: i32){
        assert!(reduce_by <= self.get_free_space(), "Not enough free space: Free space: {}, Reduce by: {}", self.get_free_space(), reduce_by);
        let free_space = self.get_free_space() - reduce_by;
        self.header.set_free_space(DataType::Int32(free_space));
    }

    fn get_free_space(&self) -> i32{
        self.header.free_space.as_int()
    }

    fn append_tuple(&mut self, tuple_data: Vec<DataType>){
        // Logic to append a tuple to the page
        // 1. Create a new tuple with the tuple_id as the last_slot + 1
        // 2. Serialize the tuple and get the size
        // 3. Update the offset of the tuple

        println!("Initial offset {:?}", self.header.offset.as_int());
        let tuple_id = self.header.last_slot.as_int() + 1;
        let tuple = Tuple::new(DataType::Int32(tuple_id), tuple_data);
        let mut tuple_size = tuple.serialize().len() as i32;
        let mut offset = self.header.offset.as_int() - tuple_size;
    

        if self.header.last_slot.as_int() == 0 {
            // First tuple
            offset -= 5; // Reservar 5 bytes adicionales.
            tuple_size += 5;
        }
        println!("Updated offset {:?}", offset);
        println!("Tuple size {:?}", tuple_size);

        // Create a new slot
        let slot = Slot::new(DataType::Int32(tuple_id), DataType::Int32(offset), DataType::Int32(tuple_size));
        let slot_size = slot.serialize().len() as i32;
        // Update the header
        self.header.last_slot = DataType::Int32(tuple_id);
        self.header.offset = DataType::Int32(offset);
        self.reduce_free_space(slot_size);
        // Store the tuple and slot
        self.slots.push_back(slot);
        self.reduce_free_space(tuple_size);
        self.data.push_front(tuple);
        
    }
}


// PAGE SERIALIZATION IS A BIT COMPLEX
// 1. SERIALIZE THE HEADER
// 2. SERIALIZE THE SLOTS
// THE TUPLES ARE STORED AT THE END OF THE PAGE AND GROW TOWARDS THE BEGINNING
// THE SLOTS ARE STORED AT THE BEGINNING OF THE PAGE AND GROW TOWARDS THE END
impl Serializable for Page {
    fn serialize(&self) -> Vec<u8>{
        // ALLOCATE MAX_PAGE SIZE
        let mut serialized = vec![0; MAX_PAGE_SIZE as usize];
        // Fill the first bytes with the header
        let serialized_header = self.header.serialize();
        let slot_offset = serialized_header.len();
        serialized.splice(0..slot_offset, serialized_header.iter().cloned());
        // Fill the next bytes with the slot array
        
        let serialized_slots = Slot::serialize_vecdeque(&self.slots);
        let slots_size = serialized_slots.len();
        println!("Serialized slots: {:?}", serialized_slots);
        serialized.splice(slot_offset..slot_offset + slots_size, serialized_slots.iter().cloned());

        // FILL THE END OF THE PAGE WITH THE TUPLES
        let tuples = self.data.clone();
        // Serialize the tuples
        let serialized_tuples = Tuple::serialize_vecdeque(&tuples);
        println!("Serialized tuples: {:?}", serialized_tuples);

        // Place the tuples at the end of the page
        let tuple_offset = self.header.offset.as_int() as usize;
        let tuples_size = serialized_tuples.len();
        println!("Tuple offset: {:?}", tuple_offset);
        println!("Tuples size: {:?}", tuples_size);

        assert!(tuples_size <= MAX_PAGE_SIZE as usize - slot_offset - slots_size , "Not enough space: Tuples size: {}, Free space: {}", tuples_size, MAX_PAGE_SIZE as usize - slot_offset- slots_size);

        serialized.splice(tuple_offset..tuple_offset + tuples_size, serialized_tuples.iter().cloned());

        // Return the serialized page
        serialized
    }

    fn deserialize(serialized: &[u8], offset: &mut usize) -> Self{
        let header = Header::deserialize(serialized, offset);
        println!("Deserialized header: {:?}", header);
        // Deserialize the slots
        let slots = Slot::deserialize_vecdeque(serialized, offset);
        println!("Deserialized slots: {:?}", slots);

        // The tuples are stored at the end of the page
        // Get the last tuple offset

        let mut last_tuple_offset = slots.back().unwrap().offset.as_int() as usize;
       

    
        println!("Last tuple offset: {:?}", last_tuple_offset);

        // Deserialize the tuples
        let tuples = Tuple::deserialize_vecdeque(serialized, &mut last_tuple_offset);
        
       
        Page::new(header, Some(slots), Some(tuples))
    }
}

// Mod tests
// This module contains the tests for the page module
// The tests are run using the command cargo test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_serialization() {
        let header = Header::new(PageType::Data(DataType::Varchar("DATA".to_string())), DataType::Int32(1000), DataType::Int32(1), None);
        let serialized = header.serialize();
        let mut offset = 0;
        let deserialized = Header::deserialize(&serialized, &mut offset);
       
        assert_eq!(header.free_space, deserialized.free_space);
        assert_eq!(header.page_number, deserialized.page_number);
        assert_eq!(header.next_page, deserialized.next_page);
    }

    #[test]
    fn test_slot_serialization() {
        let slot = Slot::new(DataType::Int32(1), DataType::Int32(100), DataType::Int32(50));
        let serialized = slot.serialize();
        let mut offset = 0;
        let deserialized = Slot::deserialize(&serialized, &mut offset);
        assert_eq!(slot.tuple_id, deserialized.tuple_id);
        assert_eq!(slot.offset, deserialized.offset);
        assert_eq!(slot.length, deserialized.length);
    }

    #[test]
    fn test_tuple_serialization() {
        let tuple = Tuple::new(DataType::Int32(1), vec![DataType::Int32(10), DataType::Varchar("test".to_string())]);
        let serialized = tuple.serialize();
        let mut offset = 0;
        let deserialized = Tuple::deserialize(&serialized, &mut offset);
        assert_eq!(tuple.tuple_id, deserialized.tuple_id);
        assert_eq!(tuple.data, deserialized.data);
    }

    #[test]
    fn test_page_serialization() {
        
        let header = Header::new(PageType::Data(DataType::Varchar("DATA".to_string())), DataType::Int32(0), DataType::Int32(1), None);
       
        let tuple = vec![DataType::Int32(10), DataType::Varchar("test".to_string())];
        let mut page = Page::new(header, None, None);
        
        page.append_tuple(tuple);
        let serialized = page.serialize();
        let mut offset = 0;
        let deserialized = Page::deserialize(&serialized, &mut offset);
        
        assert_eq!(page.header.free_space, deserialized.header.free_space);
        assert_eq!(page.header.page_number, deserialized.header.page_number);
        assert_eq!(page.header.next_page, deserialized.header.next_page);
        assert_eq!(page.slots.len(), deserialized.slots.len());
        assert_eq!(page.data.len(), deserialized.data.len());
        assert_eq!(page.data[0].data, deserialized.data[0].data);
    }

    #[test]
    fn test_append_tuple() {
        let header = Header::new(PageType::Data(DataType::Varchar("DATA".to_string())), DataType::Int32(0), DataType::Int32(1), None);
        let mut page = Page::new(header, None, None);
        let tuple_data = vec![DataType::Int32(10), DataType::Varchar("test".to_string())];
        page.append_tuple(tuple_data.clone());
        assert_eq!(page.data.len(), 1);
        assert_eq!(page.slots.len(), 1);
        assert_eq!(page.data[0].data, tuple_data);
    }

    #[test]
    fn test_multiple_tuples(){

        
        let header = Header::new(PageType::Data(DataType::Varchar("DATA".to_string())), DataType::Int32(0), DataType::Int32(1), None);
       // print!("{:?}", header);
       // println!("{:?}", header.serialize().len());
        let tuple = vec![DataType::Int32(10), DataType::Varchar("test".to_string())];
        let mut page = Page::new(header, None, None);
        page.append_tuple(tuple);
        let tuple2 = vec![DataType::Int32(20), DataType::Varchar("test2".to_string())];
        page.append_tuple(tuple2);
        let tuple3 = vec![DataType::Int32(30), DataType::Varchar("test3".to_string())];
        page.append_tuple(tuple3);
        let serialized = page.serialize();
        let mut offset = 0;
        let deserialized = Page::deserialize(&serialized, &mut offset);
        
        assert_eq!(page.header.free_space, deserialized.header.free_space);
        assert_eq!(page.header.page_number, deserialized.header.page_number);
        assert_eq!(page.header.next_page, deserialized.header.next_page);
        assert_eq!(page.slots.len(), deserialized.slots.len());
        assert_eq!(page.data.len(), deserialized.data.len());
        for i in 0..page.data.len(){
            assert_eq!(page.data[i].data, deserialized.data[i].data);
        }
    }

}
