alter table users
    add locale varchar not null default 'en';
alter table users
    add constraint username_uniqueindex unique (name);

