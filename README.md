## Advanced Portable Package Loader
APPL is a fantasy (as in non-viable for a flagship distro release) package manager.

### *APPL is not supported on Windows.*
Support for Windows is not, and will probably never be, on the roadmap. I *may* look into MacOS/OSX support.

#### How do I use it?
Making an APPL package (called a fruit) is *really* easy. Make a single JSON file containing metadata along with the package's binary like this:
```json
{
    "package": {
        "name": "foo",
        "author": "john",
        "version": "1.0.0dev", // semver
        "size": [77, 150] // [int,int]: Download size of package (in MB, not MiB), followed by total install size.
    },
    "install": {
        "destination": "/usr/share/foo/", // path: Where should the package install to? default /usr/share/(package name)
        "script": "echo Thanks for installing `foo`!" // bash: Run bash script when starting the install process
    },
    "post": {
        "script": "echo Install complete. :)" // bash Run bash script when download done. Useful for adding to PATH or linking/verifying dependancies 
    }
}
```

