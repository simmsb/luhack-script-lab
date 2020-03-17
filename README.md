# luhack-script-lab

Some lab for teaching scripting

## Running

```sh
docker build -t luhack-script-lab --build-arg FLAG="not_a_flag{nope}" lab/.
docker run --rm \
    -v /var/run/docker.sock:/var/run/docker.sock \
    --net=host \
    -it nitros12/container-per-ip \
    luhack-script-lab -p 10000-10500 -p 6969
```

## Attempting it yourself

1. Start it up:
```
docker run --rm -it -p 10000-10500:10000-10500 -p 6969:6969 nitros12/luhack-script-lab
```

2. Poke `localhost:6969`
