# Prometheus Command Exporter
[![Build Status](https://drone.k8s.array21.dev/api/badges/TheDutchMC/prometheus-command-exporter/status.svg)](https://drone.k8s.array21.dev/TheDutchMC/prometheus-command-exporter)

Export command output data to Prometheus

## Usage
- By default the program will read it's configuration from `/etc/prometheus-command-exporter/config.yml`, this behaviour can be changed with the `--config` flag. 
- The program by default listens on port 10405, this can be changed with the `--port` argument
- Verbosity can be set with `-v`. You can provide this argument multiple times to increase the verbosity up to a maximum of four times.


## Configuration
```yaml
exports:
- name: String				# The name of the metric, e.g 'sm_power_usage'
  description: String		# Description of the metric, e.g 'Power usage in whole Watts'
  type: PrometheusType		# The type of metric, e.g Gauge
  command: String			# The command to execute, e.g 'ipmitool dcmi power reading | tr -s ' ' | grep Average | rev | cut -d ' ' -f 2 | rev'. The produced value must be parsable as a floating point value. The command is executed with the sh shell.
```
Where `PrometheusType` is one of:
```
Gauge
Counter
```

## Licence
This project is licenced under the MIT licence or the Apache-2.0 licence, at your descretion.
