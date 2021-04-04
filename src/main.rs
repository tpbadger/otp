use phf::phf_map;
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

// CLI functionality
// generate a key based on message length -> Done
// encrypt a message with a provided key -> Done
// decrypt a message with a provided key -> Done
// file io 
// cli flags

static CHAR_NUM_MAP: phf::Map<char, i64> = phf_map! {
    'a' => 0,
    'b' => 1,
    'c' => 2,
    'd' => 3,
    'e' => 4,
    'f' => 5,
    'g' => 6,
    'h' => 7,
    'i' => 8,
    'j' => 9,
    'k' => 10,
    'l' => 11,
    'm' => 12,
    'n' => 13,
    'o' => 14,
    'p' => 15,
    'q' => 16,
    'r' => 17,
    's' => 18,
    't' => 19,
    'u' => 20,
    'v' => 21,
    'w' => 22,
    'x' => 23,
    'y' => 24,
    'z' => 25,
};

static NUM_CHAR_MAP: phf::Map<&str, char> = phf_map! {
   "0" => 'a',
   "1" => 'b',
   "2" => 'c',
   "3" => 'd',
   "4" => 'e',
   "5" => 'f',
   "6" => 'g',
   "7" => 'h',
   "8" => 'i',
   "9" => 'j',
   "10" => 'k',
   "11" => 'l',
   "12" => 'm',
   "13" => 'n',
   "14" => 'o',
   "15" => 'p',
   "16" => 'q',
   "17" => 'r',
   "18" => 's',
   "19" => 't',
   "20" => 'u',
   "21" => 'v',
   "22" => 'w',
   "23" => 'x',
   "24" => 'y',
   "25" => 'z',
};

fn generate_key(message_length: usize) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    let mut rng = rand::thread_rng();
    // generate key based on message length
    for _ in 1..=message_length {
        let random_num = rng.gen_range(0..26).to_string();
        let i = NUM_CHAR_MAP[&random_num[..]];
        v.push(i);
    }
    return v;
}

fn write_file(filename: String, content: Vec<char>) {
    let mut f = File::create(filename).expect("Unable to create key");
    // collect content into a String literal
    let s: String = content.into_iter().collect();
    // write to the file
    writeln!(f, "{}", s);
}

#[test]
fn test_encode() {
    // assemble
    let message = "hello".to_string();
    let key = vec!['x', 'm', 'c', 'k', 'l'];
    let expected = vec!['e', 'q', 'n', 'v', 'z'];
    // act
    let res = process_message(&message, &key, true);
    // assert
    assert_eq!(res, expected);
}

#[test]
fn test_decode() {
    // assemble
    let message = "eqnvz".to_string();
    let key = vec!['x', 'm', 'c', 'k', 'l'];
    let expected = vec!['h', 'e', 'l', 'l', 'o'];
    // act
    let res = process_message(&message, &key, false);
    // assert
    assert_eq!(res, expected);
}

fn process_message(message: &String, key: &Vec<char>, encode: bool) -> Vec<char>{
    let mut processed: Vec<char> = Vec::new();
    for (index, c) in message.chars().enumerate() {
        // ignore whitespace
        let c_string = c.to_string();
        if c_string != " " {
            // get message number value
            let message_num = CHAR_NUM_MAP[&c];
            // get key number value
            let key_char = key[index];
            let key_num = CHAR_NUM_MAP[&key_char];
            let mut mod_number: i64;
            if encode {
                mod_number = message_num + key_num;
                if mod_number > 26 {
                    mod_number -= 26
                }
            } else {
                mod_number = message_num - key_num;
                if mod_number < 0 {
                    mod_number += 26
                }
            }
            let decode_char = NUM_CHAR_MAP[&mod_number.to_string()[..]];
            processed.push(decode_char);
        }
    }
    return processed;
}

fn main() {
    // cli here
}
