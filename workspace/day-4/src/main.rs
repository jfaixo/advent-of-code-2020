use std::env;
use std::process::exit;
use regex::Regex;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let entries = parse_text_file(args[1].clone());

    eprintln!("{:?}", entries);

    println!("part 1: {}", valid_number_count_part_1(&entries));
    println!("part 2: {}", valid_number_count_part_2(&entries));
}

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>
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

    fn is_birth_year_valid(&self) -> bool {
        match &self.birth_year {
            None => false,
            Some(str) => {
                let year = str.parse::<u32>().expect("Cannot parse birth year");
                1920 <= year && year <= 2002
            }
        }
    }

    fn is_issue_year_valid(&self) -> bool {
        match &self.issue_year {
            None => false,
            Some(str) => {
                let year = str.parse::<u32>().expect("Cannot parse issue year");
                2010 <= year && year <= 2020
            }
        }
    }

    fn is_expiration_year_valid(&self) -> bool {
        match &self.expiration_year {
            None => false,
            Some(str) => {
                let year = str.parse::<u32>().expect("Cannot parse issue year");
                2020 <= year && year <= 2030
            }
        }
    }

    fn is_height_valid(&self) -> bool {
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
    fn is_hair_color_valid(&self) -> bool {
        match &self.hair_color {
            None => false,
            Some(str) => {
                let regex = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
                regex.is_match(str)
            }
        }
    }

    fn is_eye_color_valid(&self) -> bool {
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

    fn is_passport_id_valid(&self) -> bool {
        match &self.passport_id {
            None => false,
            Some(str) => {
                let regex = Regex::new(r"^\d{9}$").unwrap();
                regex.is_match(str)
            }
        }
    }
}

fn parse_text_file(file_name: String) -> Vec<Passport> {
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

fn valid_number_count_part_1(passports: &Vec<Passport>) -> u32 {
    let mut valid_passport_count : u32 = 0;

    for passport in passports {
        if passport.is_valid_part_1() {
            valid_passport_count += 1;
        }
    }

    valid_passport_count
}

fn valid_number_count_part_2(passports: &Vec<Passport>) -> u32 {
    let mut valid_passport_count : u32 = 0;

    for passport in passports {
        if passport.is_valid_part_2() {
            valid_passport_count += 1;
        }
    }

    valid_passport_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn password_birth_year() {
        let mut passport: Passport = Default::default();

        // Empty case
        assert_eq!(passport.is_birth_year_valid(), false);

        // Valid cases
        passport.birth_year = Some(String::from("1920"));
        assert_eq!(passport.is_birth_year_valid(), true);
        passport.birth_year = Some(String::from("1950"));
        assert_eq!(passport.is_birth_year_valid(), true);
        passport.birth_year = Some(String::from("2002"));
        assert_eq!(passport.is_birth_year_valid(), true);

        // Invalid cases
        passport.birth_year = Some(String::from("1919"));
        assert_eq!(passport.is_birth_year_valid(), false);
        passport.birth_year = Some(String::from("2021"));
        assert_eq!(passport.is_birth_year_valid(), false);
    }

    #[test]
    fn password_issue_year() {
        let mut passport: Passport = Default::default();

        // Empty case
        assert_eq!(passport.is_issue_year_valid(), false);

        // Valid cases
        passport.issue_year = Some(String::from("2010"));
        assert_eq!(passport.is_issue_year_valid(), true);
        passport.issue_year = Some(String::from("2015"));
        assert_eq!(passport.is_issue_year_valid(), true);
        passport.issue_year = Some(String::from("2020"));
        assert_eq!(passport.is_issue_year_valid(), true);

        // Invalid cases
        passport.issue_year = Some(String::from("2009"));
        assert_eq!(passport.is_issue_year_valid(), false);
        passport.issue_year = Some(String::from("2021"));
        assert_eq!(passport.is_issue_year_valid(), false);
    }

    #[test]
    fn password_expiration_year() {
        let mut passport: Passport = Default::default();

        // Empty case
        assert_eq!(passport.is_expiration_year_valid(), false);

        // Valid cases
        passport.expiration_year = Some(String::from("2020"));
        assert_eq!(passport.is_expiration_year_valid(), true);
        passport.expiration_year = Some(String::from("2025"));
        assert_eq!(passport.is_expiration_year_valid(), true);
        passport.expiration_year = Some(String::from("2030"));
        assert_eq!(passport.is_expiration_year_valid(), true);

        // Invalid cases
        passport.expiration_year = Some(String::from("2019"));
        assert_eq!(passport.is_expiration_year_valid(), false);
        passport.expiration_year = Some(String::from("2031"));
        assert_eq!(passport.is_expiration_year_valid(), false);
    }

    #[test]
    fn password_height() {
        let mut passport: Passport = Default::default();

        // Empty case
        assert_eq!(passport.is_height_valid(), false);

        // Valid cases
        passport.height = Some(String::from("150cm"));
        assert_eq!(passport.is_height_valid(), true);
        passport.height = Some(String::from("193cm"));
        assert_eq!(passport.is_height_valid(), true);
        passport.height = Some(String::from("59in"));
        assert_eq!(passport.is_height_valid(), true);
        passport.height = Some(String::from("76in"));
        assert_eq!(passport.is_height_valid(), true);

        // Invalid cases
        passport.height = Some(String::from("149cm"));
        assert_eq!(passport.is_height_valid(), false);
        passport.height = Some(String::from("194cm"));
        assert_eq!(passport.is_height_valid(), false);
        passport.height = Some(String::from("58in"));
        assert_eq!(passport.is_height_valid(), false);
        passport.height = Some(String::from("77in"));
        assert_eq!(passport.is_height_valid(), false);
    }

    #[test]
    fn password_hair_color() {
        let mut passport: Passport = Default::default();

        // Empty case
        assert_eq!(passport.is_hair_color_valid(), false);

        // Valid cases
        passport.hair_color = Some(String::from("#000000"));
        assert_eq!(passport.is_hair_color_valid(), true);
        passport.hair_color = Some(String::from("#aaaaaa"));
        assert_eq!(passport.is_hair_color_valid(), true);
        passport.hair_color = Some(String::from("#ffffff"));
        assert_eq!(passport.is_hair_color_valid(), true);
        passport.hair_color = Some(String::from("#999999"));
        assert_eq!(passport.is_hair_color_valid(), true);

        // Invalid cases
        passport.hair_color = Some(String::from("000000"));
        assert_eq!(passport.is_hair_color_valid(), false);
        passport.hair_color = Some(String::from("#00000"));
        assert_eq!(passport.is_hair_color_valid(), false);
        passport.hair_color = Some(String::from("#00000q"));
        assert_eq!(passport.is_hair_color_valid(), false);
    }

    #[test]
    fn password_eye_color() {
        let mut passport: Passport = Default::default();

        // Empty case
        assert_eq!(passport.is_eye_color_valid(), false);

        // Valid cases
        passport.eye_color = Some(String::from("amb"));
        assert_eq!(passport.is_eye_color_valid(), true);
        passport.eye_color = Some(String::from("blu"));
        assert_eq!(passport.is_eye_color_valid(), true);
        passport.eye_color = Some(String::from("brn"));
        assert_eq!(passport.is_eye_color_valid(), true);
        passport.eye_color = Some(String::from("gry"));
        assert_eq!(passport.is_eye_color_valid(), true);
        passport.eye_color = Some(String::from("grn"));
        assert_eq!(passport.is_eye_color_valid(), true);
        passport.eye_color = Some(String::from("hzl"));
        assert_eq!(passport.is_eye_color_valid(), true);
        passport.eye_color = Some(String::from("oth"));
        assert_eq!(passport.is_eye_color_valid(), true);

        // Invalid cases
        passport.eye_color = Some(String::from("42"));
        assert_eq!(passport.is_eye_color_valid(), false);
    }

    #[test]
    fn password_passport_id() {
        let mut passport: Passport = Default::default();

        // Empty case
        assert_eq!(passport.is_passport_id_valid(), false);

        // Valid cases
        passport.passport_id = Some(String::from("000000000"));
        assert_eq!(passport.is_passport_id_valid(), true);
        passport.passport_id = Some(String::from("999999999"));
        assert_eq!(passport.is_passport_id_valid(), true);

        // Invalid cases
        passport.passport_id = Some(String::from("00000000"));
        assert_eq!(passport.is_passport_id_valid(), false);
        passport.passport_id = Some(String::from("0000000000"));
        assert_eq!(passport.is_passport_id_valid(), false);
        passport.passport_id = Some(String::from("00000000A"));
        assert_eq!(passport.is_passport_id_valid(), false);

    }
}
