use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use regex::Regex;

pub fn problem1(filename: &str) {

    let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let  root = parse_and_construct_tree(file_contents);

    let (matched_size, _) = calculate_size(Rc::clone(&root), 0);
    println!("Matched size is: {}", matched_size);
}

pub fn problem2(filename: &str) {

    let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let  root = parse_and_construct_tree(file_contents);
    let (_, root_size) = calculate_size(Rc::clone(&root), 0);

    {
        root.borrow_mut().total_size = root_size;
    }

    let total_disk_space_available = 70000000;
    let unused_space_required = 30000000;

    let borrowed_root = root.borrow();
    let free_space = total_disk_space_available - borrowed_root.total_size;
    println!("TS: {}, FS: {}", borrowed_root.total_size, free_space);
    let space_needed_to_free  = unused_space_required - free_space;

    let min_size_to_delete = get_directory_to_delete(Rc::clone(&root), space_needed_to_free, u32::MAX);

    println!("Min size is: {}", min_size_to_delete);
}

enum ItemType {
    File = 0,
    Dir = 1
}

struct Item {
    item_type: ItemType,
    parent: Option<Rc<RefCell<Item>>>,
    children: Vec<Rc<RefCell<Item>>>,
    name: String,
    total_size: u32,
}

fn parse_and_construct_tree(file_contents: String) -> Rc<RefCell<Item>> {
    let command_regex = Regex::new(r"\$ (cd|ls)\s?(.*)?").unwrap();
    let directory_regex = Regex::new(r"dir (.+)").unwrap();
    let file_regex = Regex::new(r"(\d+) (.+)").unwrap();

    let root = Rc::new(RefCell::new(Item {
        item_type: ItemType::Dir,
        parent: None,
        children: Vec::new(),
        name: String::from("/"),
        total_size: 0
    }));

    let mut _current = Rc::clone(&root);

    for line in file_contents.lines() {
        if let Some(captures) = command_regex.captures(line) {
            let command = captures.get(1).expect("Command should exist");
            let command_arg = captures.get(2).expect("Command argument should exist");

            match (command.as_str(), command_arg.as_str()) {
                ("cd", "/") => {
                    // Go to root directory
                    _current = Rc::clone(&root);
                }
                ("cd", "..") => {
                    let current_clone = Rc::clone(&_current);
                    _current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                }
                ("cd", directory_name) => {
                    // Find and move to that directory
                    let current_clone = Rc::clone(&_current);
                    for child in current_clone.borrow().children.iter() {
                        if child.borrow().name.eq(directory_name) {
                            _current = Rc::clone(&child);
                            break;
                        }
                    }
                },
                ("ls", _) => { /* No-op */},
                (_, _) => {
                    panic!("Unrecognized command");
                }
            }
        } else if let Some(captures) = directory_regex.captures(line) {
            // Add directory
            let directory_name = captures.get(1).expect("Expecting the directory name to be present");
            let new_directory = Rc::new(RefCell::new(Item {
                item_type: ItemType::Dir,
                parent: Some(Rc::clone(&_current)),
                children: Vec::new(),
                name: String::from(directory_name.as_str()),
                total_size: 0
            }));

            _current.borrow_mut().children.push(new_directory);

        } else if let Some(captures) = file_regex.captures(line) {
            // Add directory
            let file_size = captures.get(1).expect("Expecting the file size to be present").as_str().parse::<u32>().unwrap();
            let file_name = captures.get(2).expect("Expecting the file name to be present");
            let new_directory = Rc::new(RefCell::new(Item {
                item_type: ItemType::File,
                parent: Some(Rc::clone(&_current)),
                children: Vec::new(),
                name: String::from(file_name.as_str()),
                total_size: file_size
            }));

            _current.borrow_mut().children.push(new_directory);
        } else {
            panic!("No matching pattern found");
        }
    }

    return root;
}

fn calculate_size(root: Rc<RefCell<Item>>, current_matched_size: u32) -> (u32, u32) {
    let at_most_size = 100000u32;

    let mut _current_matched_size = current_matched_size;
    let mut _sub_size = 0u32;

    for child in root.borrow().children.iter() {
        let mut _directory_size = None;
        {
            let b_child = child.borrow();
            match b_child.item_type {
                ItemType::File => {
                    _sub_size += b_child.total_size;
                },
                ItemType::Dir => {
                    let (updated_current_matched_size, directory_size) = calculate_size(Rc::clone(child), _current_matched_size);
                    _current_matched_size = updated_current_matched_size;
                    _sub_size += directory_size;

                    if directory_size <= at_most_size {
                        _current_matched_size += directory_size;
                    }
                    //println!("{}", b_child.name);
                    _directory_size = Some(directory_size);
                }
            }
        }

        if let Some(total_size) = _directory_size {
            println!("Updating directory size: {}", total_size);
            child.borrow_mut().total_size = total_size;
        }
    }

    return (_current_matched_size, _sub_size);
}

fn get_directory_to_delete(root: Rc<RefCell<Item>>, required_directory_size: u32, current_min: u32) -> u32{

    let mut _min_size = current_min;
    for child in root.borrow().children.iter() {
        let b_child = child.borrow();
        match b_child.item_type {
            ItemType::Dir => {
                let child_min = get_directory_to_delete(Rc::clone(child), required_directory_size, current_min);
                _min_size = _min_size.min(child_min);
            },
            _ => continue
        }
    }

    let borrowed_root = root.borrow();
    if (borrowed_root.total_size >= required_directory_size) {
        _min_size = _min_size.min(borrowed_root.total_size);
    }

    return _min_size;
}