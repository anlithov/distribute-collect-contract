// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "forge-std/console.sol";

/// @title  TokenDistributor
/// @notice This contract is used for distributing to multiple wallets native and ERC20
///         tokens as well as for collecting from different wallets.
contract TokenManager is ReentrancyGuard {
    uint256 public constant CALC_PRECISION = 1_000_000_000_000_000_000;
    uint256 public constant PERCENT_PRECISION = 1_000_000;

    error InvalidLengthOfWalletsOrParts();
    error InvalidWalletsLength();
    error ZeroSpentAmount();
    error InsufficientSpentAmount();
    error InvalidsPartsQuantity();
    error InvalidsSpentQuantity();
    error TooEarly();

    modifier validReceiversAndParts(
        address[] calldata receivers,
        uint256[] calldata parts
    ) {
        if (receivers.length != parts.length) {
            revert InvalidLengthOfWalletsOrParts();
        }
        if (receivers.length == 0) {
            revert InvalidWalletsLength();
        }
        _;
    }

    modifier validTotalParts(uint256[] calldata proportions) {
        uint256 totalParts = 0;
        for (uint256 i = 0; i < proportions.length; i++) {
            totalParts += proportions[i];
        }

        if (totalParts == 0) {
            revert InvalidsPartsQuantity();
        }
        _;
    }

    modifier validSpentAmount(uint256 totalAmount) {
        // require(totalAmount > 0, "Sent amount must be greater than zero");
        if (totalAmount == 0) {
            revert ZeroSpentAmount();
        }
        _;
    }

    // ******************************** //
    //          DISTRIBUTION           //
    // ******************************** //

    /// @notice Sends Native tokens to different wallets
    /// @param proportions - flexible param to define parts.
    /// @dev All parts will be summarized and treated like 100%.
    ///      Each part will have its own value from total amount.
    ///      To solve floating percentages - scaled percentages (multiplied by 10^*) can be sent as arguments
    /// @dev Calculation uses additional scale multiplier "CALC_PRECISION" for best precision
    function distributeNativeTokens(
        address[] calldata receivers,
        uint256[] calldata proportions,
        uint256 totalAmount
    )
    external
    payable
    nonReentrant
    validReceiversAndParts(receivers, proportions)
    validTotalParts(proportions)
    validSpentAmount(totalAmount)
    {
        uint256 totalParts = 0;
        for (uint256 i = 0; i < proportions.length; i++) {
            totalParts += proportions[i];
        }

        for (uint256 i = 0; i < receivers.length; i++) {
            uint256 recipientAmount = (totalAmount *
            proportions[i] *
                CALC_PRECISION) / (totalParts * CALC_PRECISION);

            (bool success,) = receivers[i].call{value: recipientAmount}("");
            require(
                success,
                "Native token transfer failed"
            );
        }
    }

    /// @notice Sends ERC20 tokens to different wallets
    /// @param proportions - flexible param to define parts.
    /// @dev All parts will be summarized and treated like 100%.
    ///      Each part will have its own value from total amount.
    ///      To solve floating percentages - scaled percentages (multiplied by 10^*) can be sent as arguments
    /// @dev Calculation uses additional scale multiplier "CALC_PRECISION" for best precision
    function distributeERC20Tokens(
        address tokenAddress,
        address[] calldata receivers,
        uint256[] calldata proportions,
        uint256 totalAmount
    )
    external
    nonReentrant
    validReceiversAndParts(receivers, proportions)
    validTotalParts(proportions)
    validSpentAmount(totalAmount)
    {
        IERC20 token = IERC20(tokenAddress);

        uint256 senderBalance = token.balanceOf(msg.sender);
        if (senderBalance < totalAmount) {
            revert InsufficientSpentAmount();
        }

        uint256 totalParts = 0;
        for (uint256 i = 0; i < proportions.length; i++) {
            totalParts += proportions[i];
        }

        for (uint256 i = 0; i < receivers.length; i++) {
            uint256 recipientAmount = (totalAmount *
            proportions[i] *
                CALC_PRECISION) / (totalParts * CALC_PRECISION);
            require(
                token.transferFrom(msg.sender, receivers[i], recipientAmount),
                "Token transfer failed"
            );
        }
    }

    // ******************************** //
    //            COLLECTION            //
    // ******************************** //

    /// @notice Collects ERC20 tokens from different wallets to one sender
    /// @param percentages - scaled by "PERCENT_PRECISION". Should be passed scaled!
    /// @dev Percentages are increased by "PERCENT_PRECISION" (10^6) to provide ability of usage floating numbers
    ///      e.g. 50.147256% - should be sent as 50147256
    /// @dev Calculation uses additional scale multiplier "CALC_PRECISION" for best precision
    function collectERC20Tokens(
        address tokenAddress,
        address[] calldata wallets,
        uint256[] calldata percentages
    ) external nonReentrant validReceiversAndParts(wallets, percentages) {
        IERC20 token = IERC20(tokenAddress);

        for (uint256 i = 0; i < wallets.length; i++) {
            uint256 walletBalance = token.balanceOf(wallets[i]);

            // Skip if no balance
            if (walletBalance == 0) {
                continue;
            }

            // Calculate the amount to collect based on the percentage
            uint256 collectAmount = (walletBalance *
            percentages[i] *
                CALC_PRECISION) / (100 * PERCENT_PRECISION * CALC_PRECISION);

            // Ensure the sender has approved this contract to transfer the required tokens
            require(
                token.transferFrom(wallets[i], msg.sender, collectAmount),
                "Token transfer failed"
            );
        }
    }
}
