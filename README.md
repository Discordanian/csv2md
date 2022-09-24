# csv2md
CSV to Mark Down table.

Takes a CSV file and displays a markdown version of it as a table.

Default is reading from STDIN and writing to STDOUT.

Can be made an OSX Service and replace text in place as seen here: https://youtu.be/YSHRy8tn8kE


## Help

```
USAGE:
    csv2md [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Turn on Debug
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --infile <FILE>           A CSV Input File
    -o, --outfile <FILE>          A Mark Down Output File to write to
```


## Example

```
Header 1, Header II, Header Three
From my childhoods hour, I have not, been
As others, were I have, not seen
As, others saw - I could, not bring
My, passions, from a common spring
```


| Header 1                |  Header II            |  Header Three         |
|-------------------------|-----------------------|-----------------------|
| From my childhoods hour |  I have not           |  been                 |
| As others               |  were I have          |  not seen             |
| As                      |  others saw - I could |  not bring            |
| My                      |  passions             |  from a common spring |

