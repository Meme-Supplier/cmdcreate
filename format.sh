cd ./src
echo Formatting main...
rustfmt main.rs
rustfmt utils.rs
echo Formatting other...
rustfmt ./cmds/*
echo Done
