#!/bin/bash
# This script starts the debug tool (gdb) 
# with the current executable of the TARGET PROGRAM.
# It is meant for debugging the code of the tests

# preset the extension of executables to nothing
exeext=""

if [[ "$OSTYPE" == "msys" ]]; then
        # MSys2+MinGW, shell and GNU utilities compiled for Windows:
        echo "msys"
        exeext=".exe"
elif [[ "$OSTYPE" == "win32" ]]; then
        # not sure this can occur:
        echo "win32"
        exeext=".exe"
else
        # Unknown.
        echo "unknown"
fi

echo gdb target/debug/shardoverse$exeext
     gdb target/debug/shardoverse$exeext
