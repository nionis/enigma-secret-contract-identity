pragma solidity ^0.5.0;

import "@openzeppelin/contracts/ownership/Ownable.sol";

/* not implemented yet */
contract Receiver is Ownable {
  address public enigma;

  modifier onlyEnigma {
    require(msg.sender == enigma, "only enigma allowed");
    _;
  }

  function setSecretContract(address _enigma) public onlyOwner {
    enigma = _enigma;
  }
}