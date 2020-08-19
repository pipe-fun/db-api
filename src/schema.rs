table! {
    users (user_name) {
        user_name -> Varchar,
        user_password -> Varchar,
        user_email -> Varchar,
        active -> Bool,
    }
}

table! {
    active_code (code) {
        code -> Varchar,
        owner -> Varchar,
    }
}

table! {
    check_code (code) {
        code -> Integer,
        owner -> Varchar,
    }
}

table! {
    device (token) {
        token -> Varchar,
	owner -> Varchar,
    }
}

table! {
    task (id) {
        id -> Integer,
        command -> Varchar,
        execute_time -> Time,
        device_token -> Varchar,
        active -> Bool,
    }
}
