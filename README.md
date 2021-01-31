# hamster-report
This is a simple program that can export time tracking data from [hamster](https://github.com/thatoddmailbox/hamster) to a CSV.

This is mainly designed so that you can then give that CSV to someone else and get paid for the time you worked.

## Usage
```
USAGE:
    hamster-report [OPTIONS] --database <database> --duration-type <duration-type>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --category <category>
    -d, --database <database>
    -t, --duration-type <duration-type>
    -o, --output <output>                   [default: output.csv]
```

The default path for your hamster database is `~/.local/share/hamster/hamster.db`. An example of running the command might be something like `hamster-report --database ~/.local/share/hamster/hamster.db --duration-type hours --category GenericCorp`, which would write a CSV file with all the time logged in the `GenericCorp` category, measured in hours.

If you do not include the `category` flag, the program will print out a list of all categories in your database. The valid options for `duration-type` are `seconds`, `minutes`, and `hours`.