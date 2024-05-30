-- ./migrations/{timestamp}_create_users_table.sql

create table images (
  id uuid primary key,
  filename varchar(100) not null,
  data bytea not null,
  created_at timestamptz default now()
);


-- insert into images (filename, data) 
-- values (
--   'wallpaper.webp', 
--  pg_read_binary_file('../src/public/wallpaper.webp')
-- )
