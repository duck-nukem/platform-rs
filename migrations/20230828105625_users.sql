create table main.users
(
    id            int
        primary key,
    name          varchar(25) not null,
    password_hash varchar(30) not null
);

