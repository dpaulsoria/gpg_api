-- ./migrations/{timestamp}_create_users_table.sql

create table users (
  id integer primary key,
  username varchar(100) not null,
  password varchar(100) not null,
  email varchar(100) not null,
  created_at timestamptz default now()
);


-- insert into users (username, password, email) values

