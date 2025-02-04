# Keylogger in Rust

## About
The Rust binary `Argus` is a keylogger that refers to Argus Panoptes, the all-seeing giant in Greek mythology with many eyes, emphasizing vigilance. The keylogger records keystrokes by memoizing them into a buffer set with capacity constraint that when met, the keystrokes are passed onto a handle function that either saves the strokes onto a file or sends them out through email depending on the specified preference.

## Examples
Initializing Argus to write the keystrokes into a file.

(*Keep in mind you should have the environment variable `PATH` defined to specify the cache location.*)
```console
argus file
```

Initializing Argus to send the keystrokes through email. Implemented email domains are *gmail.com* and *outlook.com*.

(*Environment variables are the most critical here, you should have set the environment variables `EMAIL` and `PASSWORD`*)

**For the password, it is recommended to use an app password**

If you don't know what an app password is, refer to this guide: [App Passwords](https://docs.saleshandy.com/en/articles/8064495-app-password-what-why-and-how-to-set-it-up)
```console
argus email --cap 100
```
