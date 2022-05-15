create table if not exists users (
    id              bigserial       not null    primary key,
    email           varchar(255)    default ''  unique,
    username        varchar(255)    not null,
    gender          varchar(10)     default 'unknown',
    reg_date        date            default current_date,
    recently        date            default current_date,
    password        varchar(255)    not null,
    salt            varchar(20)     not null,
    introduction    text            default '',
    avatar          varchar(255)    default '',
    role            int             default 0
);

create table if not exists topics (
    id      bigserial       not null    primary key,
    name    varchar(255)    not null,
    image   varchar(255)    not null
);

insert into topics (name, image) values ('Technology', 'https://picsum.photos/seed/2/2000/1000');
insert into topics (name, image) values ('Life', 'https://picsum.photos/seed/3/2000/1000');
insert into topics (name, image) values ('History', 'https://picsum.photos/seed/5/2000/1000');


create table if not exists contents (
    id              bigserial       not null    primary key,
    title           varchar(60)     not null,
    source          varchar(255)    not null,
    author          int             not null,
    topic           int             not null,
    description     text            default '',
    up_date         date            default current_date,
);