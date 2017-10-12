[![Build Status][ci-svg]][ci-url]

[ci-svg]: https://circleci.com/gh/fake-news-detector/api.svg?style=shield
[ci-url]: https://circleci.com/gh/fake-news-detector/api

Fake News Detector API
=======================

## JSON API endpoints

The JSON API is currently running under the url https://fake-news-detector-api.herokuapp.com/

### Categories

The news are flagged on distinct categories, such as Fake News, Click Bait, etc, you can list all with this call:

`GET /categories`

Response format:

`[{ id: int, name: string }]`

### Links

You can list all links and its main voted category with:

`GET /links/all`

Response format:

`[{ id: int, url: string, title: string, category_id: int, count: 1 }]`

This endpoint is used by [Robinho](https://github.com/fake-news-detector/robinho) to fetch the links and train its classification algorithms.

### Votes

You can get all votes to a specific link with:

`GET /votes?url=string&title=string`

Response format:

```
{
  robot: [{ category_id: int, chance: float }],
  people: [{ category_id: int, count: int }]
}
```

To insert a new vote, use this call:

`POST /vote`

Parameters:

`{ uuid: string, url: string, title: string, category_id: int }`

Response format:

`{ link_id: int, category_id: int, uuid: string, ip: string }`

The votes endpoints are used by the [Fake News Extension](https://github.com/fake-news-detector/extension).

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

Expose the database url:

```
export DATABASE_URL="postgres://postgres:password@localhost:5432/fakenews"
```

Then start the app:

```
cargo watch -x run
```


## Deploy

Read more about deploying docker with heroku [on the oficial docs](https://devcenter.heroku.com/articles/container-registry-and-runtime).
