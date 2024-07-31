//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// This contract does one thing, simply.  it allows you to send eth or tokens to multiple recipients.  Useful for setting up a testnet and funding all the validators and stakers.

contract Multisender is Ownable {
    function sendEth(address[] calldata _recipients) public payable {
        uint256 val = msg.value / _recipients.length;
        for (uint256 i = 0; i < _recipients.length; i++) {
            payable(_recipients[i]).transfer(val);
        }
    }

    function sendTokens(
        address[] calldata _recipients,
        address tokenContract
    ) public {
        ERC20 tkn = ERC20(tokenContract);
        uint256 bal = tkn.balanceOf(address(this));
        uint256 val = bal / _recipients.length;
        for (uint256 i = 0; i < _recipients.length; i++) {
            tkn.transfer(_recipients[i], val);
        }
    }

    function withdraw() public onlyOwner {
        payable(msg.sender).transfer(address(this).balance);
    }

    function withdrawTokens(address tokenContract) public onlyOwner {
        ERC20 tkn = ERC20(tokenContract);
        uint256 bal = tkn.balanceOf(address(this));
        tkn.transfer(msg.sender, bal);
    }
}
