CREATE TABLE IF NOT EXISTS users (
    uasr_name TEXT NOT NULL PRIMARY KEY,
    user_password TEXT NOT NULL,
    user_email TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS task (
    id INT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    succeed_count INT NOT NULL,
    failed_count INT NOT NULL,
    last_executed TIMESTAMP NULL DEFAULT NULL,
    owner TEXT NOT NULL,
    command TEXT NOT NULL,
    execute_time TIME NOT NULL,
    device_token TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE TABLE IF NOT EXISTS device (
    token TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    owner TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS active_code (
    code TEXT NOT NULL PRIMARY KEY,
    owner TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS check_code (
    code TEXT NOT NULL PRIMARY KEY,
    owner TEXT NOT NULL
);