<h1 align="center">Few Word Do Trick (fwdt)</h1>
<p align="center">Few Word Do Trick (fwdt) is a cross-platform general purpose fast logger that supports templated designs</p>

---

 <h1 align="center">
    <img src="img/fwdt_logo_144.png" alt="alternate text">
 </h1>

 <p align="center">
    <img src="https://media.giphy.com/media/DMNPDvtGTD9WLK2Xxa/giphy.gif" alt="alternate text">
 </p>

```txt
> cat test/data/ham_log/data.txt 
mycall wq8R
operator wq8R


# this is a comment
date 2022-08-08
40m cw
2307 qr3e 7.2230 599 599
11 kf1rx 7.0560 
13 kn6h
7 ae0bc
20m cw
2307 qr3e 14.2230 599 599
11 kf1rx 14.0560 
13 kn6h
7 ae0bc
date 2022-08-10
40m cw
2307 qr3e 7.2230 599 599
11 kf1rx 7.0560 
13 kn6h
7 ae0bc
```

```txt
> ./target/debug/fwdt test/data/ham_log/data.txt test/data/ham_log/template.toml | tv -g 5

        tv dim: 8 x 9
        call  date       freq    group mycall operator received sent time 
     1  qr3e  2022-08-08  7.2230 cw    wq8R   wq8R     599      599  2307 
     2  kn6h  2022-08-08  7.2230 cw    wq8R   wq8R     599      599  13   
     3  ae0bc 2022-08-08  7.2230 cw    wq8R   wq8R     599      599  7    
     4  ae0bc 2022-08-08  7.2230 cw    wq8R   wq8R     599      599  7    
     5  qr3e  2022-08-08 14.223  cw    wq8R   wq8R     599      599  2307 
     6  kn6h  2022-08-08 14.223  cw    wq8R   wq8R     599      599  13   
     7  ae0bc 2022-08-08 14.223  cw    wq8R   wq8R     599      599  7    
     8  ae0bc 2022-08-10 14.223  cw    wq8R   wq8R     599      599  7    

```

# things to say

What makes a good fast logger.

The thing that makes manual logging slow is, in part, proportional to the number of characters that must be entered. If a logger can generate the correct final output without needing as many characters then it accomplished its task.

In the above example there result desired has 440 characters. The input required was 279 (excluding the template text). This saves 161 charcters! 

```
> cat test/data/ham_log/data.txt | wc -m
279
> ./target/debug/fwdt test/data/ham_log/data.txt test/data/ham_log/template.toml | wc -m
440
```

# tufte of text

When tufte came up with his design of a boxplot he got there by erasing the unnecesary parts of the box plot. In a similar fashion `fwdt` may be thought of as erasing unnessary text.

![th](img/boxplot.png)

Here is an example of removing the unneccesary.

```
        # table 1 - desired output
        date       time mycall operator band mode call  freq    sent received 
        2022-08-08 2307 wq8R   wq8R     40m  cw   qr3e   7.2230 599  559      
        2022-08-08 2311 wq8R   wq8R     40m  cw   kf1rx  7.2230 599  559      
        2022-08-08 2313 wq8R   wq8R     40m  cw   kn6h   7.2230 599  559      
        2022-08-08 2317 wq8R   wq8R     40m  cw   ae0bc  7.2230 533  559      
        2022-08-08 2321 wq8R   wq8R     20m  cw   qr3g   14.223 599  555         
        2022-08-08 2322 wq8R   wq8R     20m  cw   kf1rz  14.223 599  599         
        2022-08-08 2322 wq8R   wq8R     20m  cw   kn0h   14.223 599  599         
        2022-08-08 2324 wq8R   wq8R     20m  cw   ae0gc  14.223 599  599    
```

```
	# table 2 - erase unnecesary characters
        date       time mycall operator band mode call  freq    sent received 
        2022-08-08 2307 wq8R   wq8R     40m  cw   qr3e   7.2230 599  559      
                   11                             kf1rx              
                   13                             kn6h                
                   7                              ae0bc          33     
                   2321                 20m       qr3e   14.223  99   55   
                   2                              kf1rx               99   
                   2                              kn6h               
                   4                              ae0bc              
```

```
        # table 3 - types of columns
        |time_stamp---| constants-----| goups---| observations----------------------|
                                                | full_replace |right_replace-------|
        date       time mycall operator band mode call  	freq    sent received 
        2022-08-08 2307 wq8R   wq8R     40m  cw   qr3e  	 7.2230 599  559      
                   11                             kf1rx 	             
                   13                             kn6h  	              
                   7                              ae0bc 	         33     
                   2321                 20m       qr3e  	 14.223       55   
                   2                              kf1rx 	              99   
                   2                              kn6h  	             
                   4                              ae0bc 	             

```

```
        # file 1 - example log which generates table 1
        mycall wq8r
        operator wq8r
        date 2022-08-08
        40m cw
        2307 qr3e  7.2230 599 599
        11   kf1rx
        13   kn6h 
        7    ae0bc 0 33 
	    20m
        2321 qr3e  14.223 99 55
        2    kf1rx 3 9 9
        2    kn6h 
        4    ae0bc
```

```
	# file 2 - grammer of log file 1
        <constant_key> <constant value>
        <constant_key> <constant value>
        date <date>
        <group_value> <group_value>
        <time_utc_hhmm> <obs_full_replace> <obs_right_replace> <obs_right_replace> <obs_right_replace>
        <time_utc_hhmm> <obs_full_replace> <obs_right_replace> <obs_right_replace> <obs_right_replace>
        <time_utc_hhmm> <obs_full_replace> <obs_right_replace> <obs_right_replace> <obs_right_replace>
        <time_utc_hhmm> <obs_full_replace> <obs_right_replace> <obs_right_replace> <obs_right_replace>
        <group_value> 
        <time_utc_hhmm> <obs_full_replace> <obs_right_replace> <obs_right_replace> <obs_right_replace>
        <time_utc_hhmm> <obs_full_replace> <obs_right_replace> <obs_right_replace> <obs_right_replace>
        <time_utc_hhmm> <obs_full_replace> <obs_right_replace> <obs_right_replace> <obs_right_replace>
        <time_utc_hhmm> <obs_full_replace> <obs_right_replace> <obs_right_replace> <obs_right_replace>
```

The above is rearranged into a tree-like structure.

# Line Entry Types

`header`: The header represents constants that a pulled down through the final csv. In the above example `mycall` is a header.

`group`: A group is like a header, but is replaced on enter

`observation`: Using tidy data principles, each row is an observation. In `fwdt` observations are the entries
that should be allowed to change from line to line. `call` and `time` are observations. `receiceved` and `sent` and `frequency` are also observations, but did not need to be changed often. Another property of observations is that they have the option of completly replacement or partial replacement. The `time` column is a good example of this. The first row shows `2307` the second row is `13` which is filled to `2313`. This is a 
*right-to-left* replacement.

# install
```sh
git clone <this>
cd fwdt
cargo build
```
# Example Logs

There are iphone apps that specialize in each of the following tasks. `fwdt` makes it easy to log personal data and host how you would like.

Important personal data is relatively easy to log with `fwdt`. 

Along with personal data `fwdt` may be a useful tool for observational experiment tracking. 

# Weight Lifting Log

```

```

# Weight Log

```

```


# Calorie Intake Log

```

```

# Reading Log

```

```

# Saving Rate Log

```

```

# Observational Experiment

Not all data is equally character compressible. In this example every observation is a full replace. For this reason the only thing that can be pulled 
out is the date and group. Still, this was a randomized experiment so the groups alternated frequently.

```
helicopter,wing_len,student,time
date 2017-07-17
A
1 5.5 5.29
2 6 4.54
B
3 6 3.78
A
4 4.5 4.57
5 5 3.43
6 5.5 4.54
B
7 6 5.9
8 4.5 3.49
A
9 4.5 4.08
10 5 3.98
B
11 5.5 5.52
12 5 3.2
A
13 4.5 3.96
14 5 4.4
B
15 6 3.96
16 5.5 5.56
```

https://r-data.pmagunia.com/dataset/r-dataset-package-datasets-toothgrowth

```sh
./target/debug/fwdt test/data/ham_log/data.txt test/data/ham_log/template.toml 
```
