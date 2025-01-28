const go = async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    const _ = await Lit.Actions.call({ ipfsId: 'QmRwN9GKHvCn4Vk7biqtr6adjXMs7PzzYPCzNCRjPFiDjm', params: {
        toSign: Array.from(toSign),
        publicKey,
        sigName
    }});
  };
go();
