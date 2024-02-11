This project exist to keep the contract generation separate from the lit-blockchain project. This is because the lit-blockchain project depends on lit-core, and this creates a catch-22 when the generated contracts won't compile.

The issue is, if you update ethers, then the lit-blockchain project sometimes won't compile, because the generated contracts don't conform to the new ethers version. So by separating the contract generation into it's own project, we can generate the contracts, and then update the lit-blockchain project to use the new contracts.

To use this, run "make update" inside the lit-blockchain project.
