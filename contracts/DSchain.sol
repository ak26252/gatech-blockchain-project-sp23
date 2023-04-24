// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/Strings.sol";

contract DSchain {

    //owner of contract
    address private owner;

    address private sender;
    address private origin;

    /*
    -user struct
    */
    struct User {
        address addr;
        bool isDataOwner;
        string pubKey;
        address[] subscribers;
        address[] publishers;
        bool notification;
        string[] notification_contents;
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
        string checksum;
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
        require(msg.sender == inputaddr, "msg.sender != inputaddr");
        
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
        require(msg.sender == inputaddr, "msg.sender != inputaddr");
        
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
        require(msg.sender == inputaddr, "msg.sender != inputaddr");

        //make sure subscriber not a current sub already
        if(findSubscriber(inputaddr, subaddr) == -1){
            //add subscriber
            userNetwork[inputaddr].subscribers.push(subaddr);

            //notification
            string memory addr = Strings.toHexString(uint256(uint160(inputaddr)),20);
            userNetwork[subaddr].notification_contents.push(string.concat(addr," has added you as a subscriber"));
        }
    }

    /*
    -find index of subscriber
    */
    function findSubscriber(address inputaddr, address subaddr) private view returns (int){
        for(uint i = 0; i < userNetwork[inputaddr].subscribers.length; i++){
            if(userNetwork[inputaddr].subscribers[i] == subaddr){
                return int(i);
            }
        }
        //proper error handling later
        return -1;
    }

    /*
    -remove a subscriber
    */
    function removeSubscriber(address inputaddr, address subaddr) public returns (bool){
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        require(userNetwork[subaddr].addr != address(0x0), "SUBSCRIBER NOT A CURRENT USER");
        require(msg.sender == inputaddr, "msg.sender != inputaddr");

        bool err = true;

        int index = findSubscriber(inputaddr, subaddr);

        if(index != -1){
            //replace with last element
            userNetwork[inputaddr].subscribers[uint(index)] = userNetwork[inputaddr].subscribers[userNetwork[inputaddr].subscribers.length-1];
            userNetwork[inputaddr].subscribers.pop();
            string memory addr = Strings.toHexString(uint256(uint160(inputaddr)),20);
            userNetwork[subaddr].notification_contents.push(string.concat(addr," has removed you as a subscriber"));
        }
        else{
            err = false;
        }

        return err;
    }

    /*
    -get subscribers list
    */
    function getSubscribers(address inputaddr) public view returns (address[] memory) {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[inputaddr].subscribers;
    }

     /*
    -add publisher
    -require user
    -only publisher can add publisher
    */
    function addPublisher(address inputaddr, address pubaddr) public {
        //require(msg.sender == tx.origin);
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        require(userNetwork[pubaddr].addr != address(0x0), "PUBLISHER NOT A CURRENT USER");
        require(msg.sender == inputaddr, "msg.sender != inputaddr");

        if(findPublisher(inputaddr, pubaddr) == -1){
            //add publisher
            userNetwork[inputaddr].publishers.push(pubaddr);

            //notification
            string memory addr = Strings.toHexString(uint256(uint160(inputaddr)),20);
            userNetwork[pubaddr].notification_contents.push(string.concat(addr," has added you as a publisher"));
        }
        
    }

    /*
    -find index of publisher
    */
    function findPublisher(address inputaddr, address pubaddr) private view returns (int){
        for(uint i = 0; i < userNetwork[inputaddr].publishers.length; i++){
            if(userNetwork[inputaddr].publishers[i] == pubaddr){
                return int(i);
            }
        }
        //proper error handling later
        return -1;
    }

    /*
    -remove a publisher
    */
    function removepublisher(address inputaddr, address pubaddr) public returns (bool){
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        require(userNetwork[pubaddr].addr != address(0x0), "PUBLISHER NOT A CURRENT USER");
        require(msg.sender == inputaddr, "msg.sender != inputaddr");

        bool err = true;

        int index = findPublisher(inputaddr, pubaddr);

        if(index != -1){
            //replace with last element
            userNetwork[inputaddr].publishers[uint(index)] = userNetwork[inputaddr].publishers[userNetwork[inputaddr].publishers.length-1];
            userNetwork[inputaddr].publishers.pop();
            string memory addr = Strings.toHexString(uint256(uint160(inputaddr)),20);
            userNetwork[pubaddr].notification_contents.push(string.concat(addr," has removed you as publisher"));
        }
        else{
            err = false;
        }
        
        return err;    
    }

    /*
    -get publishers list
    */
    function getPublishers(address inputaddr) public view returns (address[] memory) {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[inputaddr].publishers;
    }

    /*
    -create file struct
    */
    function createFile(string memory hash, address publisher, string memory checksum) private view returns (File memory){
        File memory newFile;
        newFile.hash = hash;
        newFile.dataowner = publisher;
        newFile.timestamp = block.timestamp;
        newFile.checksum = checksum;
        return newFile;
    }

    /*
    -upload hash
    -require user
    -get subscriber addresses => set those address notification to true
    -get timestamp
    -update subscriber file hash timestamp tuples
    */
    function uploadHash(string memory hash, address inputaddr, string memory checksum) public payable returns (bool) {
        //require(msg.sender == tx.origin);
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        require(msg.sender == inputaddr, "msg.sender != inputaddr");
        require(msg.value > 1 ether, "need payment!");

        bool err = true;

        //for every subscriber add hash to their files and give notification
        File memory newfile = createFile(hash, inputaddr, checksum);
        for(uint i = 0; i < userNetwork[inputaddr].subscribers.length; i++){
            //get subscriber
            address sub = userNetwork[inputaddr].subscribers[i];

            //check that inputaddr in sub publishers list
            //only upload file if input addr is a verified publisher for the subscriber
            if(findPublisher(sub, inputaddr) != -1){
                //add hash
                userFiles[sub].push(newfile);

                //update notification
                userNetwork[sub].notification = true;
                string memory addr = Strings.toHexString(uint256(uint160(inputaddr)),20);
                userNetwork[sub].notification_contents.push(string.concat(addr," has sent you a new file!"));

                //pay the user
                bool sent = payable(userNetwork[sub].addr).send(msg.value/(userNetwork[inputaddr].subscribers.length+1));
                if(!sent){
                    revert();
                }
            }
            else{
                err = false;
            }
        }

        return err;

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
        require(msg.sender == inputaddr, "msg.sender != inputaddr");

        //set notification to false
        userNetwork[inputaddr].notification = false;

        //clear notification
        delete userNetwork[inputaddr].notification_contents;
    }

    /*
    -get notification status
    */
    function hasNotification(address inputaddr) public view returns (uint) {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[inputaddr].notification_contents.length;
    }

    /*
    -get notification contents
    */
    function getNotification(address inputaddr) public view returns (string[] memory) {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[inputaddr].notification_contents;
    }

    function isUser(address inputaddr) public view returns (bool) {
        return userNetwork[inputaddr].addr != address(0x0);
    }

    /*
    -add user public key
    */
    function addPublicKey(address inputaddr, string memory key) public {
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        require(msg.sender == inputaddr, "msg.sender != inputaddr");

        userNetwork[inputaddr].pubKey = key;
    }
    
    /*
    -get user public key
    */
    function getPublicKey(address inputaddr) public view returns (string memory){
        require(userNetwork[inputaddr].addr != address(0x0), "YOU ARE NOT A CURRENT USER");
        return userNetwork[inputaddr].pubKey;
    }

    //TESTING FUNCTIONS

    //returns msg.sender
    function msgsenderpure() public view returns (address) {
        return sender;
    }

    //returns tx.origin
    function txoriginpure() public view returns (address) {
        return origin;
    }

    //returns msg.sender
    function msgsender() public{
        sender = msg.sender;
    }

    //returns tx.origin
    function txorigin() public{
        origin = tx.origin;
    }

    receive() external payable{

    }
    
    fallback() external payable{
    
    }
}
