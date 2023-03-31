// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract DSchain {

    //owner of contract
    address private owner;

    /*
    -user struct
    */
    struct User {
        address addr;
        bool isDataOwner;
        address[] subscribers;
        bool notification;
    }

    /*
    -user network
    -<address, address,...>
    */
    mapping(address => User) private userNetwork;

    /*
    -DataOwners dynamic array
    -<DO1, DO2, DO3, ...>
    */
    //address[] private dataOwners;

    /*
    -struct for files
    */
    struct File {
        string hash;
        uint timestamp;
        address dataowner;
    }

    mapping(address => File[]) private userFiles;

    /*
    -constructor
    -owner starts as user and data owner
    */
    constructor() {
        owner = address(msg.sender);
        //dataOwners.push(owner);
        User memory user;
        user.addr = owner;
        user.isDataOwner = true;
        user.notification = false;
        userNetwork[owner] = user;
    }

    /*
    -return the owner of the contract
    */
    function getOwner() public view returns(address) {
        return owner;
    }

    /*
    -user join network
    -user network:       usernetwork = <..., useraddr>;
    -subscriber mapping: useraddr => < >;
    -notif mapping:      useraddr => false;
    */
    function joinNetwork() public {
        //not already in network
        //require(msg.sender == tx.origin);
        require(userNetwork[msg.sender].addr == address(0x0));
        
        //create new user
        User memory newUser;
        newUser.addr = msg.sender;
        newUser.isDataOwner = false;
        newUser.notification = false;

        //add to mapping
        userNetwork[msg.sender] = newUser;
    }

    /*
    -add DataOwner
    -require user
    -only other DO's can add
    */
    function addDataOwner(address useraddr) public {
        //require(msg.sender == tx.origin);
        require(userNetwork[msg.sender].isDataOwner == true, "YOU MUST BE A DATA OWNER!");
        require(userNetwork[useraddr].addr != address(0x0), "NOT A CURRENT USER");
        
        //make user a data owner
        userNetwork[useraddr].isDataOwner = true;
    }

    /*
    -is user data owner
    */
    function isDataOwner() public view returns (bool) {
        require(userNetwork[msg.sender].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[msg.sender].isDataOwner;
    }

    /*
    -add subscriber
    -require user
    -only publisher can add subscriber
    */
    function addSubscriber(address subaddr) public {
        //require(msg.sender == tx.origin);
        require(userNetwork[msg.sender].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        require(userNetwork[subaddr].addr != address(0x0), "SUBSCRIBER NOT A CURRENT USER");

        //add subscriber
        userNetwork[msg.sender].subscribers.push(subaddr);
    }

    /*
    -get subscribers list
    */
    function getSubscribers() public view returns (address[] memory) {
        require(userNetwork[msg.sender].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[msg.sender].subscribers;
    }

    /*
    -create file struct
    */
    function createFile(string memory hash, address publisher) private view returns (File memory){
        File memory newFile;
        newFile.hash = hash;
        newFile.dataowner = publisher;
        newFile.timestamp = block.timestamp;
        return newFile;
    }

    /*
    -upload hash
    -require user
    -get subscriber addresses => set those address notification to true
    -get timestamp
    -update subscriber file hash timestamp tuples
    */
    function uploadHash(string memory hash) public {
        //require(msg.sender == tx.origin);
        require(userNetwork[msg.sender].addr != address(0x0), "YOU ARE NOT A CURRENT USER");

        //for every subscriber add hash to their files and give notification
        File memory newfile = createFile(hash, msg.sender);
        for(uint i = 0; i < userNetwork[msg.sender].subscribers.length; i++){
            //get subscriber
            address sub = userNetwork[msg.sender].subscribers[i];
            
            //add hash
            userFiles[sub].push(newfile);

            //update notification
            userNetwork[sub].notification = true;
        }

    }

    /*
    -get hash(es) for user
    -require user
    -return dynamic array from user file hash mapping
    -set notification bool for user to false
    */
    function getHash() public returns (File[] memory){
        require(userNetwork[msg.sender].addr != address(0x0), "YOU ARE NOT A CURRENT USER");

        //set notification to false
        userNetwork[msg.sender].notification = false;

        //get file array
        return userFiles[msg.sender];
        
    }

    /*
    -get notification status
    */
    function hasNotification() public view returns (bool) {
        require(userNetwork[msg.sender].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[msg.sender].notification;
    }

    //TESTING FUNCTIONS

    //returns msg.sender
    function msgsender() public view returns (address) {
        return msg.sender;
    }

    //returns tx.origin
    function txorigin() public view returns (address) {
        return tx.origin;
    }
}

/*
STRUCTS:
struct User {
        address addr;
        bool isDataOwner;
        address[] subscribers;
        bool notification;
}

struct File {
        string hash;
        uint timestamp;
        address dataowner;
}

VARIABLES:
address private owner;
mapping(address => User) private userNetwork;
mapping(address => File[]) private userFiles;

FUNCTIONS:
function getOwner() public view returns(address);
function joinNetwork(address useraddr) public;
function addDataOwner(address useraddr) public;
function isDataOwner() public view returns (bool);
function addSubscriber(address subaddr) public;
function getSubscribers() public view returns (address[] memory);
function createFile(string memory hash, address publisher) private view returns (File memory);
function uploadHash(string memory hash) public;
function getHash() public returns (File[] memory);
function hasNotification() public view returns (bool);
function msgsender() public view returns (address);
function txorigin() public view returns (address);

*might have to change msg.sender to playerAddress or something bc msg.sender might not be who I want it to be
*added testing functions in case to see
*/
