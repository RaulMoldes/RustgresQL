use std::cmp::Ordering;

// An entry of a node.
// It contains a key and a value.
// The key is used to sort the entries in the node.
// The value can be any type, so it is generic.
#[derive(Debug, Clone)]
pub struct Entry<T> {
    key: i32,
    value: T,
}

impl<T> Entry<T> {
    pub fn new(key: i32, value: T) -> Self {
        Entry { key, value }
    }
}

// Implement the PartialEq trait for the Entry struct.
impl<T> PartialEq for Entry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

// Implement the PartialOrd trait for the Entry struct.
impl<T> PartialOrd for Entry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.key.cmp(&other.key))
    }
}

// A node of a B-Tree.
// It contains a list of entries and a list of children.
#[derive(Debug, Clone)]
struct BTreeNode<T>
where
    T: Clone,
{
    entries: Vec<Entry<T>>,
    children: Vec<BTreeNode<T>>,
    is_leaf: bool,
    is_root: bool,
}

impl<T: std::clone::Clone> BTreeNode<T> {
    pub fn new(
        entries: Option<Vec<Entry<T>>>,
        children: Option<Vec<BTreeNode<T>>>,
        is_leaf: bool,
        is_root: bool,
    ) -> Self {
        BTreeNode {
            entries: entries.unwrap_or(Vec::new()),
            children: children.unwrap_or(Vec::new()),
            is_leaf,
            is_root,
        }
    }

    pub fn is_full(&self, degree: i32) -> bool {
        self.entries.len() == (2 * degree - 1) as usize
    }

    pub fn is_underflow(&self, degree: i32) -> bool {
        self.entries.len() < degree as usize - 1
    }

    pub fn get_predecessor(&self, key: i32) -> &Entry<T> {
        // From the key, find previous entry in the node
        let mut i = 0;
        while i < self.entries.len() && key < self.entries[i].key {
            i += 1;
        }
        &self.entries[i - 1]
    }

    pub fn get_successor(&self, key: i32) -> &Entry<T> {
        // From the key, find the key of the predecessor child

        let mut i = 0;
        while i < self.entries.len() && key < self.entries[i].key {
            i += 1;
        }
        &self.entries[i]
    }

    pub fn len(&self) -> i32 {
        self.entries.len() as i32
    }
}

#[derive(Debug)]
struct BTree<T: std::clone::Clone> {
    root: BTreeNode<T>,
    degree: i32,
}

impl<T: std::clone::Clone> BTree<T> {
    pub fn new(root: Option<BTreeNode<T>>, degree: i32) -> Self {
        BTree {
            root: root.unwrap_or(BTreeNode::new(None, None, true, true)),
            degree,
        }
    }

    pub fn search<'a>(&'a self, u: &'a BTreeNode<T>, key: i32) -> Option<&'a Entry<T>> {
        // Linear search for the key in the node
        let mut i = 0;
        while i < u.entries.len() && key > u.entries[i].key {
            i += 1;
        }

        // If the key is found, return the entry
        if i < u.entries.len() && key == u.entries[i].key {
            return Some(&u.entries[i]);
        }

        // If the node is a leaf, the key is not in the tree
        if u.is_leaf {
            return None;
        }

        // Recursively search for the key in the child node
        self.search(&u.children[i], key)
    }

    // Inserts a new entry into the B-Tree on a non-full node
    // FIX-THEN-PROCEED strategy
    pub fn insert_non_full(&self, u: &mut BTreeNode<T>, key: i32, value: T) {
        let mut i = 0;
        while i < u.entries.len() && key > u.entries[i].key {
            i += 1;
        }

        if u.is_leaf {
            let entry = Entry::new(key, value);
            u.entries.insert(i, entry);
            // self.write_to_disk(&u);
        } else {
            if u.children[i].is_full(self.degree) {
                self.split_child(u, i);
                if key > u.entries[i].key {
                    i += 1;
                }
                self.insert_non_full(&mut u.children[i], key, value);
            }
        }
    }

    // Helper function to split the root node when full
    // Creates a new root node with the old root as its child
    // This is the only case where the height of the tree increases
    pub fn split_root(&self) -> BTreeNode<T> {
        let root = self.root.clone();
        let t = self.degree;
        let mut new = BTreeNode::new(None, None, false, false);
        new.children.push(root);
        self.split_child(&mut new, 0);
        new
    }

    pub fn split_child(&self, u: &mut BTreeNode<T>, i: usize) {
        let mut z = u.children[i].clone();
        let t = self.degree;

        let mut new = BTreeNode::new(None, None, z.is_leaf, false);
        new.entries.extend_from_slice(&z.entries[t as usize..]);

        if !z.is_leaf {
            new.children = z.children.split_off(t as usize);
        }
        u.children.insert(i + 1, new);
        u.entries.insert(i, z.entries[t as usize - 1].clone()); // median entry
        z.entries.truncate(t as usize - 1);
        z.children.truncate(t as usize);
        // self.write_to_disk(&z);
        // self.write_to_disk(&new);
        // self.write_to_disk(&u);
    }

    pub fn merge_children(&self, u: &mut BTreeNode<T>, i: usize) {
        // Merge the i-th child of the node u with its i+1-th sibling

        let t = self.degree;
        let median_entry = u.entries.remove(i);
        u.children[i].entries.push(median_entry);
        let (left, right) = u.children.split_at_mut(i + 1);
        left[i].entries.extend_from_slice(&right[0].entries);
        if !u.children[i].is_leaf {
            let (left, right) = u.children.split_at_mut(i + 1);
            left[i].children.extend_from_slice(&right[0].children);
        }
        u.children.remove(i + 1);
    }

    // Deletes a key from the B-Tree
    //FIX-THEN-PROCEED strategy
    // 3 cases:
    // 1. The key is in the node u and is a leaf
    // 2. The key is in the node u and is an internal node
    // 3. The key is not in the node u
    pub fn delete(&mut self, u: &mut BTreeNode<T>, key: i32) {
        // Assumption: u has at least t keys or is the root
        let t = self.degree;
        let mut i = 0;
        while i < u.entries.len() && key > u.entries[i].key {
            i += 1;
        }
        // Case 1: The key is in the node u and is a leaf
        if u.is_leaf {
            if i < u.entries.len() && key == u.entries[i].key {
                u.entries.remove(i); // Remove the key
                                     // self.write_to_disk(&u);
            } else {
                panic!("Key not found in the B-Tree");
            }
            return;
        }
        // u is an internal node
        // Case 2: The key is in the node u and is an internal node
        if i < u.entries.len() && key == u.entries[i].key {
            // Case 2a: The predecessor child has at least t keys
            if u.children[i].entries.len() >= t as usize {
                // Find the predecessor entry
                let mut child = &u.children[i];
                let pred_entry = child.get_predecessor(key).clone();
                let pred_key = pred_entry.key;
                // Call delete on the predecessor child
                self.delete(&mut u.children[i], pred_key);
                u.entries[i] = pred_entry;
            }
            // Case 2b: The successor child has at least t keys
            else if u.children[i + 1].entries.len() >= t as usize {
                // Find the successor entry
                let mut child = &u.children[i + 1];
                let succ_entry = child.get_successor(key).clone();
                let succ_key = succ_entry.key;
                // Call delete on the successor child
                self.delete(&mut u.children[i + 1], succ_key);
                u.entries[i] = succ_entry;
            } else {
                // Case 2c: Both predecessor and successor children have t-1 keys
                // Merge the key and the successor child into the predecessor child
                //TODO: Implement this case
                self.merge_children(u, i);
                if u.is_root && u.entries.len() == 0 {
                    self.root = u.children.remove(0);
                }
                self.delete(&mut u.children[i], key);
                // self.write_to_disk(&u);
            }
        }
        // Case 3: U is an internal node but the key is not in the node
        else {
            // Case 3a: The child that precedes key has t keys
            if u.children[i].entries.len() >= t as usize {
                self.delete(&mut u.children[i], key); // Recursively delete the key
            }
            // Case 3b - 1: The child that follows key has t keys
            else if u.children[i + 1].entries.len() >= t as usize {
                u.children[i].entries.push(u.entries[i].clone());
                u.entries[i] = u.children[i + 1].entries.remove(0);
                if !u.children[i + 1].is_leaf {
                    let (left, right) = u.children.split_at_mut(i + 1);
                    left[i].children.push(right[0].children.remove(0));
                }
                self.delete(&mut u.children[i], key); // Recursively delete the key
            }
            // Case 3b - 2: The left sibilng has t keys
            else if u.children[i - 1].entries.len() >= t as usize {
                u.children[i].entries.push(u.entries[i].clone());
                u.entries[i] = u.children[i - 1].entries.remove(0);
                if !u.children[i - 1].is_leaf {
                        let (left, right) = u.children.split_at_mut(i);
                        left[i - 1].children.push(right[0].children.remove(0));
                }
                self.delete(&mut u.children[i], key); // Recursively delete the key
            } else {
                // Case 3c: Both the child that precedes and follows key have t-1 keys
                // Merge the key and the successor child into the predecessor child
                if i > 0 {
                    self.merge_children(u, i - 1);
                    i -= 1;
                } else {
                    self.merge_children(u, i);
                }
                if u.is_root && u.entries.len() == 0 {
                    self.root = u.children.remove(0);
                }
                self.delete(&mut u.children[i], key);
            }
        }
    }
}

fn main() {
    let entry = Entry::new(1, "Hello");
    println!("Key: {}, Value: {}", entry.key, entry.value);

    let entry2 = Entry::new(2, "World");
    println!("Key: {}, Value: {}", entry2.key, entry2.value);

    let entry3 = Entry::new(3, "Hello");
    println!("Key: {}, Value: {}", entry3.key, entry3.value);

    // Comparar dos entradas
    println!("Comparar dos entradas: {:?}", &entry < &entry2);
    println!("Comparar dos entradas: {:?}", &entry < &entry3);

    let btree_node = BTreeNode {
        entries: vec![entry, entry2, entry3],
        children: vec![],
        is_leaf: true,
        is_root: true,
    };
    println!("{:?}", btree_node);

    let mut  btree = BTree {
        root: btree_node,
        degree: 3,
    };
    println!("{:?}", btree);
    let mut root = btree.root.clone();

    let found = btree.search(&root, 2);

    match found {
        Some(entry) => println!("Found: {:?}", entry),
        None => println!("Not found"),
    }

    let entry4 = Entry::new(4, "Hola");
    btree.insert_non_full(&mut root, 4, "Hola");
    println!("{:?}", btree);
    let found = btree.search(&root, 4);

    match found {
        Some(entry) => println!("Found: {:?}", entry),
        None => println!("Not found"),
    }

    btree.insert_non_full(&mut root, 12, "Hola");
    println!("{:?}", btree);
    let found = btree.search(&root, 12);

    match found {
        Some(entry) => println!("Found: {:?}", entry),
        None => println!("Not found"),
    }

    btree.insert_non_full(&mut root, 13, "Hola");
    println!("{:?}", btree);
    let found = btree.search(&root, 13);

    match found {
        Some(entry) => println!("Found: {:?}", entry),
        None => println!("Not found"),
    }
    btree.insert_non_full(&mut root, 12, "Hola");
    println!("{:?}", btree);
    let found = btree.search(&root, 12);

    match found {
        Some(entry) => println!("Found: {:?}", entry),
        None => println!("Not found"),
    }

    btree.insert_non_full(&mut root, 5, "Hola");
    println!("{:?}", btree);
    let found = btree.search(&root, 5);

    match found {
        Some(entry) => println!("Found: {:?}", entry),
        None => println!("Not found"),
    }

    btree.insert_non_full(&mut root, 6, "Hola");
    println!("{:?}", btree);
    let found = btree.search(&root, 6);

    match found {
        Some(entry) => println!("Found: {:?}", entry),
        None => println!("Not found"),
    }

    btree.delete(&mut root, 6);
    println!("{:?}", btree);
    let found = btree.search(&root, 6);

    match found {
        Some(entry) => println!("Found: {:?}", entry),
        None => println!("Not found"),
    }
}
