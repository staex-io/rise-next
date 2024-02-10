create table stations (
  address text primary key,
  location text not null,
  price integer not null
);

create table agreements (
  id integer primary key autoincrement,
  station text not null,
  entity text not null,
  amount integer not null,
  is_signed bool not null,
  unique (station, entity)
  on conflict replace
);

create table landings (
  id integer primary key,
  drone text not null,
  station text not null,
  landlord text not null,
  is_taken_off bool not null default false,
  is_rejected bool not null default false,
  date integer not null,
  unique (drone, station, landlord)
  on conflict replace
);

create table stats (
  address text primary key,
  landings integer not null,
  amount integer not null
);

create trigger update_stats after insert on landings begin
--
-- Drone 
--
insert into
  stats (address, landings, amount)
values
  (
    new.drone,
    1,
    (
      select
        -amount
      from
        agreements
      where
        station = new.station
        and entity = new.drone
    )
  )
on conflict do
update
set
  landings = landings + 1,
  amount = amount + (
    select
      - amount
    from
      agreements
    where
      station = new.station
      and entity = new.drone
  );

--
-- Station 
--
insert into
  stats (address, landings, amount)
values
  (
    new.station,
    1,
    (
      select
        amount
      from
        agreements
      where
        station = new.station
        and entity = new.drone
    )
  )
on conflict do
update
set
  landings = landings + 1,
  amount = amount + (
    select
      amount
    from
      agreements
    where
      station = new.station
      and entity = new.drone
  );

update stats
set
  amount = amount + (
    select
      - amount
    from
      agreements
    where
      station = new.station
      and entity = new.landlord
  )
where
  address = new.station;

--
-- Landlord.
--
insert into
  stats (address, landings, amount)
values
  (
    new.landlord,
    1,
    (
      select
        amount
      from
        agreements
      where
        station = new.station
        and entity = new.landlord
    )
  )
on conflict do
update
set
  landings = landings + 1,
  amount = amount + (
    select
      amount
    from
      agreements
    where
      station = new.station
      and entity = new.landlord
  );

end;
