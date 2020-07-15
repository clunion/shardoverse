### script to not forget to test and doc before a push to Git repository:

#------------------------------------- 
#cargo fmt <-- not yet, preferred format is still unstable
#if ! cargo fmt; then
#    echo cargo fmt returned error: $?
#    exit
#else
#    echo format OK
#fi 

#------------------------------------- 
printf "##>>>>> cargo test:\n"
if ! cargo test; then
    printf "##<<<<< cargo test returned error: " $? "\n\n"
    exit
else
    printf "##<<<<< cargo build: OK\n"
fi 

#------------------------------------- 
printf "##>>>>> cargo doc:\n"
if ! cargo doc; then
    printf "##<<<<< cargo doc returned error: " $? "\n\n"
    exit
else
    printf "##<<<<< cargo docs: OK\n"
    echo docs OK
fi 

#------------------------------------- 
printf "##>>>>> git commit:\n"
if ! git commit -a; then
    printf "##<<<<< git commit -a returned error: " $? "\n\n"
    exit
else
    printf "##<<<<< git commit: OK\n"
fi 

#------------------------------------- 
printf "##>>>>> git push:\n"
if ! git push; then
    printf "##<<<<< git push returned error: " $? "\n\n"
    exit
else
    printf "##<<<<< git push: OK\n"
fi 

