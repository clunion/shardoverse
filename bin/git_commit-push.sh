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
if ! cargo test; then
    echo cargo test returned error: $?
    exit
else
    echo tests OK
fi 

#------------------------------------- 
if ! cargo doc; then
    echo cargo doc returned error: $?
    exit
else
    echo docs OK
fi 

#------------------------------------- 
if ! git commit -a; then
    echo git commit -a returned error: $?
    exit
else
    echo commited OK
fi 

#------------------------------------- 
if ! git push; then
    echo git push returned error: $?
    exit
else
    echo pushed OK
fi 

