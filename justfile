build-watch:
    cargo watch -s "just test"


test: 
    cargo test

clean:
    git clean -fdx