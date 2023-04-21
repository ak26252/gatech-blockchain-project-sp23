#!/bin/bash

ipfs get $1 -o=$2

OUTPUT=$(md5sum $2)
echo " "
echo "CHECKSUM: "
echo "${OUTPUT}"