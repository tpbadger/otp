use phf::phf_map;
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use structopt::StructOpt;

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
    ' ' => 26,
    '.' => 27,
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
   "26" => ' ',
   "27" => '.',
};

#[derive(StructOpt)]
struct Cli {
    /// command to run -> [GenerateKey, Encode, Decode]
    command: String,
    /// path to key file if encoding/decoding
    #[structopt(short = "k", long = "key")]
    key_file: Option<String>,
    /// path to message file if encoding/decoding
    #[structopt(short = "m", long = "message")]
    message_file: Option<String>,
    /// path to message file if encoding/decoding
    #[structopt(short = "l", long = "length")]
    key_length: Option<u64>,
}
 
fn generate_key(message_length: u64) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    let mut rng = rand::thread_rng();
    // generate key based on message length
    for _ in 1..=message_length {
        let random_num = rng.gen_range(0..28).to_string();
        let i = NUM_CHAR_MAP[&random_num[..]];
        v.push(i);
    }
    return v;
}

fn write_file(filename: String, content: Vec<char>) {
    let path = Path::new(&filename);
    let display = path.display();

    // create the file
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // collect the content vec to write to file
    let collected: String = content.into_iter().collect();

    // write to the file
    std::fs::write(filename, collected).expect("Unable to write to file");
}

fn read_file(filename: String) -> Result<Vec<char>, Error> {
    // reads a file (key or message) into char vec for consumption
    let file = File::open(filename)?;
    let br = BufReader::new(file);
    let mut content = Vec::new();
    for line in br.lines() {
        let line = line?;
        let n = line.trim();
        for c in n.chars() {
            content.push(c);
        }
    }
    Ok(content) // everything is Ok, return vector
}

#[test]
fn test_encode() {
    // assemble
    let message = vec!['h', 'e', 'l', 'l' ,'o'];
    let key = vec!['x', 'm', 'c', 'k', 'l'];
    let expected = vec!['c', 'q', 'n', 'v', 'z'];
    // act
    let res = process_message(&message, &key, true);
    // assert
    assert_eq!(res, expected);
}

#[test]
fn test_decode() {
    // assemble
    let message = vec!['c', 'q', 'n', 'v' ,'z'];
    let key = vec!['x', 'm', 'c', 'k', 'l'];
    let expected = vec!['h', 'e', 'l', 'l', 'o'];
    // act
    let res = process_message(&message, &key, false);
    // assert
    assert_eq!(res, expected);
}

fn process_message(message: &Vec<char>, key: &Vec<char>, encode: bool) -> Vec<char>{
    let mut processed: Vec<char> = Vec::new();
    for (index, c) in message.into_iter().enumerate() {
        // get message number value
        let message_num = CHAR_NUM_MAP[&c];
        // get key number value
        let key_char = key[index];
        let key_num = CHAR_NUM_MAP[&key_char];
        let mut mod_number: i64;
        if encode {
            mod_number = message_num + key_num;
            if mod_number > 27 {
                mod_number -= 28
            }
        } else {
            mod_number = message_num - key_num;
            if mod_number < 0 {
                mod_number += 28
            }
        }
        let decode_char = NUM_CHAR_MAP[&mod_number.to_string()[..]];
        processed.push(decode_char);
    }
    return processed;
}

fn cli_generate_key(message_length: u64) {
    let k = generate_key(message_length);
    write_file("key.txt".to_string(), k);
}

fn cli_encode(message_file_path: String, key_file_path: String) {
    let m = read_file(message_file_path);
    let k = read_file(key_file_path);

    match m {
        Ok(m) => {
            match k {
                Ok(k) => {
                    let e = process_message(&m, &k, true);
                    write_file("encoded.txt".to_string(), e);
                },
                Err(k) => panic!("Couldn't read from key file")
            }
        },
        Err(m) => panic!("Couldn't read from message file")
    }
}

fn cli_decode(message_file_path: String, key_file_path: String) {
    let m = read_file(message_file_path);
    let k = read_file(key_file_path);

    match m {
        Ok(m) => {
            match k {
                Ok(k) => {
                    let e = process_message(&m, &k, false);
                    write_file("decoded.txt".to_string(), e);
                },
                Err(k) => panic!("Couldn't read from key file")
            }
        },
        Err(m) => panic!("Couldn't read from message file")
    }
}

fn main() {   
    let args = Cli::from_args();

    match &args.command[..] {
        "GenerateKey" => {
            if let Some(length) = args.key_length {
                cli_generate_key(length);
            } else {
                println!("No length provided")
            }
        },
        "Encode" => {
            if let Some(key_file) = args.key_file {
                if let Some(message_file) = args.message_file {
                    cli_encode(message_file, key_file);
                }
                else {
                    // message file not provided
                    println!("path to message file not provided")
                }
            } else {
                // key file not provided
                println!("path to key file not provided")
            }
        },
        "Decode" => {
            if let Some(key_file) = args.key_file {
                if let Some(message_file) = args.message_file {
                    cli_decode(message_file, key_file);
                }
                else {
                    // message file not provided
                    println!("path to message file not provided")
                }
            } else {
                // key file not provided
                println!("path to key file not provided")
            }
        },
        _ => {println!("{}", args.command + " not valid!")}
    }
}
