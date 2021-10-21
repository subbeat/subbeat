# subbeat
subbeat

## Example

```
subbeat grafana http://localhost:3000 eyJrIjoiWnRRMTNmcGpvTHNPb3UzNzdUNUphRm53Rk9tMTNzOTQiLCJuIjoic3ViYmVhdC10ZXN0IiwiaWQiOjF9 "/api/datasources/proxy/1/api/v1/query_range" "rate(go_memstats_alloc_bytes_total[5m])" 1634672070 1634672970 15
```


### Datasources
#### Grafana
* Prometheus