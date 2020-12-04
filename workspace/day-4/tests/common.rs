mod day_4 {
    use day_4::Passport;

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