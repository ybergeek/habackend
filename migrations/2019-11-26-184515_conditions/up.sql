create table conditions(
  time         timestamp with time zone  not null,
  device_id    text ,
  temperature  numeric,
  humidity     numeric,
  PRIMARY KEY(time, device_id),
  FOREIGN KEY (device_id) REFERENCES locations (device_id)
);
--SELECT create_hypertable('conditions', 'time', chunk_time_interval => interval '1 day');

create or replace view avg_min_max as select
    c.device_id,
    c.temperature,
    c.time
    from conditions c,locations l
    where c.device_id=l.device_id
    and l.show_in_graph = 't';
