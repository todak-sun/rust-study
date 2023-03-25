use thiserror::Error;

#[derive(Error, Debug)]
enum LoginError {
    #[error("database error")] // 에러가 발생했을 때 함께 표시할 내용
    DatabaseError(#[from] SqlError),

    #[error("password expired")]
    PasswordExpired,

    #[error("user not found")]
    UserNotFound,

    #[error("network connection error")]
    NetworkError(#[from] std::io::Error),

    #[error("wrong password")]
    WrongPassword,
}

fn login(user: &str, password: &str) -> Result<String, LoginError> {
    let connection: Result<Connection, std::io::Error> = connect()?;
    let user_id = get_user_id(user)?;

    if try_password(user_id, password)? {
        let session: Result<String, SqlError> = get_session(user_id)?;
        Ok(session)
    } else {
        Err(LoginError::WrongPassword)
    }
}
fn main() {
    login("kate", "123")
}
