// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require('hardhat');
const fs = require('fs');
var spawn = require('child_process').spawn;

const { ethers } = hre;
const chainName = hre.network.name;
const rpcUrl = hre.network.config.url;

const CONTRACT_ADDRESS = '0x8F75a53F65e31DD0D2e40d0827becAaE2299D111';
const CONTRACT_ABI = [
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: 'address',
        name: 'from',
        type: 'address',
      },
      {
        indexed: true,
        internalType: 'address',
        name: 'to',
        type: 'address',
      },
      {
        indexed: true,
        internalType: 'uint256',
        name: 'tokenId',
        type: 'uint256',
      },
    ],
    name: 'Transfer',
    type: 'event',
  },
];

const provider = ethers.provider;
const contract = new ethers.Contract(CONTRACT_ADDRESS, CONTRACT_ABI, provider);

async function fetchMintEvents() {
  const ZERO_ADDRESS = '0x0000000000000000000000000000000000000000';
  const eventFilter = contract.filters.Transfer(ZERO_ADDRESS, null, null);
  let events = [];
  const BATCH_SIZE = 10; // Adjust the batch size as required
  const MAX_BLOCKS_PER_QUERY = 10000; // Adjust as needed
  const latestBlock = await provider.getBlockNumber();
  console.log(`Total blocks to scan: ${latestBlock}`);
  for (
    let startBlock = 0;
    startBlock <= latestBlock;
    startBlock += MAX_BLOCKS_PER_QUERY * BATCH_SIZE
  ) {
    // Create an array of promises for each batch
    let promises = [];
    for (
      let offset = 0;
      offset < BATCH_SIZE &&
      startBlock + offset * MAX_BLOCKS_PER_QUERY <= latestBlock;
      offset++
    ) {
      const fromBlock = startBlock + offset * MAX_BLOCKS_PER_QUERY;
      const toBlock = Math.min(fromBlock + MAX_BLOCKS_PER_QUERY, latestBlock);
      promises.push(contract.queryFilter(eventFilter, fromBlock, toBlock));
    }
    // Fetch events in parallel batches
    const batchResults = await Promise.all(promises);
    // Merge the results
    for (let batch of batchResults) {
      events = [...events, ...batch];
    }
    console.log(
      `Scanned up to block: ${Math.min(
        startBlock + MAX_BLOCKS_PER_QUERY * BATCH_SIZE,
        latestBlock
      )}`
    );
  }
  return events;
}

async function getEventCounts(events) {
  let counts = {};
  let blockCache = {};

  // Extract unique block numbers from events
  const blockNumbers = [...new Set(events.map((event) => event.blockNumber))];

  console.log(`Total unique blocks to fetch: ${blockNumbers.length}`);

  // Function to fetch a block's timestamp
  const MAX_RETRIES = 20; // You can adjust this number as needed
  const RETRY_DELAY = 1000; // Delay between retries (in ms)

  async function fetchTimestamp(blockNumber) {
    if (blockCache[blockNumber]) {
      return blockCache[blockNumber];
    }

    for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
      try {
        const block = await provider.getBlock(blockNumber);
        blockCache[blockNumber] = block.timestamp;
        return block.timestamp;
      } catch (error) {
        if (attempt === MAX_RETRIES) {
          throw error;
        }
        console.warn(
          `Error fetching block ${blockNumber}. Retrying in ${RETRY_DELAY}ms... (Attempt ${attempt}/${MAX_RETRIES})`
        );
        await new Promise((resolve) => setTimeout(resolve, RETRY_DELAY));
      }
    }
  }

  // Fetch timestamps in batches
  const BATCH_SIZE = 20;
  let timestampMap = {};

  for (let i = 0; i < blockNumbers.length; i += BATCH_SIZE) {
    const batchBlockNumbers = blockNumbers.slice(i, i + BATCH_SIZE);
    const batchTimestamps = await Promise.all(
      batchBlockNumbers.map(fetchTimestamp)
    );

    batchBlockNumbers.forEach((blockNumber, index) => {
      timestampMap[blockNumber] = batchTimestamps[index];
    });

    console.log(
      `Processed blocks: ${i + batchBlockNumbers.length} / ${
        blockNumbers.length
      }`
    );
  }

  console.log(
    'Finished fetching block timestamps. Now counting events per day...'
  );

  // Count events per day using the cached timestamps
  for (let event of events) {
    const timestamp = timestampMap[event.blockNumber];
    const date = new Date(timestamp * 1000).toISOString().split('T')[0];
    counts[date] = (counts[date] || 0) + 1;
  }

  console.log('Finished counting events per day.');

  return counts;
}

function generateCsv(counts) {
  console.log('generating csv');
  let csv = 'Date,Count\n';
  for (let date in counts) {
    csv += `${date},${counts[date]}\n`;
  }
  return csv;
}

(async function () {
  console.log('Counting PKPs');

  const events = await fetchMintEvents();
  const counts = await getEventCounts(events);
  const csv = generateCsv(counts);
  console.log(csv); // Save this CSV data or handle as needed

  // Write the CSV data to a file
  fs.writeFileSync('events.csv', csv);
})();
