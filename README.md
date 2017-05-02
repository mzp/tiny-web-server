# :globe_with_meridians: tiny web server
Tiny http server serve static files to public it for other machines at same network.

## :star: Usage

```
# macOS
curl -L -o tiny-web-server https://github.com/mzp/tiny-web-server/releases/download/1.0.0/tiny-web-server-x86_64-apple-darwin

# linux
curl -L -o tiny-web-server https://github.com/mzp/tiny-web-server/releases/download/1.0.0/tiny-web-server-x86_64-apple-darwin

chmod a+x tiny-web-server
./tiny-web-server
```

and access http://localhost:8423

## :wrench: Build

```
docker-compose build
```

## :+1: Commit symbol

|emoji              |mean                                    |
|-------------------|----------------------------------------|
|:wrench:           |improve development environment         |
|:sparkles:         |add new feature                         |
|:lipstick:         |improve the format/structure of the code|
|:hocho:            |split file to improve structure of the code|
|:truck:            |move file to improve structure of the code|
|:memo:             |improve document                        |
|:package:          |improve cargo settings                  |

## :copyright: License
MIT License
