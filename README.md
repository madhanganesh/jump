# Jump Tool

## What is this?
Jump tool helps you to change your directories easily. Jump, in essense is same as using `cd <dir>` to change ditrectory and its usage is similar too.

```
jump ~/temp/src/learnings/rustbook/
```

but when you use jump it something more:
1. It keeps this folder in a most-recetly-used cache file 
2. Next time you can simply pass part pf the path as in `jump rustbook`. This will search the MRU cache and jump to the path matching the part
3. In case there are multiple paths matching the part it lists all of them and then you can navigate manually by picking one
4. If you simply run `jump` witout any parameters it lsists all the top MRU files

## Usage
1. Compile the binary and keep in your path. So the rust binary `jumpbin` should be in your path
2. In order for the shell to cd to the path executable is printing, please following macro in you shell start command. For eg in MacOS add following command in ~/.zshrc or ~/.bashrc
```
jump() {
    local target=$(jumpbin "$@")
    if [ -n "$target" ] && [ -d "$target" ]; then
        pushd "$target" > /dev/null || echo "Failed to change directory"
    fi
}
```

## Variables
1. The MRU file by default will be ~/jump.txt but you can change by seeting an env variable `DEFAULT_CACHE_FILE`
2. The MRU cache size is 10

# Feedback
Any feedback please drop a mail tio madhanganesh@gmail.com
