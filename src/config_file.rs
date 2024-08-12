use std::{fs::File, path::Path};
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
pub struct UserConfig {
    pub email: String,
    pub password: String,
    pub auth_header: String
}

fn get_config_file(fp: &str) -> Result<File, Error> {
    let file_path = Path::new(fp);
    return File::open(file_path);
}

pub fn read_config_file(fp: &str) -> Option<UserConfig> {
    let config_file = match get_config_file(fp) {
        Ok(file) => file,
        Err(error) => {
            println!("Missing or invalid config file: {:?}", error);
            return None
        }
    };

    let config_buff = BufReader::new(config_file);
    let config_lines = config_buff.lines();
    let mut user_email = String::new(); 
    let mut user_password = String::new();


    for line in config_lines {
        let parts = line.unwrap_or_default();
        let split_parts: Vec<&str> = parts.split("=").collect();
        match split_parts[0] {
            "email" => {
                user_email = split_parts[1].to_string();
            },
            "password" => {
                user_password = split_parts[1].to_string();
            },
            "auth_header" => {
                auth_header = split_parts[1].to_string();
            },
            _ => (),
        }
    }

    if Some(&user_email)?.is_empty() {
        return None;
    }

    if Some(&user_password)?.is_empty() {
        return None;
    }
    if Some(&auth_header)?.is_empty() {
        return None;
    }

    return Some(UserConfig{
        email: Some(user_email).unwrap(),
        password: Some(user_password).unwrap(),
        auth_header: Some(auth_header).unwwrap()
    });
}


#[cfg(test)]
mod tests {
    // TODO: Currently relies on fixture test data in ./config/test.txt
    use std::io;

    use super::*;

    #[test]
    fn test_get_config_file_not_found() {
        let result = get_config_file("fake").map_err(|e| e.kind());
        match result {
            Err(error) => assert_eq!(error, io::ErrorKind::NotFound),
            _ => assert!(false, "Unexpected error type")
        }
    }

    #[test]
    fn test_read_config_file_missing_file() {
        let result = read_config_file("fake");
        assert!(result.is_none());
    }

    #[test]
    fn test_read_config_file_invalid() {
        let result = read_config_file("./config/test_wrong.txt");
        println!("{:?}", result);
        assert!(result.is_none());
    }

    #[test]
    fn test_read_config_file_success() {
        assert!(
            read_config_file("./config/test.txt")
                .is_some_and(|x| x.email == "foo@bar.com")
        );
    }
}