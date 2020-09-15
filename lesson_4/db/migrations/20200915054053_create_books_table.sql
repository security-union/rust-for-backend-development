-- migrate:up
create table movies (
  title varchar(255) PRIMARY KEY,
  genre varchar(255) NOT NULL
);

-- migrate:down
drop table movies;
