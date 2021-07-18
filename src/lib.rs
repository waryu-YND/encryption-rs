use rand::prelude::*;
use anyhow::{ensure,Context,Result};

const ENCRYPTION_CHARACTER: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[derive(Debug)]
pub struct EncryptionId {
    pub string: String,
    numbers: Vec<u8>
}

impl EncryptionId {
    pub fn new(id: Option<String>,id_length: usize) -> Result<EncryptionId> {

        let mut string = String::new();
        let mut numbers: Vec<u8> = Vec::new();

        match id {
            Some(string) => {
                
                let strings = string.chars();

                ensure!(&string.len() > &0,"Error: ID is 0 char.");
                let _encryption_character_chars = ENCRYPTION_CHARACTER.chars();
                ensure!(matches!(&strings,_encryption_character_chars),"Error: invalid ID: {}\nThe letters that can be used for ID are {}.",&string,ENCRYPTION_CHARACTER);

                for i in strings {
                    numbers.push(ENCRYPTION_CHARACTER.find(i).context("Unknown error")? as u8)
                }
                return Ok(EncryptionId {string,numbers})
            }
            None => {
                let mut rng = thread_rng();
                let characters = ENCRYPTION_CHARACTER.chars().collect::<Vec<char>>();
                
                for _i in 0..id_length {
                    let random = rng.gen_range(0..characters.len());
                    string.push(characters[random]);
                    numbers.push(random as u8);
                }

                return Ok(EncryptionId {string,numbers})
            }
        }
    }
}

pub struct Encryption {
    pub id: EncryptionId,
    pub contents: Vec<u8>
}

impl Encryption {
    pub fn new(contents: Vec<u8>,encryption_id: EncryptionId) -> Result<Encryption> {

        Ok(Encryption {id: encryption_id,contents})
    }

    pub fn encrypt(&mut self) {

        // Put a different integer in each digit.
        let mut add_value:u8 = 0;
        for i in 0..self.contents.len() {
            add_value = add_value.wrapping_add(self.id.numbers[0]);
            self.contents[i] = self.contents[i].wrapping_add((i as u8).wrapping_add(add_value));
        }

        // Process all digits using id.
        for i in self.id.numbers.iter() {
            for j in self.contents.iter_mut() {
                *j = match i{
                    i if i % 4 == 0 => j.wrapping_add(*i + 1),
                    i if i % 4 == 1 => j.wrapping_sub(*i + 1),
                    i if i % 4 == 2 => j.rotate_right((*i + 1) as u32),
                    i if i % 4 == 3 => j.rotate_left((*i + 1) as u32),
                    _ => unreachable!(),
                };
            }
        }
    }

    pub fn decrypt(&mut self) {

        // Process all digits using id.
        for i in self.id.numbers.iter().rev() {
            for j in self.contents.iter_mut() {
                *j = match i{
                    i if i % 4 == 0 => j.wrapping_sub(*i + 1),
                    i if i % 4 == 1 => j.wrapping_add(*i + 1),
                    i if i % 4 == 2 => j.rotate_left((*i + 1) as u32),
                    i if i % 4 == 3 => j.rotate_right((*i + 1) as u32),
                    _ => unreachable!(),
                };
            }
        }

        // Put a different integer in each digit.
        let mut add_value:u8 = 0;
        for i in 0..self.contents.len() {
            add_value = add_value.wrapping_sub(self.id.numbers[0]);
            self.contents[i] = self.contents[i].wrapping_sub((i as u8).wrapping_sub(add_value));
        }
    }
}