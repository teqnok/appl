## Advanced Portable Package Loader
APPL is a package manager with extensive support for AppImages and custom packages.

## Why?
I created `appl` after finding that there was difficulty in efficiently and automatically managing AppImages.
I also wanted to create an AUR-like platform on Fedora.
(yes, cURL and wget arent unusable, but this is intended at a beginner *nix user.)

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
Save the file as a fruit (*packagename*.fruit.json) and publish it to the registry with `appl pubpkg *directory name*` 

