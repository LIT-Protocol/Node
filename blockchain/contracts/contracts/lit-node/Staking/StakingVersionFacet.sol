//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { StakingBalances } from "../StakingBalances.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

import { LibStakingStorage } from "./LibStakingStorage.sol";

// import "hardhat/console.sol";

contract StakingVersionFacet {
    error CallerNotOwner();

    /* ========== Modifiers ========== */

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
        _;
    }

    /* ========== VIEWS ========== */

    function getMinVersion()
        external
        view
        returns (LibStakingStorage.Version memory)
    {
        return LibStakingStorage.getStorage().versionRequirements[0];
    }

    function getMaxVersion()
        external
        view
        returns (LibStakingStorage.Version memory)
    {
        return LibStakingStorage.getStorage().versionRequirements[1];
    }

    function getMinVersionString() external view returns (string memory) {
        LibStakingStorage.Version storage minVersion = LibStakingStorage
            .getStorage()
            .versionRequirements[0];

        return
            string(
                abi.encodePacked(
                    Strings.toString(minVersion.major),
                    ".",
                    Strings.toString(minVersion.minor),
                    ".",
                    Strings.toString(minVersion.patch)
                )
            );
    }

    function getMaxVersionString() external view returns (string memory) {
        LibStakingStorage.Version storage maxVersion = LibStakingStorage
            .getStorage()
            .versionRequirements[1];

        return
            string(
                abi.encodePacked(
                    Strings.toString(maxVersion.major),
                    ".",
                    Strings.toString(maxVersion.minor),
                    ".",
                    Strings.toString(maxVersion.patch)
                )
            );
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function setMinVersion(
        LibStakingStorage.Version memory version
    ) external onlyOwner {
        LibStakingStorage.getStorage().versionRequirements[0] = version;
        emit VersionRequirementsUpdated(0, version);
    }

    function setMaxVersion(
        LibStakingStorage.Version memory version
    ) external onlyOwner {
        LibStakingStorage.getStorage().versionRequirements[1] = version;
        emit VersionRequirementsUpdated(1, version);
    }

    function checkVersion(
        LibStakingStorage.Version memory version
    ) public view returns (bool) {
        LibStakingStorage.Version storage minVersion = LibStakingStorage
            .getStorage()
            .versionRequirements[0];
        LibStakingStorage.Version storage maxVersion = LibStakingStorage
            .getStorage()
            .versionRequirements[1];

        if (
            (version.major > minVersion.major &&
                version.major < maxVersion.major) ||
            (version.major == minVersion.major &&
                version.minor > minVersion.minor) ||
            (version.major == maxVersion.major &&
                version.minor < maxVersion.minor) ||
            (version.major == minVersion.major &&
                version.minor == minVersion.minor &&
                version.patch >= minVersion.patch) ||
            (version.major == maxVersion.major &&
                version.minor == maxVersion.minor &&
                version.patch <= maxVersion.patch)
        ) {
            return true;
        } else {
            return false;
        }
    }

    /* ========== EVENTS ========== */

    event VersionRequirementsUpdated(
        uint256 index,
        LibStakingStorage.Version version
    );
}
