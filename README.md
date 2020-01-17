# Snowy Simple Logger

Object oriented library created in Rust to logging data into stdout and files.

## Project structure

```txt
snowy_simple_logger
  | Console
    | success(message: str) # staticmethod
    | error(message: str) # staticmethod
    | warn(message: str) # staticmethod
    | info(message: str) # staticmethod
  | File(log_file_full_name: str) # constructor
    | path # points to the log_file_full_name parameter in the constructor
    | success(message: str)
    | error(message: str)
    | warn(message: str)
    | info(message: str)
```

## Special cases

When you pass to the `snowy_simple_logger.File(log_file_full_name: str)` constructor empty string (`""` or `''`), library will use as path to the log file `"log.log"` next to interpreted Python file.

When you pass to the `snowy_simple_logger.File(log_file_full_name: str)` constructor word file name without extension `.log`, library will use as path this name but with injected to the end `.log`.