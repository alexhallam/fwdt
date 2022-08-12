# fwdt

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

# install
```sh
git clone <this>
cd fwdt
cargo build
```
# example
```sh
./target/debug/fwdt test/data/ham_log/data.txt test/data/ham_log/template.toml 
```
