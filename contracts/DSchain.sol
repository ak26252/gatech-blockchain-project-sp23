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
    function joinNetwork(address inputaddr) public {
        //not already in network
        //require(msg.sender == tx.origin);
        require(userNetwork[inputaddr].addr == address(0x0));
        
        //create new user
        User memory newUser;
        newUser.addr = inputaddr;
        newUser.isDataOwner = false;
        newUser.notification = false;

        //add to mapping
        userNetwork[inputaddr] = newUser;
    }

    /*
    -add DataOwner
    -require user
    -only other DO's can add
    */
    function addDataOwner(address inputaddr, address useraddr) public {
        //require(msg.sender == tx.origin);
        require(userNetwork[inputaddr].isDataOwner == true, "YOU MUST BE A DATA OWNER!");
        require(userNetwork[useraddr].addr != address(0x0), "NOT A CURRENT USER");
        
        //make user a data owner
        userNetwork[useraddr].isDataOwner = true;
    }

    /*
    -is user data owner
    */
    function isDataOwner(address inputaddr) public view returns (bool) {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[inputaddr].isDataOwner;
    }

    /*
    -add subscriber
    -require user
    -only publisher can add subscriber
    */
    function addSubscriber(address inputaddr, address subaddr) public {
        //require(msg.sender == tx.origin);
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        require(userNetwork[subaddr].addr != address(0x0), "SUBSCRIBER NOT A CURRENT USER");

        //add subscriber
        userNetwork[inputaddr].subscribers.push(subaddr);
    }

    /*
    -find index of subscriber
    */
    function findSubscriber(address inputaddr, address subaddr) private view returns (uint){
        for(uint i = 0; i < userNetwork[inputaddr].subscribers.length; i++){
            if(userNetwork[inputaddr].subscribers[i] == subaddr){
                return i;
            }
        }
        //proper error handling later
        return 0;
    }

    /*
    -remove a subscriber
    */
    function removeSubscriber(address inputaddr, address subaddr) public {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        require(userNetwork[subaddr].addr != address(0x0), "SUBSCRIBER NOT A CURRENT USER");

        uint index = findSubscriber(inputaddr, subaddr);

        //replace with last element
        userNetwork[inputaddr].subscribers[index] = userNetwork[inputaddr].subscribers[userNetwork[inputaddr].subscribers.length-1];
        userNetwork[inputaddr].subscribers.pop();
        
    }

    /*
    -get subscribers list
    */
    function getSubscribers(address inputaddr) public view returns (address[] memory) {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[inputaddr].subscribers;
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
    function uploadHash(string memory hash, address inputaddr) public {
        //require(msg.sender == tx.origin);
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");

        //for every subscriber add hash to their files and give notification
        File memory newfile = createFile(hash, inputaddr);
        for(uint i = 0; i < userNetwork[inputaddr].subscribers.length; i++){
            //get subscriber
            address sub = userNetwork[inputaddr].subscribers[i];
            
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
    function getHash(address inputaddr) public view returns (File[] memory){
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");

        //get file array
        return userFiles[inputaddr];
        
    }

    /*
    -clears notification status
    */
    function clearNotification(address inputaddr) public {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");

        //set notification to false
        userNetwork[inputaddr].notification = false;
    }

    /*
    -get notification status
    */
    function hasNotification(address inputaddr) public view returns (bool) {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[inputaddr].notification;
    }

    function isUser(address inputaddr) public view returns (bool) {
        return userNetwork[inputaddr].addr != address(0x0);
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