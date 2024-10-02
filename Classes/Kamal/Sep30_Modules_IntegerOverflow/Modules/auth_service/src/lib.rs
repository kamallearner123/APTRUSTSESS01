#[derive(Debug)]
pub enum AuthError {
    AuthSuccess,
    AuthError
}

pub fn verify_authentication(user:&str, pass:&str) -> Result<AuthError, String> {
    println!("Going to to authenticate user: {} and pass: {}", user, pass);
    return Ok(AuthError::AuthSuccess);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
