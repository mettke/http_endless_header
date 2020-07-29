# http_endless_body

Small attack script that tries to overload a HTTP Connection by sending an infinite amount of data in the body section. If the server is compliant it should abort the connection before running out of memory. Also supports encrypted connections. 

## Configure

Settup your env file using

```sh
cp .env.example .env
```

and fill in the required variables to match your setup.

## Execute

Afterwards you can start it using

```sh
cargo run --bin http_endless_body
```

## Results

There are three possible results:

> Wrote x bytes. This looks like a good limit!

Everything should be fine. The server aborted the connection after receiving the printed amount of bytes. The result code will be 0.

> Wrote x bytes. Either you do not have a limit or its very high. You may want to set it to 1_048_576b or lower!

This means that there probably is no limit and the server ran out of memory or the limit is very high. You may want to reduce it. Result code is 1.

> Aborting as we reached a value outside the usize range while sending data. You may want to introduce a limit to your body parsing!

Congratulations, your server was able to buffer quite a lot of data (over 4G) for a Body Value without breaking. But before you celebrate, add a fucking limit! Result code is 2.
