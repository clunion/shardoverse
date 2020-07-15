### shortcuts for Rust's cargo-Build-processes:

printf "##>>>>> cargo clippy lint:\n"
if ! cargo clippy --release ; then
    printf "cargo clippy returned error: " $? "\n\n"
    exit
else
    printf "##<<<<< cargo clippy lint: OK\n\n"
fi 

printf "##>>>>> cargo build:\n"
#cargo build  --release  2>&1 | tee -a log/build_release.log
if ! cargo build --release ; then
    printf "##<<<<< cargo build returned error: "$? "\n"
    exit
else
    printf "##<<<<< cargo build: OK \n"
fi 

filename=.\\target\\release\\shardoverse.exe
file_size_byte=0
file_size_kb=0

if [ -e $filename ] 
then
    file_size_byte=`du -b "$filename" | cut -f1`
    file_size_kb=`du -k "$filename" | cut -f1`
fi

printf "    Size of target binary: %s Byte (%s KB)\n" $file_size_byte $file_size_kb
