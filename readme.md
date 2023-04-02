# bjsubway

A toy for speedrunning rust. Don't be serious xD.

APIs from: [北京地铁官网](https://map.bjsubway.com/)

## Features
- [x] Update data.
- [x] Print the name of all lines.
- [x] Print details of a given line.
- [x] Find a path from two given destinations.
## Usages

```
Usage: bjsubway.exe [OPTIONS] [COMMAND]

Commands:
  update
  lines
  detail
  find
  help    Print this message or the help of the given subcommand(s)

Options:
  -l, --lcode <LCODE>  [default: ]
  -f, --from <FROM>    [default: ]
  -t, --to <TO>        [default: ]
  -h, --help           Print help
```

See examples.txt


## APIs

### All lines and stations' information

`/subwaymap/beijing.xml`

### Interchange information

`/subwaymap/interchange.xml`

### Information of all stations

`/subwaymap/stations.xml`

