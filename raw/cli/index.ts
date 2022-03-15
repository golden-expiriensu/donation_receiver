import web3 = require('@solana/web3.js')

const connection = new web3.Connection(web3.clusterApiUrl('devnet'))
const key = Uint8Array.from([
    132,193,222,133,137,236,12,201,152,123,114,37,93,150,20,114,
    128,178,167,253,123,220,220,208,132,150,145,207,174,57,66,88,
    212,215,130,236,37,189,39,43,8,46,33,251,166,251,13,131,244,
    72,87,214,157,222,47,122,195,229,142,95,67,197,193,214
])
const programId = new web3.PublicKey("8qGenigMJogtv3yFcUSCK3dyQiWTYi6kuydumJTscDh9")

const main = async () => {
    const signer = web3.Keypair.fromSecretKey(key)
    const data = Buffer.from("sdf")
    const transaction = new web3.Transaction().add(
        new web3.TransactionInstruction({
            keys: [],
            programId,
            data
        })
    )

    await web3.sendAndConfirmTransaction(
        connection,
        transaction,
        [signer]
    ).then(sig => console.log(sig))
}

main()
    .then(() => process.exit(0))
    .catch((e) => {
        console.error(e)
        process.exit(1)
    })