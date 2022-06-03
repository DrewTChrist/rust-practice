/// This struct holds the number of logins made by a user
#[derive(Debug)]
pub struct Logins {
    /// username
    pub username: String,
    /// first time logging in
    pub first_login: String,
    /// last time logging in
    pub last_login: String,
    /// number of time logged in
    pub count: u32,
}

impl Logins {
    /// Returns a Logins struct
    ///
    /// # Arguments
    ///
    /// * `username` - A string holding a username
    /// * `first_login` - A string holding first date logged in
    /// * `last_login` - A string holding the last date logged in
    /// * `count` - A u32 for counting logins
    ///
    pub fn new(username: String, first_login: String, last_login: String, count: u32) -> Logins {
        Logins {
            username,
            first_login,
            last_login,
            count,
        }
    }
}

impl Clone for Logins {
    fn clone(&self) -> Logins {
        Logins {
            username: self.username.clone(),
            first_login: self.first_login.clone(),
            last_login: self.last_login.clone(),
            count: self.count,
        }
    }
}
