import { Wallet, verifyMessage } from 'ethers';
import { SiweMessage } from 'siwe';

export const getAuthSig = async () => {
    const wallet = Wallet.createRandom();
    const domain = 'localhost';
    const origin = 'https://localhost/';
    const statement = 'Lit is 🔥';

    const siweMessage = new SiweMessage({
        domain,
        address: wallet.address,
        statement,
        uri: origin,
        version: '1',
        chainId: '1',
        expirationTime: new Date(Date.now() + 1000 * 60 * 60 * 24 * 7).toISOString()
    });

    const messageToSign = siweMessage.prepareMessage();
    const signature = await wallet.signMessage(messageToSign);
    const recoveredAddress = verifyMessage(messageToSign, signature);

    const authSig = {
        sig: signature,
        derivedVia: 'web3.eth.personal.sign',
        signedMessage: messageToSign,
        address: recoveredAddress,
    };

    return authSig;
}
