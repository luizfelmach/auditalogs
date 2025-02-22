// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

struct MerkleRoot {
    bytes32 value;
    bool exists;
}

contract Auditability {
    mapping(string => MerkleRoot) hashes;

    function store(string memory index, bytes32 root) public {
        require(!hashes[index].exists, "Index already added.");
        hashes[index] = MerkleRoot(root, true);
    }

    function proof(
        string memory index,
        bytes32 root
    ) public view returns (bool) {
        return hashes[index].value == root;
    }
}
