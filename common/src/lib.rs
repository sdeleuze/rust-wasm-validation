use anyhow::{anyhow, Error, Ok};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub firstname: String,
    pub lastname: String,
    pub age: u32,
}

impl Person {
    pub fn validate(&self) -> Result<(), Error> {
        if self.firstname.is_empty() {
            Err(anyhow!("Invalid person, firstname should not be empty"))
        } else if self.lastname.is_empty() {
            Err(anyhow!("Invalid person, lastname should not be empty"))
        } else if self.age == 0 {
            Err(anyhow!("Invalid person, lastname should not be empty"))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Person;

    #[test]
    fn valid() {
        let person = Person {
            firstname: String::from("Sébastien"),
            lastname: String::from("Deleuze"),
            age: 40,
        };
        assert!(person.validate().is_ok())
    }

    #[test]
    fn invalid_firstname() {
        let person = Person {
            firstname: String::from(""),
            lastname: String::from("Deleuze"),
            age: 40,
        };
        assert!(person.validate().is_err())
    }

    #[test]
    fn invalid_lastname() {
        let person = Person {
            firstname: String::from("Sébastien"),
            lastname: String::from(""),
            age: 40,
        };
        assert!(person.validate().is_err())
    }

    #[test]
    fn invalid_age() {
        let person = Person {
            firstname: String::from("Sébastien"),
            lastname: String::from("Deleuze"),
            age: 0,
        };
        assert!(person.validate().is_err())
    }

    #[test]
    fn serialize() {
        let person = Person {
            firstname: String::from("Sébastien"),
            lastname: String::from("Deleuze"),
            age: 40,
        };
        let serialized = serde_json::to_string(&person).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn deserialize() {
        let serialized = r#"{"firstname":"Sébastien","lastname":"Deleuze","age":40}"#;
        let person: Person = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", person);
    }
}
