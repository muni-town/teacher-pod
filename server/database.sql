create table if not exists auth (
    id      varchar(128)    not null   primary key,
    account int             not null,
    expire  int             not null
);

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

-- test data
insert into users (email, username, password, salt) values ('mrxzx@qq.com', 'mrxiaozhuox', '22b48f7a98d9a8d684c7000dde01ef6e', 'ccIvwLYPegqy');

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
    type            int             default 0,
    title           varchar(60)     not null,
    source          varchar(255)    not null,
    author          int             not null,
    topic           int             not null,
    description     text            default '',
    cover_image     varchar(255)    default '',
    up_date         date            default current_date
);

create table if not exists config (
    id              bigserial       not null    primary key,
    name            varchar(100)    not null,
    value           text            not null,
)
--- default config data
insert into config (name, value) values ("RecommendList", "[]");

create type actype as enum ('collect', 'follow', 'publish', '')
create table if not exists activity (
    id              bigserial       not null    primary key,
    account         int             not null,
    type            actype          not null,
    target          int             not null,
    curr_time       timestamp       default current_timestamp
);