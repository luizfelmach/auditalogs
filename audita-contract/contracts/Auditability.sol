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

    constructor(address _owner) {
        owner = _owner;
    }

    function store(string memory index, bytes32 hash) public onlyOwner {
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

contract AuditabilityFactory {
    address public factoryOwner;
    mapping(address => address[]) public deployedAuditContracts;
    event AuditContractCreated(address indexed owner, address contractAddress);

    constructor() {
        factoryOwner = msg.sender;
    }

    function createAuditContract() public returns (address) {
        Auditability newAuditContract = new Auditability(msg.sender);
        deployedAuditContracts[msg.sender].push(address(newAuditContract));
        emit AuditContractCreated(msg.sender, address(newAuditContract));
        return address(newAuditContract);
    }

    function getDeployedContracts(
        address owner
    ) public view returns (address[] memory) {
        return deployedAuditContracts[owner];
    }

    function getContractCount(address owner) public view returns (uint256) {
        return deployedAuditContracts[owner].length;
    }
}
