// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

contract Auditability {
    address public owner;

    struct IndexData {
        bytes32 hash;
        bool exists;
    }

    mapping(string => IndexData) private indices;
    event IndexStored(string indexed index, bytes32 hash);

    constructor() {
        owner = msg.sender;
    }

    function store(string memory index, bytes32 hash) public {
        require(!indices[index].exists, "Index already added.");

        indices[index] = IndexData({hash: hash, exists: true});

        emit IndexStored(index, hash);
    }

    function proof(
        string memory index,
        bytes32 hash
    ) public view returns (bool) {
        require(indices[index].exists, "Index not found.");
        return indices[index].hash == hash;
    }

    function exists(string memory index) public view returns (bool) {
        return indices[index].exists;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner.");
        _;
    }
}
