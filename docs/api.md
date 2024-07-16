# Front-end developer test server: API

The server implements an extremely simple API with the following endpoints:

## /v1/health

TOKEN required: **no**

This is a simple healthcheck endpoint, to check whether the container is running.

Example request:

```
curl -H "Authorization: Bearer SUPERSECRETTOKEN" https://fedevtest.azurewebsites.net/v1/health
```

Example response:

```
Everything is OK
```

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
  }
]
```

## /v1/models/:id

TOKEN required: **yes**

Fetch detailed information about an individual "model" (building, dwelling). The parameter `id` should match one of the IDs returned by the `/v1/models/` endpoint.

Example request:
```
 curl -H "Authorization: Bearer SUPERSECRETTOKEN" https://fedevtest.azurewebsites.net/v1/models/1
```

Example response:

```

{
  "id": 1,
  "address1": "24 Hesters Way Road",
  "address2": "",
  "city": "Cheltenham",
  "state": "Gloucestershire",
  "postal_code": "GL51 0DA",
  "thumbnail": "/thumbs/1.png",
  "model": "https://planarificfedevtest.blob.core.windows.net/models/house1.glb"
}
```
