#!/bin/bash

IPFS_PATH=~/.ipfs ipfs add $1

OUTPUT=$(md5sum $1)
echo " "
echo "CHECKSUM: "
echo "${OUTPUT}"