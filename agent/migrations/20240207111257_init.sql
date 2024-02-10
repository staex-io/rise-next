create table stations (
  address text primary key,
  location text not null,
  price integer not null
);

create table agreements (
  id serial primary key,
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
  unique (drone, station, landlord)
  on conflict replace
);
