create table conditions(
  time         timestamp with time zone  not null,
  device_id    text references locations(device_id),
  temperature  numeric,
  humidity     numeric,
  PRIMARY KEY(time, device_id)
);
SELECT create_hypertable('conditions', 'time', chunk_time_interval => interval '1 day');
