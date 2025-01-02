// mod directory;
// the directory contains the locations of the pages and objects in the database.
// it is a hashmap of page_id to path and object_id to page_id.
// As the catalog, as it is a SmallFile, it can be load into memory and written to disk.
// The directory should be able to add and remove pages and objects, and get the objects for a page.

use std::collections::HashMap;


use crate::storagemanager::fileops::{ManagedFile, SmallFile};
use crate::page::{PageId, PageType, Page};
use crate::storagemanager::serialization::{DataType, Serializable};
use crate::catalog::ObjectId;


struct Directory {
    pages: HashMap<PageId, DataType>,
    objects: HashMap<ObjectId, PageId>,
    file: ManagedFile,
}

impl Directory {
    fn new(pages: Option<HashMap<PageId, DataType>>, objects: Option<HashMap<ObjectId, PageId>>) -> Self {
        Self {
            pages:pages.unwrap_or(HashMap::new()),
            objects:objects. unwrap_or(HashMap::new()),
            file: ManagedFile::new("data/directory.db"),
        }
    }

   fn add_page(&mut self, page_id: PageId, path: DataType){
         self.pages.insert(page_id, path);
   }

   fn add_object(&mut self, object_id: ObjectId, page_id: PageId){
        assert!(self.pages.contains_key(&page_id));
       self.objects.insert(object_id, page_id);
   }

   fn remove_object(&mut self, object_id: ObjectId){
       self.objects.remove(&object_id);
   }

   fn get_objects_for_page(&self, page_id: PageId) -> Vec<ObjectId>{
       self.objects.iter().filter(|(_, ref v)| **v == page_id).map(|(k, _)| k.clone()).collect()
   }

   fn remove_page(&mut self, page_id: PageId){
       self.pages.remove(&page_id);
   }

   fn get_page(&self, page_id: PageId) -> Option<&DataType>{
       self.pages.get(&page_id)
   }

   fn set_file(&mut self, path: &str){
       self.file = ManagedFile::new(path);
   }

}


impl Serializable for Directory{

    fn serialize(&self) -> Vec<u8> {
        // Use serialize_hashmap from DataType
        let mut serialized = Vec::new();
        serialized.extend(DataType::serialize_hashmap(&self.pages));
        serialized.extend(DataType::serialize_hashmap(&self.objects));
        serialized
       
    }

    fn deserialize(buffer: &[u8], offset: &mut usize) -> Self where Self: Sized {
        let pages = DataType::deserialize_hashmap(buffer, offset);
        let objects = DataType::deserialize_hashmap(buffer, offset);
        Directory { pages, objects , file: ManagedFile::new("data/directory.db")}
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_page() {
        let mut directory = Directory::new(None, None);
        let page_id = PageId::Int32(1);
        let data_type = DataType::Int32(42);
        directory.add_page(page_id.clone(), data_type.clone());
        assert_eq!(directory.get_page(page_id), Some(&data_type));
    }

    #[test]
    fn test_add_object() {
        let mut directory = Directory::new(None, None);
        let page_id = PageId::Int32(1);
        let object_id = ObjectId::Int32(1);
        let data_type = DataType::Int32(42);
        directory.add_page(page_id.clone(), data_type);
        directory.add_object(object_id.clone(), page_id.clone());
        assert_eq!(directory.objects.get(&object_id), Some(&page_id));
    }

    #[test]
    #[should_panic]
    fn test_add_object_without_page() {
        let mut directory = Directory::new(None, None);
        let object_id = ObjectId::Int32(1);
        let page_id = PageId::Int32(1);
        directory.add_object(object_id, page_id);
    }

    #[test]
    fn test_remove_object() {
        let mut directory = Directory::new(None, None);
        let page_id = PageId::Int32(1);
        let object_id = ObjectId::Int32(1);
        let data_type = DataType::Int32(42);
        directory.add_page(page_id.clone(), data_type);
        directory.add_object(object_id.clone(), page_id);
        directory.remove_object(object_id.clone());
        assert!(directory.objects.get(&object_id).is_none());
    }

    #[test]
    fn test_get_objects_for_page() {
        let mut directory = Directory::new(None, None);
        let page_id = PageId::Int32(1);
        let object_id1 = ObjectId::Int32(1);
        let object_id2 = ObjectId::Int32(2);
        let data_type = DataType::Int32(42);
        directory.add_page(page_id.clone(), data_type);
        directory.add_object(object_id1.clone(), page_id.clone());
        directory.add_object(object_id2.clone(), page_id.clone());
        let objects = directory.get_objects_for_page(page_id);
        assert_eq!(objects.len(), 2);
        assert!(objects.contains(&object_id1));
        assert!(objects.contains(&object_id2));
    }

    #[test]
    fn test_remove_page() {
        let mut directory = Directory::new(None, None);
        let page_id = PageId::Int32(1);
        let data_type = DataType::Int32(42);
        directory.add_page(page_id.clone(), data_type);
        directory.remove_page(page_id.clone());
        assert!(directory.get_page(page_id).is_none());
    
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut directory = Directory::new(None, None);
        let page_id = PageId::Int32(1);
        let object_id = ObjectId::Int32(1);
        let data_type = DataType::Int32(42);
        directory.add_page(page_id.clone(), data_type);
        directory.add_object(object_id, page_id);

        let serialized = directory.serialize();
        let mut offset = 0;
        let deserialized_directory = Directory::deserialize(&serialized, &mut offset);

        assert_eq!(directory.pages, deserialized_directory.pages);
        assert_eq!(directory.objects, deserialized_directory.objects);
    }

    #[test]
    fn save_to_disk(){
        let mut directory = Directory::new(None, None);
        let page_id = PageId::Int32(1);
        let object_id = ObjectId::Int32(1);
        let data_type = DataType::Int32(42);
        directory.add_page(page_id.clone(), data_type);
        directory.add_object(object_id, page_id);
        directory.set_file("data/directory.db");
        directory.file.write_all(&directory.serialize()).unwrap();

    }

    #[test]
    fn load_from_disk(){
        let mut directory = Directory::new(None, None);
        let page_id = PageId::Int32(1);
        let object_id = ObjectId::Int32(1);
        let data_type = DataType::Int32(42);
        directory.add_page(page_id.clone(), data_type);
        directory.add_object(object_id, page_id);
        directory.set_file("data/directory.db");
        let deserialized = Directory::deserialize(&directory.file.read_to_end().unwrap(), &mut 0);
        assert_eq!(directory.pages, deserialized.pages);
        assert_eq!(directory.objects, deserialized.objects);
    }
}
