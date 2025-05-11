# Usage

## Prebuilt executable

https://tiralabra.eliaseskelinen.fi/ is set up to build and display the latest valid version of the project automatically.

## Building it yourself

1. Download the Code

```bash
git clone git@github.com:xypine/tiralabra.git
```

### Using Docker

The docker image should work, but it's not guaranteed as docker images change over time.

2. Build the Docker Image

```bash
docker build . -t aaltofunktionromautus
```

3. Run the Docker Image

```bash
docker run --rm -it -p 3000:80 aaltofunktionromautus
```

This will expose the web interface on your machine at [localhost:3000](http://localhost:3000/). You can change the 3000 to any other port you like by changing the first value after `-p`.
