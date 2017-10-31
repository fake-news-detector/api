[![Build Status][ci-svg]][ci-url]

[ci-svg]: https://circleci.com/gh/fake-news-detector/api.svg?style=shield
[ci-url]: https://circleci.com/gh/fake-news-detector/api

Fake News Detector API
=======================

## JSON API endpoints

The JSON API is currently running under the url https://api.fakenewsdetector.org/

### Categories

The news are flagged on distinct categories, such as Fake News, Click Bait, etc, you can list all with this call:

`GET /categories`

Response format:

`[{ id: int, name: string }]`

### Links

You can list all links and its main voted category with:

`GET /links/all`

Response format:

`[{ id: int, url: string, title: string, content: null | string, category_id: int, count: 1 }]`

This endpoint is used by [Robinho](https://github.com/fake-news-detector/robinho) to fetch the links and train its classification algorithms.

### Votes

You can get all votes to a specific link with:

`GET /votes?url=string&title=string`

Response format:

```
{
  verified: null | { category_id: int },
  robot: [{ category_id: int, chance: float }],
  people: [{ category_id: int, count: int }]
}
```

The `verified` key is only present if the given url is listed on one of our [manually verified links](https://github.com/fake-news-detector/api/blob/master/src/data/verified_list.rs).
When present, this value should be used over robot and people's guesses.

To insert a new vote, use this call:

`POST /vote`

Parameters:

`{ uuid: string, url: string, title: string, category_id: int }`

Response format:

`{ link_id: int, category_id: int, uuid: string, ip: string }`

The votes endpoints are used by the [Fake News Extension](https://github.com/fake-news-detector/extension).

# Contributing

If you want to help the project, you can fork it and run on your machine, for more details, read the [CONTRIBUTING.md](https://github.com/fake-news-detector/api/blob/master/CONTRIBUTING.md) guide.
