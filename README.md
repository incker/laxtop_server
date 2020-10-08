# Laxtop

Application for retailers to simplify making wholesale orders

# Installation

Install [Docker](https://docs.docker.com/engine/installation/) & [Docker-Compose](https://docs.docker.com/compose/install/)

Project requires external docker database container.
Create a `docker-compose.yml` file anywhere you like with such content:

```bash
version: "3.7"

services:
  db:
    image: "mysql:8.0.20"
    container_name: mydb
    # Set max_allowed_packet to 256M
    command: --default-authentication-plugin=mysql_native_password --max_allowed_packet=32505856
    restart: always
    networks:
      - common_mysql
    ports: 
      - "3306:3306"
    volumes:
      - ./data:/var/lib/mysql
    environment:
      MYSQL_DATABASE: laxtop
      MYSQL_ROOT_PASSWORD: oOtIGj47bzQLOo
      MYSQL_USER: incker
      MYSQL_PASSWORD: xX03XfYTeDj5Xx
networks:
  common_mysql:
    driver: bridge
```

Make `docker-compose up -d` in a directory where file located

When database is up, steps to launch project:


```bash
# clone project into new directory
git clone https://github.com/incker/laxtop_server.git laxtop_server
cd laxtop_server

# create api/.env file
# with db url (have to be the same as in Rocket.toml file) to connect with external db
# other env variables (DOMAIN_NAME, TELEGRAM_TOKEN, NODE_PATH) can be omitted for basic launch

touch api/.env
echo DATABASE_URL=mysql://user:password@db/laxtop >> api/.env
echo DOMAIN_NAME=https://laxtop.com >> api/.env
echo TELEGRAM_TOKEN=000000000:XXXX_xxxxxx-xxx-xxx-xxxxxxxxxxxxxxx >> api/.env
echo NODE_PATH=/usr/src/app/node_firebase/src >> api/.env

# start it up, db migration will be executed automaticaly
docker-compose up
```

Open [http://localhost:3000](http://localhost:3000) (or `{docker ip}:3000` on windows) to view it in the browser.

The api server will restart if you make edits in the `/api/src` folder.


# Workflow

I like to use a couple terminals (tabbed) one to run the containers and watch stdout, the other to run any other commands.

### Terminal #1
1. `docker-compose up` to start
2. `Ctrl+C` to stop
3. Sometimes `docker-compose down` to dispose of containers

### Terminal #2
Examples of useful commands.

* `docker-compose exec api bash`
    * `cargo upgrade`
    * `diesel setup`
    * `diesel migration generate {name}`
    * `diesel migration run`
    * `diesel migration redo`
    * See [Rocket](https://rocket.rs) & [Diesel](http://diesel.rs) docs for more.


# Troubleshooting

If Rocket ever fails to build with an error concerning nightly version date, this can be fixed by rebuilding the docker container with the latest nightly `docker-compose build --no-cache api`.

# Line separator config

Settings in Intellij: 
`Settings -> Code Style -> Line separator` Set `Unix and OS X (\n)`

Settings in Git: 

```bash 
# just use LF everywhere on windows
git config --global core.autocrlf false
```
