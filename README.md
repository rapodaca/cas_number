# A PostgreSQL Extension for CAS Numbers

This project builds a Postgres extension capable of validating CAS Registry Numbers and generating random instances of them. Read more about it on [the blog post](https://depth-first.com/articles/2021/09/07/a-rust-postgresql-extension-for-cas-numbers/).

```sql
drop extension if exists cas_number cascade;
create extension cas_number;
```

```sql
create table cas_numbers (id int, cas_number casnumber);
-- CREATE TABLE
```

```sql
insert into cas_numbers
select generate_series(1, 100000) as id,
       random_cas_number();
-- INSERT 0 100000
```

```sql
create index cas_numbers_cas_number on cas_numbers (cas_number);
-- CREATE INDEX
```

```sql
explain analyze
select   id, cas_number
from     cas_numbers
where    cas_number > '67-64-1'
order by cas_number
limit    25;
```

```sql
insert into cas_numbers (id, cas_number) values (100001, '111-11-1');
-- ERROR:  invalid CAS Number
```