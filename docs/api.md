# Front-end developer test server: API

The server implements an extremely simple API with the following endpoints:

## /v1/models

TOKEN required: **yes**

Fetch a list of "models" (buildings, dwellings).

Example request:
```
 curl -H "Authorization: Bearer SUPERSECRETTOKEN" https://fedevtest.azurewebsites.net/v1/models
```

Example response:

```
[
  {
    "id": 3,
    "description": "9 Vancouver Road BN13 2SN",
    "thumbnail": "/thumbs/3.png"
  },
  {
    "id": 6,
    "description": "41 Tresawls Avenue TR1 3LA",
    "thumbnail": "/thumbs/6.png"
  },
  {
    "id": 2,
    "description": "10 Beehive Court WF15 7BT",
    "thumbnail": "/thumbs/2.png"
  },
  {
    "id": 4,
    "description": "Hiram House, Spring Lane GL5 2DU",
    "thumbnail": "/thumbs/4.png"
  },
  {
    "id": 1,
    "description": "24 Hesters Way Road GL51 0DA",
    "thumbnail": "/thumbs/1.png"
  },
  {
    "id": 5,
    "description": "316B Cricklewood Lane NW2 2QE",
    "thumbnail": "/thumbs/5.png"
  },
  {
    "id": 7,
    "description": "9 Venus Street BS49 5HA",
    "thumbnail": "/thumbs/7.png"
  },
  {
    "id": 8,
    "description": "4 Wheatley Grove WS6 6ES",
    "thumbnail": "/thumbs/8.png"
  },
  {
    "id": 9,
    "description": "6 Regent Court GU19 5QD",
    "thumbnail": "/thumbs/9.png"
  },
  {
    "id": 10,
    "description": "Denewood SN10 3LL",
    "thumbnail": "/thumbs/10.png"
  }
]
```