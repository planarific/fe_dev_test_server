# Front-end developer test server

This repository holds the source code for a simple API server, against which front-end developer candidates may develop demo UI applications.

## The task

1. Develop a simple web UI application that combines both traditional 2D web UI elements, and 3D visualisations, following the [UX brief](docs/ux_brief.md). Your UI should utilise [the API provided by the test server](docs/api.md). In addition, the server offers some [static assets](docs/assets.md).
2. Develop in github or gitlab and send us the URL, so we can discuss your code
3. **Either** deploy your UI somewhere on the internet, and send us the URL so we can play with it...
4. ...**or** provide a Dockerfile in your repo, so that we can build a Docker container image and run it ourselves.

## Running the API server

We provide a running API server at `https://fedevtest.azurewebsites.net/`.

Alternatively you can clone this repo to build and run the Docker image, e.g.:

```
 docker build -t fe_dev_test_server .

 docker run -it --rm --env TOKEN=PASSWORD -p3000:3000 --name my-running-app fe_dev_test_server
```

This will build the container image and run it, with the API listening on port 3000. 

## API token

The API server requires an *API token*. If you make a request without the API token, you'll get a `401 Unauthorised` response. 

If you are using `https://fedevtest.azurewebsites.net/` we will send you the token by email. 

If you are running a local version of the server, the API token comes from the `TOKEN` environment variable, which you can see in the `docker run` example above.

You should make best efforts in your code to protect the token!

