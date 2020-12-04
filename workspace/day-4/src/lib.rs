use regex::Regex;
use std::fs;

#[derive(Debug, Default)]
pub struct Passport {
    pub birth_year: Option<String>,
    pub issue_year: Option<String>,
    pub expiration_year: Option<String>,
    pub height: Option<String>,
    pub hair_color: Option<String>,
    pub eye_color: Option<String>,
    pub passport_id: Option<String>,
    pub country_id: Option<String>
}

impl Passport {
    fn is_valid_part_1(&self) -> bool {
        self.birth_year.is_some() &&
            self.issue_year.is_some() &&
            self.expiration_year.is_some() &&
            self.height.is_some() &&
            self.hair_color.is_some() &&
            self.eye_color.is_some() &&
            self.passport_id.is_some()
    }

    fn is_valid_part_2(&self) -> bool {
        self.is_birth_year_valid() &&
            self.is_issue_year_valid() &&
            self.is_expiration_year_valid() &&
            self.is_height_valid() &&
            self.is_hair_color_valid() &&
            self.is_eye_color_valid() &&
            self.is_passport_id_valid()
    }

    pub fn is_birth_year_valid(&self) -> bool {
        match &self.birth_year {
            None => false,
            Some(str) => {
                let year = str.parse::<u32>().expect("Cannot parse birth year");
                1920 <= year && year <= 2002
            }
        }
    }

    pub fn is_issue_year_valid(&self) -> bool {
        match &self.issue_year {
            None => false,
            Some(str) => {
                let year = str.parse::<u32>().expect("Cannot parse issue year");
                2010 <= year && year <= 2020
            }
        }
    }

    pub fn is_expiration_year_valid(&self) -> bool {
        match &self.expiration_year {
            None => false,
            Some(str) => {
                let year = str.parse::<u32>().expect("Cannot parse issue year");
                2020 <= year && year <= 2030
            }
        }
    }

    pub fn is_height_valid(&self) -> bool {
        match &self.height {
            None => false,
            Some(str) => {
                if str.ends_with("cm") {
                    // cm
                    let value = str.strip_suffix("cm").unwrap_or("error").parse::<u32>().unwrap_or(0);
                    150 <= value && value <= 193
                }
                else {
                    // in
                    let value = str.strip_suffix("in").unwrap_or("error").parse::<u32>().unwrap_or(0);
                    59 <= value && value <= 76
                }
            }
        }
    }

    //noinspection RegExpRedundantNestedCharacterClass
    //noinspection RegExpDuplicateCharacterInClass
    pub fn is_hair_color_valid(&self) -> bool {
        match &self.hair_color {
            None => false,
            Some(str) => {
                let regex = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
                regex.is_match(str)
            }
        }
    }

    pub fn is_eye_color_valid(&self) -> bool {
        match &self.eye_color {
            None => false,
            Some(str) => {
                match str.as_str() {
                    "amb" => true,
                    "blu" => true,
                    "brn" => true,
                    "gry" => true,
                    "grn" => true,
                    "hzl" => true,
                    "oth" => true,
                    _ => false
                }
            },
        }
    }

    pub fn is_passport_id_valid(&self) -> bool {
        match &self.passport_id {
            None => false,
            Some(str) => {
                let regex = Regex::new(r"^\d{9}$").unwrap();
                regex.is_match(str)
            }
        }
    }
}

pub fn parse_text_file(file_name: String) -> Vec<Passport> {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    return file_content
        // Split between passports
        .split("\n\n")
        .map(|passport_string| {
            // For each passport
            // Create the passport with default empty fields
            let mut passport : Passport = Default::default();

            // Process passport fields
            passport_string.split_ascii_whitespace()
                .for_each(|field| {
                    let field_key_value : Vec<String> = field.split(":")
                        .map(|s| String::from(s))
                        .collect();
                    match field_key_value[0].as_str() {
                        "byr" => passport.birth_year = Some(field_key_value[1].clone()),
                        "iyr" => passport.issue_year = Some(field_key_value[1].clone()),
                        "eyr" => passport.expiration_year = Some(field_key_value[1].clone()),
                        "hgt" => passport.height = Some(field_key_value[1].clone()),
                        "hcl" => passport.hair_color = Some(field_key_value[1].clone()),
                        "ecl" => passport.eye_color = Some(field_key_value[1].clone()),
                        "pid" => passport.passport_id = Some(field_key_value[1].clone()),
                        "cid" => passport.country_id = Some(field_key_value[1].clone()),
                        _ => {}
                    }
                });
            passport
        })
        .collect()
}

pub fn valid_number_count_part_1(passports: &Vec<Passport>) -> u32 {
    let mut valid_passport_count : u32 = 0;

    for passport in passports {
        if passport.is_valid_part_1() {
            valid_passport_count += 1;
        }
    }

    valid_passport_count
}

pub fn valid_number_count_part_2(passports: &Vec<Passport>) -> u32 {
    let mut valid_passport_count : u32 = 0;

    for passport in passports {
        if passport.is_valid_part_2() {
            valid_passport_count += 1;
        }
    }

    valid_passport_count
}