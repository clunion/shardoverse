### shortcut for Rust's cargo unit-test of the target:

#cargo fmt <-- not yet, preferred format is still unstable

if ! cargo test; then
    echo cargo test returned error: $?
    exit
else
    echo test OK
fi 


if ! cargo doc; then
    echo cargo doc returned error: $?
    exit
else
    echo docs OK
fi 


if ! git commit -a; then
    echo git commit -a returned error: $?
    exit
else
    echo git commit: OK
fi 

# 
if ! git push; then
    echo git push returned error: $?
    exit
else
    echo docs OK
fi 

