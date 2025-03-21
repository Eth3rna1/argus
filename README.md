# Keylogger in Rust

## About
The Rust binary name `Argus` refers to Argus Panoptes, the all-seeing giant in Greek mythology with many eyes, emphasizing vigilance. The keylogger records keystrokes by memoizing them into a buffer with a predefined capacity constraint that when met, the keystrokes are passed onto a handle function that either saves the strokes onto a file or sends them out through email depending on the specified preference. **The default `buffer capacity` is `100` characters long**

## Examples
### Initializing Argus to write the keystrokes into a file.

(*Keep in mind you should have the environment variable `FILE` defined to specify the cache location.*)

When defining the `FILE` environment variable, the name of the cache file is arbitrary. If the file doesn't exist, the program will create the file itself, otherwise, it will append the keystrokes to such file.
```console
argus file
```

### Initializing Argus to send the keystrokes through email.

(*Environment variables are the most critical here, you should have set the environment variables `EMAIL` and `PASSWORD`*)

**For the password, it is recommended to use an app password**

If you don't know what an app password is, refer to this guide: [App Passwords](https://docs.saleshandy.com/en/articles/8064495-app-password-what-why-and-how-to-set-it-up)
```console
argus email --cap 200
```

### Using optional flags

The `--log` flag prints out the state of the buffer regarding its length onto the standard output.
In other words, the program will print out how many characters are in such buffer
before having to handle the keystrokes.

```console
argus file --log
```

If any further help or reminder is needed, you can pass the `--help` flag.
