# TODO

- [x] Load text from file into log widget
- [ ] Parse text from log messages into fields
- [ ] Add simple parsing mode.
  - Users can input the number of fields that are part of a standard log line
  header and lumberjack will split the message on whitespace and use that number
  of fields as the info fields.
- [ ] Test each field for if it can be parsed as a date field.
- [ ] Press space to open filter menu.
  - Users can filter based on all fields in log message.
- [ ] Automatically parse log messages into fields for users.
- [ ] Allow users to customize log parsing.
- [ ] Allow user to hide bottom hotkey information
- [ ] Allow users to change hotkeys
- [ ] Search message text using both fixed-strings and regex.
- [ ] When calculating the max width of a line use the `UnicodeWidthStr` method
from `unicode_width` crate.
- [ ] Support piping into the application.
  - Probably just want to save the text in a temp directory as data is passed in
    and only want to load the area that the user is looking at at any given
    time.
- [ ] Allow the user to hide lines in the log that don't match the given parsing
  filter.
- [ ] Also allow the user to append lines that don't match the parsing filter to
  the last line that matched the filter.
  - This catches things like stacktraces that usually take up many lines but
  don't usually have the standard log info on every line.
