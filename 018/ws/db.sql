-- drop table if exists course;

create table course
(
    id serial primary key,
    teacher_id int not null,
    name varchar(140) not null,
    time timestamp default now()
);

insert into
    course
    (id, teacher_id, name, time)
values
    (1, 1, 'First course', '2022-01-17 05:50:00');

insert into
    course
    (id, teacher_id, name, time)
values
    (2, 1, 'Second course', '2022-01-18 05:45:00');


-- auto-generated definition
create table teacher
(
    id serial primary key,
    name varchar(100),
    picture_url varchar(100),
    profile varchar(2000)
);

create unique index teacher_id_uindex
    on teacher (id);