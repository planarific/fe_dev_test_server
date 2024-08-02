# Front-end developer test server: API

The API server serves the following static assets:

## /planarific.webp


TOKEN required: **yes**

The Planarific logo.

Example request:

```
curl -H "Authorization: Bearer SUPERSECRETTOKEN" https://fedevtest.azurewebsites.net/planarific.webp
```

Example response:

![Planarific logo](planarific.png)

