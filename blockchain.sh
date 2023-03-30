#!/bin/bash

#restart chain
rm -rf eth/data/miner/geth eth/data/node1/geth eth/data/node2/geth
geth --datadir eth/data/node1/ init eth/setup/genesis.json
geth --datadir eth/data/node2/ init eth/setup/genesis.json
geth --datadir eth/data/miner/ init eth/setup/genesis.json
sleep 5

#bootnode
gnome-terminal --tab --working-directory=/home/kunho/eth/ --command "bootnode -nodekey setup/boot.key -addr :30300"
sleep 15

#node1
gnome-terminal --tab --working-directory=/home/kunho/eth/ --command "geth --datadir data/node1 --ipcdisable --port 30301 --bootnodes 'enode://71c4d1df0c9b796fd489feafa883cd63e30f5da0066b80f4bbca0ae4a5144a63a9ad2e4263b98a724b12676fe1b30d7b305403c0d6b2bb0bce193d93ce9a3c66@127.0.0.1:0?discport=30300' --networkid 107440 --http --http.addr 'localhost' --http.port 8101 --http.api 'admin,debug,eth,miner,net,personal,txpool,web3' --allow-insecure-unlock"
sleep 15

#node2
gnome-terminal --tab --working-directory=/home/kunho/eth/ --command "geth --datadir data/node2 --ipcdisable --port 30302 --bootnodes 'enode://71c4d1df0c9b796fd489feafa883cd63e30f5da0066b80f4bbca0ae4a5144a63a9ad2e4263b98a724b12676fe1b30d7b305403c0d6b2bb0bce193d93ce9a3c66@127.0.0.1:0?discport=30300' --networkid 107440 --authrpc.port 8547 --http --http.addr 'localhost' --http.port 8102 --http.api 'admin,debug,eth,miner,net,personal,txpool,web3' --allow-insecure-unlock"
sleep 15

#miner
gnome-terminal --tab --working-directory=/home/kunho/eth/ --command "geth --datadir data/miner --ipcdisable --port 30303 --networkid 107440 --bootnodes 'enode://71c4d1df0c9b796fd489feafa883cd63e30f5da0066b80f4bbca0ae4a5144a63a9ad2e4263b98a724b12676fe1b30d7b305403c0d6b2bb0bce193d93ce9a3c66@127.0.0.1:0?discport=30300' --authrpc.port 8548 --mine --miner.threads=1 --miner.etherbase=0x7D01a594Ee79618d5DB9dB3b272fbf03F9Ebda77"
sleep 15

#geth attach
gnome-terminal --working-directory=/home/kunho/eth/ --command "geth attach http://localhost:8101"
gnome-terminal --working-directory=/home/kunho/eth/ --command "geth attach http://localhost:8102"