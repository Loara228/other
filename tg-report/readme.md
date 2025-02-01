
<div align="center"><h3>Open</h3></div>

_example_

```bash
$ cargo run -- -t 'hello!' open '/home/tyoma/Pictures/image1.png'
```

```bash
Usage: tg-report open <PATH>

Arguments:
  <PATH>  

Options:
  -h, --help  Print help
```


<div align="center"><h3>Close</h3></div>

_example_

```bash
$ cargo run -- close --credit-requests 3 --credit-responses 1 --registrations 7 --cash 35228 /home/tyoma/Pictures/image1.png
```

```bash
Usage: tg-report close [OPTIONS] <PATH>

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