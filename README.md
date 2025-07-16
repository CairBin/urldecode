# urldecode

## Description

It takes strings like `https%3A%2F%2Fwww.google.ca%2F%3Fgws_rd%3Dssl%23q%3Durl%2Bdecoding` and turns it into strings like `https://www.google.ca/?gws_rd=ssl#q=url+decoding`.

Note that it doesn't convert +'s to spaces(as per the spec), it's expected that the programmer does this on their own.

Refer to repository [abejfehr/URLDecode](https://github.com/abejfehr/URLDecode) and [eptansuo/FileNameDecode](https://github.com/EPTansuo/FileNameDecode).

## Usage

If you only want to input text and output it on the command line, you can use this method.

```sh
urldecode [CONTENT]
```

You can also input the decoded string into a specified file, for example:

```sh
urldecode [CONTENT] -o /path/to/output_file
# urldecode [CONTENT] --ouput /path/to/output_file
```

Or parse the content of the file to a specified file
```sh
urldecode -f /path/to/input_file -o /path/to/output_file
# urldecode --file /path/to/input_file --output /path/to/output_file
```

In some cases, you need to decode their file names instead of their content.So the command also provides this way:

```sh
urldecode -n %e6%b5%8b%e8%af%95%e6%96%87%e4%bb%b6.txt
# urldecode --name %e6%b5%8b%e8%af%95%e6%96%87%e4%bb%b6.txt
```

It will eventually be processed into `测试文件.txt`.It should be noted that `--name` does not change any content. It just changes the file name.


## Build

Build and install the program.

```sh
make
make install
```

On the Windows system, the program is installed by default to `%LOCALAPPDATA%\urldecode\bin`. And On Linux or macOS, it will be installed to `/usr/local/bin/urldecode`.You may need to configure environment variables.
