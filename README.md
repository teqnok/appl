## Terrible Package Loader
TPL is a fantasy (as in non-viable for a flagship distro release) package manager.

### *TPL is not supported on Windows.*
Support for Windows is not, and will probably never be, on the roadmap. I  *may* look into MacOS/OSX support.

#### How do I use it?
Making a TPL package is *really* easy. Make a single JSON file containing metadata along with the package's binary like this:
```json
{
    "package": {
        "name": "foo",
        "author": "john",
        "version": "",
    }
}
```

