### shortcuts for Rust's cargo-Build-processes:

printf "cargo clippy lint:\n"
#cargo clippy --release  2>&1 | tee    log/build_release.log
if ! cargo clippy --release ; then
    echo cargo clippy returned error: $?
    exit
else
    echo cargo clippy lint: OK
fi 

printf "\n\ncargo build:\n"
#cargo build  --release  2>&1 | tee -a log/build_release.log
if ! cargo build --release ; then
    echo cargo build returned error: $?
    exit
else
    echo cargo build: OK
fi 

filename=.\\target\\release\\shardoverse.exe
file_size_byte=0
file_size_kb=0

if [ -e $filename ] 
then
    file_size_byte=`du -b "$filename" | cut -f1`
    file_size_kb=`du -k "$filename" | cut -f1`
fi

# file_size_hr=`du -h "$filename" | cut -f1`  # -h is a non POSIX GNU-extension
printf "\n    Size of target binary: %s Byte (%s KB)\n" $file_size_byte $file_size_kb
