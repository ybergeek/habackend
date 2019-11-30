create table locations(
  device_id    text  primary key
, location     text
, environment  text
);
INSERT INTO locations (device_id,"location",environment) VALUES
('outhouse','Lovis','outhouse')
,('inhouse','Lovis','inhouse')
,('mandolyn-temperaturehumidity-11-humidity','Lovis','badtunna-tryck')
,('mandolyn-temperaturehumidity-11','Lovis','badtunna')
