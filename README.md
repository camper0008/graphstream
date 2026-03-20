# graphstream

take a stdin stream and make a graph

## install

use `cargo install --git`

## input

each line is a "step" on the time scale (x axis), with a value (y axis). semicolons differentiate between different data sets. i.e. `<humidity>;<temperature>`

example input: 

```
10;20.3;47
57.2;-50.3;22
```

will make a graph with 3 lines and 2 steps on the x axis

## colors

you can specify colors for the different lines in the argument. it is in the format of `<color>;<color>`. colors are specified as `r,g,b` or `#abcdef` or `#abc`

i.e. `graphstream 255,0,0;255,255,0;0,0,255` will make the aforementioned 3 lines red, orange and blue

if no color is specified, defaults to red
