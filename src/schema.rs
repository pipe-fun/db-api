table! {
    pipe_users (id) {
        id -> Integer,
        user_name -> Varchar,
        user_password -> Varchar,
        user_email -> Varchar,
        user_registered_time -> Datetime,
        user_recently_login_time -> Datetime,
    }
}