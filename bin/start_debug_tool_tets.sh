#!/bin/bash
# This script starts the debug tool (gdb) 
# with the current UNIT-TEST-EXECUTABLE
# It is meant for debugging the code of the unit tests

# use cargo test to find the correct executable:
tmpstr=`cargo test 2>&1 | grep -Fi "Running target\debug\deps\shard"`
exe4tests=${tmpstr:12:100}

echo gdb $exe4tests
     gdb $exe4tests
