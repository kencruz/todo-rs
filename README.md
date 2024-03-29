# todo-rs

A todo app written in Rust, using Actix, Diesel and SQLite. Frontend uses Vue, Axios, and Bulma. A live demonstration can be seen [here.](http://todo.kencruz.ca)

## Getting the image:
The image has been pushed to [Dockerhub](https://hub.docker.com/r/kencruz/todo-rs/)  
So you can just:

```
docker pull kencruz/todo-rs
```
## Running the image:
- Run the Docker container: `docker run -p 8088:8088 --rm --name todo kencruz/todo-rs`
- Goto [http://localhost:8088](http://localhost:8088)

## Building the image from source.:

- First, clone this project.

```
$ git clone https://github.com/kencruz/todo-rs.git
```

- Second, change directory to the just now cloned repository.

```
$ docker build -t kencruz/todo-rs .
```

