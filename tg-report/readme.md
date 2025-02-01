
example open

```bash
$ cargo run -- -t 'hello!' open '/home/tyoma/Pictures/image1.png'
```

```bash
$ cargo run -- close --credit-requests 3 --credit-responses 1 --registrations 7 --cash 35228 /home/tyoma/Pictures/image1.png
```

```bash
Usage: tg-report [OPTIONS] [COMMAND]
```

```bash
Commands:
  open      Отправить открытие смены
  close     Отправить закрытие смены
  help      Print this message or the help of the given subcommand(s)

Options:
  -t, --text <TEXT>  
  -h, --help         Print help
  -V, --version      Print version
  ```

Usage: tg-report open <PATH>

```bash
Arguments:
  <PATH> 
  
  Options:
  -h, --help  Print help
```

Usage: tg-report close [OPTIONS] <PATH>

```bash
Arguments:
  <PATH>

  Options:
      --credit-requests <CREDIT_REQUESTS>    [default: 0]
      --credit-responses <CREDIT_RESPONSES>  [default: 0]
      --registrations <REGISTRATIONS>        [default: 0]
      --cash <CASH>                          [default: 0]
      --adapter <ADAPTER>                    [default: 0]
  -h, --help                                 Print help
```