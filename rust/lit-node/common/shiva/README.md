# Shiva

Tool for running multiple testnsts and controlling network mutation over an out of process interface

**Currently a work in progress, can only reliably manage a single testnet at this time.**

## Starting the runtime

`cargo run src/main.rs`

**Note: build artifacts will be in ../targets**

You will need to configure the 'IPFS_API_KEY` which you can get from Piniata's IPFS Pinning service
You will need to have dependencies installed within `../../../../blockchain/contracts`
Currently works with `npm` and `NodeJS version 18.x.x`

## Generating TS Type Defs
run `cargo test` which will create a `bindings` folder. This should not be commited.

***Note: response format heavily in development will be updated soon***


## HTTP controller routes

### POST `/test/create/testnet`

Creates a new tesnet and returns the id.

``` json
{
    // Number of peers in the inital epoch
  "nodeCount": 6,
  // The inerval for polling the chain for state updates
  "pollingInterval": "5000",
  // The amount of time between each epoch after the first inital epochs
  "epochLength": 3000,
  // (Optional) absolute path to lit node binary
  "customBuildPath": "<Path to lit node binary>",
  // (Optional) absolute path to lit action binary
  "litActionServerCustomBuildPath": "<path to lit action binary>"
}
```

**Response**

```json
{
    "testnetId": "<testnet id>",
    "command": "CREATE_TESTNET",
    "wasCanceled": false,
    "body": undefined,
    "last_state_observed": "Busy",
    "messages": undefined | [],
    "errors": undefined | [],
}
```

### GET `/test/poll/testnet/<id>`

**Response**

```json
{
    "testnetId": "<testnet id>",
    "command": "POKE",
    "wasCanceled": false,
    "body": "<STATE>",
    "last_state_observed": "Busy",
    "messages": undefined | [],
    "errors": undefined | [],
}
```

### GET `/test/get/testnets`

Returns all id's for testnets being managed

**Response**

```json
[
    // testnet id's may be in any state
]
```

### GET `/test/get/info/testnet/<id>`

Returns information on a given testnet
- active validators
- contract addresses
- epoch length

**Response**

```json
{
    "testnetId": "<testnet id>",
    "command": "POKE",
    "wasCanceled": false,
    "body": {
        // contract addresses
        "contractAddresses": [],
        // validator addresses
        "validatorAddresses": [],
        // epoch length
        "epochLength": 100,
    },
    "last_state_observed": "Busy",
    "messages": undefined | [],
    "errors": undefined | [],
}

```

### GET `/test/delete/testnet/<id>`

Deletes all resources for a given testnet id
stops all validators and will kill the chain
and Lit Action server

**Response**

```json
{
    "testnetId": "<testnet id>",
    "command": "SHUTDOWN",
    "wasCanceled": false,
    "body": true | false,
    "last_state_observed": "<state>",
    "messages": undefined | [],
    "errors": undefined | [],
}
```

### GET `/test/action/stop/random/wait<id>`

Stops a random validator in the active set and waits for the next epoch transition

**Response**

```json
{
    "testnetId": "<testnet id>",
    "command": "STOP_RANDOM_AND_WAIT",
    "wasCanceled": false,
    "body": true | false,
    "last_state_observed": "<state>",
    "messages": undefined | [],
    "errors": undefined | [],
}
```


### GET `/test/action/stop/random/<id>`

Stops a random validator in the active set. Does not wait for state to be Actve.

**Response**

```json
{
    "testnetId": "<testnet id>",
    "command": "STOP_RANDOM_AND_WAIT",
    "wasCanceled": false,
    "body": true | false,
    "last_state_observed": "<state>",
    "messages": undefined | [],
    "errors": undefined | [],
}
```

### GET `/test/action/transition/epoch/wait/<id>`

Will wait for the next epoch to be transitioned to.

**Response**

```json
{
    "testnetId": "<testnet id>",
    "command":  "TRANSITION_EPOCH_AND_WAIT",
    "wasCanceled": false,
    "body": true | false,
    "last_state_observed": "<state>",
    "messages": undefined | [],
    "errors": undefined | [],
}
```


## Testnet FSM

Each tesnet state is tracked. when a response contains a `last_state_observed` it will be one of the below options

**Note: If a testnet returns it's `lastStateObserved` as `UNKNOWN` it means the testnet had an error occure and it is no longer trackable by Shiva and has been shutdown**

<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:lucid="lucid" width="1040" height="840"><g transform="translate(-2140 -220)" lucid:page-tab-id="0_0"><path d="M-500-500h4000v2500H-500z" fill="#fff"/><path d="M2380 346a6 6 0 0 1 6-6h148a6 6 0 0 1 6 6v148a6 6 0 0 1-6 6h-148a6 6 0 0 1-6-6z" stroke="#b8f5ed" stroke-width="2" fill="#c3f7c8"/><use xlink:href="#a" transform="matrix(1,0,0,1,2385,345) translate(36.79012345679013 78.71527777777777)"/><path d="M2760 346a6 6 0 0 1 6-6h148a6 6 0 0 1 6 6v148a6 6 0 0 1-6 6h-148a6 6 0 0 1-6-6z" stroke="#b8f5ed" stroke-width="2" fill="#fff3d9"/><use xlink:href="#b" transform="matrix(1,0,0,1,2765,345) translate(44.783950617283956 78.71527777777777)"/><path d="M2760 586a6 6 0 0 1 6-6h148a6 6 0 0 1 6 6v148a6 6 0 0 1-6 6h-148a6 6 0 0 1-6-6z" stroke="#b8f5ed" stroke-width="2" fill="#e3fae3"/><use xlink:href="#c" transform="matrix(1,0,0,1,2765,585) translate(18.796296296296298 78.71527777777777)"/><path d="M2760 815.9a6 6 0 0 1 6-6h148a6 6 0 0 1 6 6v148a6 6 0 0 1-6 6h-148a6 6 0 0 1-6-6z" stroke="#b8f5ed" stroke-width="2" fill="#fe7070"/><use xlink:href="#d" transform="matrix(1,0,0,1,2765,814.8958333333331) translate(43.611111111111114 78.71527777777777)"/><path d="M2547.87 413.95h194.75" stroke="#3a414a" fill="none"/><path d="M2541.98 413.95l12.9-7.64v15.28zM2757.38 413.95l-14.26 4.63v-9.27z" stroke="#3a414a" fill="#3a414a"/><path d="M2465.76 507.87V654a6 6 0 0 0 6 6h270.86" stroke="#3a414a" fill="none"/><path d="M2465.76 501.98l7.64 12.9h-15.28zM2757.38 660l-14.26 4.63v-9.26z" stroke="#3a414a" fill="#3a414a"/><path d="M2378.5 416.58H2345a6 6 0 0 0-6 6V883.9a6 6 0 0 0 6 6h397.62" stroke="#3a414a" fill="none"/><path d="M2379 417.06h-.5v-.95h.5z" stroke="#3a414a" stroke-width=".05" fill="#3a414a"/><path d="M2757.38 889.9l-14.26 4.63v-9.27z" stroke="#3a414a" fill="#3a414a"/><path d="M2921.5 420h92.5a6 6 0 0 1 6 6v457.9a6 6 0 0 1-6 6h-76.62" stroke="#3a414a" fill="none"/><path d="M2921.5 420.48h-.5v-.96h.5z" stroke="#3a414a" stroke-width=".05" fill="#3a414a"/><path d="M2922.62 889.9l14.26-4.64v9.27z" stroke="#3a414a" fill="#3a414a"/><path d="M2921.5 660h33.5a6 6 0 0 1 6 6v217.9a6 6 0 0 1-6 6h-17.62" stroke="#3a414a" fill="none"/><path d="M2921.5 660.48h-.5v-.96h.5z" stroke="#3a414a" stroke-width=".05" fill="#3a414a"/><path d="M2922.62 889.9l14.26-4.64v9.27z" stroke="#3a414a" fill="#3a414a"/><defs><path fill="#282c33" d="M233-177c-1 41-23 64-60 70L243 0h-38l-65-103H63V0H30v-248c88 3 205-21 203 71zM63-129c60-2 137 13 137-47 0-61-80-42-137-45v92" id="e"/><path fill="#282c33" d="M30 0v-248h187v28H63v79h144v27H63v87h162V0H30" id="f"/><path fill="#282c33" d="M205 0l-28-72H64L36 0H1l101-248h38L239 0h-34zm-38-99l-47-123c-12 45-31 82-46 123h93" id="g"/><path fill="#282c33" d="M30-248c118-7 216 8 213 122C240-48 200 0 122 0H30v-248zM63-27c89 8 146-16 146-99s-60-101-146-95v194" id="h"/><path fill="#282c33" d="M137-103V0h-34v-103L8-248h37l75 118 75-118h37" id="i"/><g id="a"><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,0,0)" xlink:href="#e"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,15.987654320987653,0)" xlink:href="#f"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,30.80246913580247,0)" xlink:href="#g"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,45.61728395061728,0)" xlink:href="#h"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,61.60493827160493,0)" xlink:href="#i"/></g><path fill="#282c33" d="M160-131c35 5 61 23 61 61C221 17 115-2 30 0v-248c76 3 177-17 177 60 0 33-19 50-47 57zm-97-11c50-1 110 9 110-42 0-47-63-36-110-37v79zm0 115c55-2 124 14 124-45 0-56-70-42-124-44v89" id="j"/><path fill="#282c33" d="M232-93c-1 65-40 97-104 97C67 4 28-28 28-90v-158h33c8 89-33 224 67 224 102 0 64-133 71-224h33v155" id="k"/><path fill="#282c33" d="M185-189c-5-48-123-54-124 2 14 75 158 14 163 119 3 78-121 87-175 55-17-10-28-26-33-46l33-7c5 56 141 63 141-1 0-78-155-14-162-118-5-82 145-84 179-34 5 7 8 16 11 25" id="l"/><g id="b"><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,0,0)" xlink:href="#j"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,14.814814814814813,0)" xlink:href="#k"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,30.80246913580247,0)" xlink:href="#l"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,45.61728395061728,0)" xlink:href="#i"/></g><path fill="#282c33" d="M240 0l2-218c-23 76-54 145-80 218h-23L58-218 59 0H30v-248h44l77 211c21-75 51-140 76-211h43V0h-30" id="m"/><path fill="#282c33" d="M127-220V0H93v-220H8v-28h204v28h-85" id="n"/><path fill="#282c33" d="M33 0v-248h34V0H33" id="o"/><path fill="#282c33" d="M190 0L58-211 59 0H30v-248h39L202-35l-2-213h31V0h-41" id="p"/><path fill="#282c33" d="M143 4C61 4 22-44 18-125c-5-107 100-154 193-111 17 8 29 25 37 43l-32 9c-13-25-37-40-76-40-61 0-88 39-88 99 0 61 29 100 91 101 35 0 62-11 79-27v-45h-74v-28h105v86C228-13 192 4 143 4" id="q"/><g id="c"><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,0,0)" xlink:href="#m"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,18.456790123456788,0)" xlink:href="#k"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,34.44444444444444,0)" xlink:href="#n"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,46.2962962962963,0)" xlink:href="#g"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,59.44444444444444,0)" xlink:href="#n"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,72.96296296296296,0)" xlink:href="#o"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,79.1358024691358,0)" xlink:href="#p"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,95.12345679012346,0)" xlink:href="#q"/></g><g id="d"><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,0,0)" xlink:href="#n"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,13.518518518518517,0)" xlink:href="#f"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,28.33333333333333,0)" xlink:href="#e"/><use transform="matrix(0.06172839506172839,0,0,0.06172839506172839,44.32098765432098,0)" xlink:href="#m"/></g></defs></g></svg>
