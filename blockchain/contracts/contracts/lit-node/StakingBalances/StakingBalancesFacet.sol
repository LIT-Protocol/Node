//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { ERC20Burnable } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { Staking } from "../Staking.sol";
import { StakingFacet } from "../Staking/StakingFacet.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { LibStakingBalancesStorage } from "./LibStakingBalancesStorage.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";

import "hardhat/console.sol";

contract StakingBalancesFacet {
    using EnumerableSet for EnumerableSet.AddressSet;

    error CannotStakeZero();
    error StakeMustBeGreaterThanMinimumStake(
        uint256 amountStaked,
        uint256 minimumStake
    );
    error StakeMustBeLessThanMaximumStake(
        uint256 amountStaked,
        uint256 maximumStake
    );
    error TryingToWithdrawMoreThanStaked(
        uint256 yourBalance,
        uint256 requestedWithdrawlAmount
    );
    error CannotWithdrawZero();
    error OnlyStakingContract(address sender);
    error StakerNotPermitted(address stakerAddress);
    error ActiveValidatorsCannotLeave();
    error MaxAliasCountReached(uint256 aliasCount);
    error AliasNotOwnedBySender(address aliasAccount, address stakerAddress);
    error CannotRemoveAliasOfActiveValidator(address aliasAccount);
    error CallerNotOwner();

    modifier onlyStakingContract() {
        if (msg.sender != getStakingAddress()) {
            revert OnlyStakingContract(msg.sender);
        }
        _;
    }

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
        _;
    }

    /* ========== VIEWS ========== */
    function s()
        internal
        pure
        returns (LibStakingBalancesStorage.StakingBalancesStorage storage)
    {
        return LibStakingBalancesStorage.getStorage();
    }

    function contractResolver() external view returns (address) {
        return address(s().contractResolver);
    }

    /// get the staking address from the resolver
    function getStakingAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.STAKING_CONTRACT(),
                s().env
            );
    }

    /// get the token address from the resolver
    function getTokenAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.LIT_TOKEN_CONTRACT(),
                s().env
            );
    }

    function permittedStakersOn() public view returns (bool) {
        return s().permittedStakersOn;
    }

    function minimumStake() public view returns (uint256) {
        return s().minimumStake;
    }

    function maximumStake() public view returns (uint256) {
        return s().maximumStake;
    }

    function totalStaked() public view returns (uint256) {
        return s().totalStaked;
    }

    function balanceOf(address account) external view returns (uint256) {
        // support aliases
        if (s().aliases[account] != address(0)) {
            account = s().aliases[account];
        }
        return s().balances[account];
    }

    function rewardOf(address account) external view returns (uint256) {
        // support aliases
        if (s().aliases[account] != address(0)) {
            account = s().aliases[account];
        }
        return s().rewards[account];
    }

    function isPermittedStaker(address staker) public view returns (bool) {
        // support aliases
        if (s().aliases[staker] != address(0)) {
            staker = s().aliases[staker];
        }
        return s().permittedStakers[staker];
    }

    function checkStakingAmounts(address account) public view returns (bool) {
        // support aliases
        if (s().aliases[account] != address(0)) {
            account = s().aliases[account];
        }
        uint256 amountStaked = s().balances[account];
        if (amountStaked < s().minimumStake) {
            revert StakeMustBeGreaterThanMinimumStake(
                amountStaked,
                s().minimumStake
            );
        }
        if (amountStaked > s().maximumStake) {
            revert StakeMustBeLessThanMaximumStake(
                amountStaked,
                s().maximumStake
            );
        }
        return true;
    }

    /* ========== MUTATIVE FUNCTIONS ========== */
    /// Stake tokens for a validator
    function stake(uint256 amount, address account) public onlyStakingContract {
        if (amount == 0) {
            revert CannotStakeZero();
        }

        if (s().permittedStakersOn && !s().permittedStakers[account]) {
            revert StakerNotPermitted(account);
        }

        ERC20Burnable stakingToken = ERC20Burnable(getTokenAddress());
        stakingToken.transferFrom(account, address(this), amount);
        s().balances[account] += amount;

        s().totalStaked += amount;

        emit Staked(account, amount);
    }

    /// Stake tokens for another user
    function stakeForValidator(
        uint256 amount,
        address account,
        address sender
    ) public onlyStakingContract {
        if (amount == 0) {
            revert CannotStakeZero();
        }

        if (s().permittedStakersOn && !s().permittedStakers[account]) {
            revert StakerNotPermitted(account);
        }

        ERC20Burnable stakingToken = ERC20Burnable(getTokenAddress());
        stakingToken.transferFrom(sender, address(this), amount);
        s().balances[account] += amount;

        s().totalStaked += amount;

        emit Staked(account, amount);
    }

    /// Withdraw staked tokens.  This can only be done by users who are not active in the validator set.
    /// @param amount The amount of tokens to withdraw
    function withdraw(
        uint256 amount,
        address account
    ) public onlyStakingContract {
        if (amount == 0) {
            revert CannotWithdrawZero();
        }
        StakingViewsFacet staking = StakingViewsFacet(getStakingAddress());
        address[] memory validatorsInCurrentEpoch = staking
            .getValidatorsInCurrentEpoch();
        bool isValidatorInCurrentEpoch = false;
        for (uint256 i = 0; i < validatorsInCurrentEpoch.length; i++) {
            if (validatorsInCurrentEpoch[i] == account) {
                isValidatorInCurrentEpoch = true;
                break;
            }
        }
        if (isValidatorInCurrentEpoch) {
            revert ActiveValidatorsCannotLeave();
        }

        if (s().balances[account] < amount) {
            revert TryingToWithdrawMoreThanStaked(
                s().balances[account],
                amount
            );
        }

        s().totalStaked = s().totalStaked - amount;
        s().balances[account] = s().balances[account] - amount;

        ERC20Burnable stakingToken = ERC20Burnable(getTokenAddress());
        stakingToken.transfer(account, amount);
        emit Withdrawn(account, amount);
    }

    function rewardValidator(
        uint256 amount,
        address account
    ) public onlyStakingContract {
        // support aliases
        if (s().aliases[account] != address(0)) {
            // only reward if the main staking account is not an active validator, too
            StakingViewsFacet staking = StakingViewsFacet(getStakingAddress());
            if (staking.isActiveValidator(s().aliases[account])) {
                emit ValidatorNotRewardedBecauseAlias(
                    s().aliases[account],
                    account
                );
                return;
            }
            account = s().aliases[account];
        }
        s().rewards[account] += amount;
        emit ValidatorRewarded(account, amount);
    }

    function penalizeTokens(
        uint256 amount,
        address account
    ) public onlyStakingContract {
        if (s().aliases[account] != address(0)) {
            account = s().aliases[account];
        }

        s().balances[account] -= amount;
        s().totalStaked -= amount;

        s().penaltyBalance += amount;
        emit ValidatorTokensPenalized(account, amount);
    }

    /// Transfer any outstanding reward tokens
    function getReward(address account) public onlyStakingContract {
        if (s().aliases[account] != address(0)) {
            account = s().aliases[account];
        }

        uint256 reward = s().rewards[account];
        if (reward > 0) {
            s().rewards[account] = 0;
            ERC20Burnable stakingToken = ERC20Burnable(getStakingAddress());
            stakingToken.transfer(account, reward);
            emit RewardPaid(account, reward);
        }
    }

    /// Add an alias.  Must come from staker address.
    function addAlias(address aliasAccount) public {
        if (s().aliasCounts[msg.sender] >= s().maxAliasCount) {
            revert MaxAliasCountReached(s().aliasCounts[msg.sender]);
        }
        s().aliases[aliasAccount] = msg.sender;
        s().aliasCounts[msg.sender] += 1;
        emit AliasAdded(msg.sender, aliasAccount);
    }

    /// Remove an alias.  Must come from staker address.
    function removeAlias(address aliasAccount) public {
        // auth
        if (s().aliases[aliasAccount] != msg.sender) {
            revert AliasNotOwnedBySender(aliasAccount, msg.sender);
        }
        // don't let them remove an alias of an active validator
        StakingViewsFacet staking = StakingViewsFacet(getStakingAddress());
        if (staking.isActiveValidator(aliasAccount)) {
            revert CannotRemoveAliasOfActiveValidator(aliasAccount);
        }

        delete s().aliases[aliasAccount];
        s().aliasCounts[msg.sender] -= 1;
        emit AliasRemoved(msg.sender, aliasAccount);
    }

    function withdrawPenaltyTokens(uint256 balance) public onlyOwner {
        require(balance <= s().penaltyBalance, "Not enough penalty balance");

        s().penaltyBalance -= balance;

        ERC20Burnable stakingToken = ERC20Burnable(getTokenAddress());
        stakingToken.transfer(msg.sender, balance);
    }

    function transferPenaltyTokens(
        uint256 balance,
        address recipient
    ) public onlyOwner {
        require(balance <= s().penaltyBalance, "Not enough penalty balance");

        s().penaltyBalance -= balance;

        ERC20Burnable stakingToken = ERC20Burnable(getTokenAddress());
        stakingToken.transfer(recipient, balance);
    }

    function restakePenaltyTokens(
        address staker,
        uint256 balance
    ) public onlyOwner {
        require(balance <= s().penaltyBalance, "Not enough penalty balance");

        s().totalStaked += balance;
        s().penaltyBalance -= balance;

        s().balances[staker] += balance;

        ERC20Burnable stakingToken = ERC20Burnable(getTokenAddress());
        stakingToken.transfer(address(this), balance);
    }

    /// this is for if someone accidently sends unwrapped tokens
    function withdraw() public onlyOwner {
        uint256 withdrawAmount = address(this).balance;
        (bool sent, ) = payable(msg.sender).call{ value: withdrawAmount }("");
        require(sent);
    }

    function addPermittedStakers(address[] memory stakers) public onlyOwner {
        for (uint256 i = 0; i < stakers.length; i++) {
            addPermittedStaker(stakers[i]);
        }
    }

    function addPermittedStaker(address staker) public onlyOwner {
        s().permittedStakers[staker] = true;
        emit PermittedStakerAdded(staker);
    }

    function removePermittedStaker(address staker) public onlyOwner {
        s().permittedStakers[staker] = false;
        emit PermittedStakerRemoved(staker);
    }

    function setPermittedStakersOn(bool permitted) public onlyOwner {
        s().permittedStakersOn = permitted;
        emit PermittedStakersOnChanged(permitted);
    }

    function setMinimumStake(uint256 newMinimumStake) public onlyOwner {
        s().minimumStake = newMinimumStake;
        emit MinimumStakeSet(newMinimumStake);
    }

    function setMaximumStake(uint256 newMaximumStake) public onlyOwner {
        s().maximumStake = newMaximumStake;
        emit MaximumStakeSet(newMaximumStake);
    }

    function setContractResolver(address newResolverAddress) public onlyOwner {
        s().contractResolver = ContractResolver(newResolverAddress);
        emit ResolverContractAddressSet(newResolverAddress);
    }

    function setMaxAliasCount(uint256 newMaxAliasCount) public onlyOwner {
        s().maxAliasCount = newMaxAliasCount;
        emit MaxAliasCountSet(newMaxAliasCount);
    }

    /* ========== EVENTS ========== */

    event Staked(address indexed staker, uint256 amount);
    event Withdrawn(address indexed staker, uint256 amount);
    event ValidatorRewarded(address indexed staker, uint256 amount);
    event ValidatorTokensPenalized(address indexed staker, uint256 amount);
    event RewardPaid(address indexed staker, uint256 reward);
    event AliasAdded(address indexed staker, address aliasAccount);
    event AliasRemoved(address indexed staker, address aliasAccount);
    event ValidatorNotRewardedBecauseAlias(
        address indexed staker,
        address aliasAccount
    );

    // onlyOwner events
    event TokenRewardPerTokenPerEpochSet(
        uint256 newTokenRewardPerTokenPerEpoch
    );
    event MinimumStakeSet(uint256 newMinimumStake);
    event MaximumStakeSet(uint256 newMaximumStake);
    event PermittedStakerAdded(address staker);
    event PermittedStakerRemoved(address staker);
    event PermittedStakersOnChanged(bool permittedStakersOn);
    event ResolverContractAddressSet(address newResolverAddress);
    event MaxAliasCountSet(uint newMaxAliasCount);
}
