### shortcuts for Rust's cargo-Build-processes:
cargo clippy            2>&1 | tee    log/build_debug.log
cargo build             2>&1 | tee -a log/build_debug.log

filename=.\\target\\debug\\shardoverse.exe
file_size_byte=0
file_size_kb=0

if [ -e $filename ] 
then
    file_size_byte=`du -b "$filename" | cut -f1`
    file_size_kb=`du -k "$filename" | cut -f1`
fi

# file_size_hr=`du -h "$filename" | cut -f1`  # -h is a non POSIX GNU-extension
printf "\n    Size of target binary: %s Byte (%s KB)\n" $file_size_byte $file_size_kb
