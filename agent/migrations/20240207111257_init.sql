create table stations (
  address text primary key,
  location text not null,
  price text not null
);

create table agreements (
  id integer primary key autoincrement,
  station text not null,
  entity text not null,
  amount text not null,
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
  amount text not null
);
