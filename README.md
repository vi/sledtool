# sledtool
CLI tool to work with [Sled](https://github.com/spacejam/sled) key-value databases.

```
$ sledtool --help

Usage: sledtool <dbpath> <command> [<args>]

CLI tool to work with Sled databases

Options:
  --help            display usage information

Commands:
  export            Outout entire content of the database to JSON with
                    hex-encoded buffers
  import            Import entire content of the database from JSON with
                    hex-encoded buffers
  get               Get value of specific key from the database
  set               Set value of specific key in the database
  nop               No operation, just open and close the database
  idle              Open Sled database, then wait indefinitely
  treenames         List tree names
  genid             Generate monotonic ID
  checksum          Call `checksum` and output the result
  sizeondisk        Call `size_on_disk` and output the result
```

```
$ sledtool <dbname> get --help

Usage: sledtool get <key> [-t <tree>] [-r] [-R] [-T] [-g] [-l] [-K] [-q]

Get value of specific key from the database

Options:
  -t, --tree        tree to use
  -r, --raw-value   inhibit hex-encoding the value
  -R, --raw-key     inhibit hex-decoding or hex-encoding the key
  -T, --raw-tree-name
                    inhibit hex-decoding the tree name
  -g, --gt          use `get_gt` instead of `get`
  -l, --lt          use `get_lt` instead of `get`
  -K, --print-key   print key in addition to the value, with `=` sign in between
  -q, --quiet       do not print `Not found` to console, just set exit code 1
  --help            display usage information
```

```
$ sledtool qqq export
{
 "5f5f736c65645f5f64656661756c74":{
  "71717132": "71776572747961736466"
 }
}
```