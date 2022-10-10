 <h1 align="center">
    <img src="img/fwdt_logo_144.png" alt="alternate text">
 </h1>

<h1 align="center">Few Word Do Trick (fwdt)</h1>



<p align="center">Few Word Do Trick (fwdt) is a cross-platform general purpose fast logger that supports templated designs</p>


 <p align="center">
    <img src="https://media.giphy.com/media/DMNPDvtGTD9WLK2Xxa/giphy.gif" alt="alternate text">
 </p>

---
<h2 align="center">Two Laws Of Human Data Entry</h2>
<h3 align="center"><strong>First Law: </strong>To prevent data entry errors, never enter data.</h3>
<h3 align="center"><strong>Second Law: </strong>When First Law is not possible only enter changes.</h3>

--- 

 <h1 align="center">
    <img src="img/fwdt_example.jpg" alt="alternate text">
 </h1>


## Personal Anecdote

Data entry by humans is error prone. I know this from first hand experience. Though I do not have the evidence in front of me I assume that the average person can't generate flawless entries especially as the number of those entries increase to larger numbers.

Recently I stumbled across a ham radio data logger [fle](https://df3cb.com/fle/). It is a domain specific language (DSL) which allows data entry with minimal repetition. This command line utility is similar in spirit, but does not invent its own DSL. It uses incomplete csv files as the data input and outputs complete csv files.

## Example

Assume a user has the final dataset in mind. Using `wc -m` The character count is `453`.

```
        date       group mycall operator received sent freq    time call   
     1  2022-08-08 cw    wq8R   wq8R     599      599   7.2230 1107 qr3e   
     2  2022-08-08 cw    wq8R   wq8R     599      599   7.2230 1113 kn6h   
     3  2022-08-08 cw    wq8R   wq8R     599      599   7.2230 1127 ae0bc  
     4  2022-08-08 cw    wq8R   wq8R     599      599   7.2230 1207 ae4bc  
     5  2022-08-08 cw    wq8R   wq8R     599      599  14.223  1207 qr3e   
     6  2022-08-08 cw    wq8R   wq8R     599      599  14.223  1213 kn6h   
     7  2022-08-08 cw    wq8R   wq8R     599      599  14.223  1217 a2rat  
     8  2022-08-08 cw    wq8R   wq8R     599      599  14.223  1217 ko7rqq
```

If a user erases the repetitive data the character count may be reduced to `187`.
The only data the user must enter is shown below. I will refer to these incomplete
csv files as <em>fast logged</em> (fl) files. 

```
date,group,mycall,operator,received,sent,freq,time,call 
2022-08-08,cw,wq8R,wq8R,599,599,7.2230,1107,qr3e
1113,kn6h
1127,ae0bc
1207,ae4bc
14.223,1207,qr3e
1213,kn6h
1217,a8rat
1217,ko7rqq
```

If this were mapped to the original formatting it would be easier to see what was erased. 

```
        date       group mycall operator received sent freq  time call   
     1  2022-08-08 cw    wq8R   wq8R     599      599   7.22 1107 qr3e   
     2  NA         NA    NA     NA       NA       NA   NA    1113 kn6h   
     3  NA         NA    NA     NA       NA       NA   NA    1127 ae0bc  
     4  NA         NA    NA     NA       NA       NA   NA    1207 ae4bc  
     5  NA         NA    NA     NA       NA       NA   14.2  1207 qr3e   
     6  NA         NA    NA     NA       NA       NA   NA    1213 kn6h   
     7  NA         NA    NA     NA       NA       NA   NA    1217 a8rat  
     8  NA         NA    NA     NA       NA       NA   NA    1217 ko7rqq
```

## Usage

The command used to go from a fl file to a csv file, which is comma separated, is:

```
fwdt -s, test/data/radio_log_small.csv
```
The output is 

```
date,group,mycall,operator,received,sent,freq,time,call
2022-08-08,cw,wq8R,wq8R,599,599,7.2230,1107,qr3e
2022-08-08,cw,wq8R,wq8R,599,599,7.2230,1113,kn6h
2022-08-08,cw,wq8R,wq8R,599,599,7.2230,1127,ae0bc
2022-08-08,cw,wq8R,wq8R,599,599,7.2230,1207,ae4bc
2022-08-08,cw,wq8R,wq8R,599,599,14.223,1207,qr3e
2022-08-08,cw,wq8R,wq8R,599,599,14.223,1213,kn6h
2022-08-08,cw,wq8R,wq8R,599,599,14.223,1217,a8rat
2022-08-08,cw,wq8R,wq8R,599,599,14.223,1217,ko7rqq
```

## How to make a valid fast log file

The rules are very simple.

1. The first two lines (column headers and the first line of data) must be complete.
2. All subsequent lines are replaced by the right most column. (Put columns that change the most frequently to the right).


## Install

```
cargo install fwdt
```

## Scorch

> How do I know when to use fl files over csv?

Scorch is defined as `[1 - (fl_word_count/csv_word_count)]`. Using the example at the top of the page `fl_word_count=187` and `csv_word_count=453`, so `1 - (187/453) ~ 59%`. This is an uncommonly high scorch. It 
represents the percent of text saved by using a fl file over a complete csv. Even if scorch is smaller, 5%, 
that is still 5% that will not have to entered by hand thus will be error free. 

## Help

```
fwdt 0.1.0
📝🔥 Few Word Do Trick (fwdt) is a fast data logger 📝🔥

    Example Usage:
    fwdt -s, data.csv

USAGE:
    fwdt [FLAGS] [OPTIONS] [FILE]

FLAGS:
    -d, --debug-mode    Print object details to make it easier for the maintainer to find and resolve bugs.
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -s, --delimiter <delimiter>    The delimiter separating the columns. Example #1 `fwdt -s ' '
                                   test/data/power_lift.csv`. Example #2 `fwdt -s, test/data/radio_log_small.csv`

ARGS:
    <FILE>    Data file to process
```
