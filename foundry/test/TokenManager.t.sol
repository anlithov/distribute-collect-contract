// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import "../src/TokenManager.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "forge-std/console.sol";

contract TokenManagerT is Test {
    TokenManager public tokenManager;
    IERC20 public mockToken;


    address public sender = address(0x10);
    address[] public wallets;
    uint256[] public parts;

    function setUp() public {
        tokenManager = new TokenManager();
        mockToken = new ERC20Mock();

        wallets = new address[](3);
        wallets[0] = address(0x1);
        wallets[1] = address(0x2);
        wallets[2] = address(0x3);

        parts = new uint256[](3);
        parts[0] = 33_333_333;
        parts[1] = 66_666_666;
        parts[2] = 1;

        // Mint some tokens for testing
        ERC20Mock(address(mockToken)).mint(sender, 1_000_000 ether);
        mockToken.approve(address(tokenManager), 1_000_000 ether);

        for (uint256 i = 0; i < wallets.length; i++) {
            address wallet = wallets[i];

            vm.prank(wallet);
            mockToken.approve(address(tokenManager), 100_000_000 ether);
            ERC20Mock(address(mockToken)).mint(wallet, 1_000_000 ether);
        }

        vm.deal(sender, 1_000 ether);

        vm.stopPrank();
    }

    function testDistributeNativeTokens() public {
        // Initial balance check
        uint256 receiver1Balance = wallets[0].balance;
        uint256 receiver2Balance = wallets[1].balance;
        uint256 receiver3Balance = wallets[2].balance;

        // Total amount to send
        uint256 totalAmount = 1 ether;

        // Send native tokens
        vm.prank(sender);
        tokenManager.distributeNativeTokens{value: totalAmount}(wallets, parts, totalAmount);

        // Check if the balances are updated correctly
        assertEq(wallets[0].balance, receiver1Balance + 0.33333333 ether);
        assertEq(wallets[1].balance, receiver2Balance + 0.66666666 ether);
        assertEq(wallets[2].balance, receiver3Balance + 0.00000001 ether);
    }

    function testDistributeERC20Tokens() public {
        // Initial token balances of the receivers
        uint256 receiver1Balance = mockToken.balanceOf(wallets[0]);
        uint256 receiver2Balance = mockToken.balanceOf(wallets[1]);
        uint256 receiver3Balance = mockToken.balanceOf(wallets[2]);

        uint256 totalAmount = 1_000 ether;

        // Sender mints and approves tokens to distribute
        vm.prank(sender);
        mockToken.approve(address(tokenManager), totalAmount);

        // Distribute ERC20 tokens
        vm.prank(sender);
        tokenManager.distributeERC20Tokens(address(mockToken), wallets, parts, totalAmount);

        // Check if the balances are updated correctly
        assertEq(mockToken.balanceOf(wallets[0]), receiver1Balance + 333.33333 ether);
        assertEq(mockToken.balanceOf(wallets[1]), receiver2Balance + 666.66666 ether);
        assertEq(mockToken.balanceOf(wallets[2]), receiver3Balance +   0.00001 ether);
    }

    function testCollectTokens() public {
        parts[0] = 50_500_000; // 50.5%
        parts[1] = 25_500_000; // 25.5%
        parts[2] = 25_500_000; // 25.5%

        // Initial balances of the wallets and sender
        uint256 senderInitialBalance = mockToken.balanceOf(sender);
        uint256 wallet1InitialBalance = mockToken.balanceOf(wallets[0]);
        uint256 wallet2InitialBalance = mockToken.balanceOf(wallets[1]);
        uint256 wallet3InitialBalance = mockToken.balanceOf(wallets[2]);

        // Sender collects tokens from wallets
        vm.prank(sender);
        tokenManager.collectTokens(address(mockToken), wallets, parts);

        // Verify collected balances
        assertEq(mockToken.balanceOf(sender), senderInitialBalance + (wallet1InitialBalance * 505 / 1000) + (wallet2InitialBalance * 255 / 1000) + (wallet3InitialBalance * 255 / 1000));
        assertEq(mockToken.balanceOf(wallets[0]), wallet1InitialBalance * (1000 - 505) / 1000);
        assertEq(mockToken.balanceOf(wallets[1]), wallet2InitialBalance * (1000 - 255) / 1000);
        assertEq(mockToken.balanceOf(wallets[2]), wallet3InitialBalance * (1000 - 255) / 1000);
    }

    function testFailInsufficientNativeTokenBalance() public {


        vm.prank(sender);

        uint256 totalAmount = 1_000_000_000 ether;
        uint256 senderBalance = sender.balance;
        console.log(senderBalance);
        console.log(totalAmount);
        tokenManager.distributeNativeTokens{value: totalAmount}(wallets, parts, totalAmount);
        console.log(totalAmount);
    }
}
