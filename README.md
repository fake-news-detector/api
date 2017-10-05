Fake News Detector API
=======================

## How to run

To run the project you will need to have Docker installed and run:

```
docker-compose up
```

That's it!

## How to run outside docker

If you want to run the app outside docker for faster development flow, you will need to install [nightly rust](https://doc.rust-lang.org/1.2.0/book/nightly-rust.html), and [cargo watch](https://github.com/passcod/cargo-watch).

Also, you will need to have a postgres database running, if you don't, you can run it with docker-compose:

```
docker-compose up database
```

Then start the app:

```
cargo watch -x run
```


## Deploy

Read more about deploying docker with heroku [on the oficial docs](https://devcenter.heroku.com/articles/container-registry-and-runtime).
