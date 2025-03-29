mod sencitive_data;

use std::fmt::{Debug, Display};

use sencitive_data::SensitiveData;

#[derive(Debug)]
struct User {
    name: String,
    password: SensitiveData<String>,
    social_insurance_number: SensitiveData<u32>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name {}, password: {}, sin: {}",
            self.name, self.password, self.social_insurance_number
        )
    }
}

fn main() {
    let user = User {
        name: "Denys".into(),
        password: SensitiveData::from("THIS_is_PASSWORD".to_string()),
        social_insurance_number: SensitiveData::from(1234567890),
    };

    dbg!(user);
}

#[cfg(test)]
mod test {
    use crate::{SensitiveData, User};

    #[test]
    fn test() {
        let user = User {
            name: "Denys".into(),
            password: SensitiveData::from("THIS_is_PASSWORD".to_string()),
            social_insurance_number: SensitiveData::from(1234567890),
        };
        assert_eq!(
            user.password,
            SensitiveData::from("THIS_is_PASSWORD".to_string())
        );
        assert_eq!(
            user.social_insurance_number,
            SensitiveData::from(1234567890)
        );

        assert_eq!(
            format!("{}", user),
            r#"name Denys, password: hidden, sin: hidden"#
        );

        assert_eq!(
            format!("{:?}", user),
            "User { name: \"Denys\", password: SensitiveData(\"hidden\"), social_insurance_number: SensitiveData(\"hidden\") }"
        );
    }
}
