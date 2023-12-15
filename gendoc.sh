cargo doc --no-deps
rm ./target/doc/crates.js
echo 'window.ALL_CRATES = ["appl"];' > ./target/doc/crates.js
firefox target/doc/appl/index.html