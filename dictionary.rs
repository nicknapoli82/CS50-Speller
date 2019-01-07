// Implements a dictionary's functionality

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ffi::CStr;
use std::os::raw::{c_char, c_uint};
use std::collections::HashSet;
use std::mem::{transmute, transmute_copy};

static mut dict_words: [u8; 40] = [0u8; 40];
static mut num_words: u32 = 0;

// Loads dictionary into memory, returning true if successful else false
#[no_mangle]
pub extern "C" fn load(dictionary: *const c_char) -> bool
{
    let d = unsafe { CStr::from_ptr(dictionary) };
    let in_file = File::open(d.to_str().unwrap()).unwrap();

    let mut dict: HashSet<String> = HashSet::new();

    for line in BufReader::new(in_file).lines() {
        let mut line = line.unwrap();
        dict.insert(line);
    }

    unsafe { num_words = dict.len() as u32 };
    
    unsafe {
        dict_words = transmute::<HashSet<String>, [u8; 40]>(dict);
    }

    return true;
}

// Returns number of words in dictionary if loaded else 0 if not yet loaded
#[no_mangle]
pub extern "C" fn size() -> c_uint
{
    return unsafe { num_words as c_uint };
}

// Returns true if word is in dictionary else false
#[no_mangle]
pub extern "C" fn check(w: *const c_char) -> bool
{
    let w = unsafe { CStr::from_ptr(w) };
    let word = w.to_str().unwrap().to_lowercase();

    let dict: HashSet<String> = unsafe { transmute_copy(&dict_words) };
    if dict.contains(&word) {
        std::mem::forget(dict);
        return true
    }
    else {
        std::mem::forget(dict);
        return false
    }
}

// Unloads dictionary from memory, returning true if successful else false
#[no_mangle]
pub extern "C" fn unload() -> bool
{
    // TODO
    // I don't think I need to do anything here?
    // But for complete code to make sure everything is cleaned up
    // lets drop dict_words
    let _dict: HashSet<String> = unsafe { transmute_copy(&dict_words) };
    return true;
}
