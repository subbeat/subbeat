# subbeat
subbeat

## Example

```
subbeat grafana http://localhost:3000 eyJrIjoiWnRRMTNmcGpvTHNPb3UzNzdUNUphRm53Rk9tMTNzOTQiLCJuIjoic3ViYmVhdC10ZXN0IiwiaWQiOjF9 "/api/datasources/proxy/1/api/v1/query_range" "rate(go_memstats_alloc_bytes_total[5m])" 1634672070 1635110190 15
```


### Datasources

#### Prometheus

```
subbeat prometheus http://localhost:9090/ "rate(go_memstats_alloc_bytes_total[5m])" 1634172070 1635110190 15
```

#### Influx
```
subbeat influx http://localhost:8086 5abe4759f7360f1c sCAB2MVo8TJxhUH8UDJZIeCPwf-cykBtO0jhr207qCQSZ9d43JXObCYK_uAml2BL26JBYFauz95yIeC51kxQol== 'from(bucket:"main-backet") |> $range |> filter(fn:(r) => r._measurement == "cpu")' 1635985640 1635986810 10
```

#### Grafana
* Prometheus
