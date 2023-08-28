create table main.users
(
    id            integer         not null
        primary key autoincrement,
    name          varchar(25) not null,
    password_hash varchar(30) not null
);

