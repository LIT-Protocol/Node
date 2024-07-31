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

const provider = ethers.provider;

const BATCH_SIZE = 10000;
const CONCURRENT_BATCHES = 10;

async function scanBlockForEvents(provider, fromBlock, toBlock) {
  const logs = await provider.getLogs({
    fromBlock: fromBlock,
    toBlock: toBlock,
    topics: [ethers.utils.id('PKPMinted(uint256,bytes)')],
  });

  return logs;
}

async function scanBlockBatch(provider, fromBlock) {
  const toBlock = Math.min(
    fromBlock + BATCH_SIZE - 1,
    await provider.getBlockNumber()
  );
  console.log(`Scanning blocks from ${fromBlock} to ${toBlock}...`);
  const events = await scanBlockForEvents(provider, fromBlock, toBlock);
  console.log(
    `Found ${events.length} events in blocks ${fromBlock} to ${toBlock}.`
  );
  return events;
}

async function fetchMintEvents() {
  const latestBlock = await provider.getBlockNumber();
  console.log(`Starting scan. Latest block is ${latestBlock}.`);

  let currentBlock = 0; // You can adjust this value to a more recent block if desired.
  const allCollectedEvents = []; // This will store all the events collected during the scan

  while (currentBlock < latestBlock) {
    const promises = [];
    const limit = Math.min(
      currentBlock + BATCH_SIZE * CONCURRENT_BATCHES,
      latestBlock
    );

    for (
      let endBlock = currentBlock;
      endBlock < limit;
      endBlock += BATCH_SIZE
    ) {
      promises.push(scanBlockBatch(provider, endBlock));
    }

    const batchEvents = await Promise.all(promises);
    for (const events of batchEvents) {
      allCollectedEvents.push(...events);
    }

    currentBlock += BATCH_SIZE * CONCURRENT_BATCHES;
  }

  console.log('Scan complete.');
  return allCollectedEvents;
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
