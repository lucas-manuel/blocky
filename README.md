# LBL

Simple tool to generate code comment blocks like below:

```
/**************************/
/*** External Functions ***/
/**************************/
```

Clone the repo and type `cargo run`. Input the desired word and the program will output the comment block.

To install to command line: `cargo install lbl`.

To run:

```
➜ lbl singleWord
/******************/
/*** singleWord ***/
/******************/

➜ lbl "Multiple Words"
/**********************/
/*** Multiple Words ***/
/**********************/
````
