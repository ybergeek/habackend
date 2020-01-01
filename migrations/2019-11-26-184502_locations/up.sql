create table locations(
  device_id    text  primary key
, location     text not null
, environment  text not null
,show_in_graph boolean not null
,  unit varchar(10) not null
,
);
INSERT INTO locations (device_id,"location",environment,show_in_graph,unit) VALUES
('outhouse','Lovis','outhouse',true,'&deg;C')
,('inhouse','Lovis','inhouse',true,'&deg;C')
,('mandolyn-temperaturehumidity-11-humidity','Lovis','badtunna-tryck',false,'%')
,('mandolyn-temperaturehumidity-11','Lovis','badtunna',false,'&deg;C')
