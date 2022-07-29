# von

`von` is a utility for running interpreters on scripts and data located within the same file.

A file formatted to use von takes the following form:

```
interpreter
--
source code to be interpreted
--
input for the program
```

A common use case might be within the vim editor, writing a program to
generate some repetitive code quickly. For example:

```
python3
--
for i in range(20):
    print(f"A[{i}] = i;")
--
```


## Planned flags

- `-k, --keep` - outputs the inputed von file followed by --\n followed by the output of the program  
- `-i, --inplace` - outputs the inputed von file with the input section replaced with the output of the program  
- `-s, --save` - saves the run von file to ~/.von
- `-r, --run` - runs a specified von file
