# Advent of Code Solutions🎄
No puzzle inputs have been provided in this repo per [Eric Wastl's request](https://adventofcode.com/2019/about#faq_copying]).

All solutions can be checked against an input/output directory with the structure:
```
.
├── 2019
│   ├── 01
│   │   ├── input.txt
│   │   └── output.txt
│   ├── 02
│   │   ├── input.txt
│   │   └── output.txt
│   ├── 03
│   │   ├── input.txt
│   │   └── output.txt
└── 2023
    ├── 01
    │   ├── input.txt
    │   └── output.txt
    ├── 02
    │   ├── input.txt
    │   └── output.txt
    └── 03
        ├── input.txt
        └── output.txt
```

And can be run using the provided justfile with the command:
```bash
just run ~/path/to/inputs
```

An optional year can be provided, and if a year is provided, an optional day can be provided as well:
```bash
just run ~/path/to/inputs/ 2019
just run ~/path/to/inputs/ 2019 5
```

## Intcode Interpreter Debug Screen
This is a screenshot of a (somewhat) interactive debug screen for the intcode computer from 2019.
![image](https://github.com/user-attachments/assets/35fbe590-24b9-41b3-8ba6-c2ee291cdd0a)
