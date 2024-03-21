import * as anchor from "@coral-xyz/anchor";  
import { Program } from "@coral-xyz/anchor";
import { AnchorSplToken } from "../target/types/anchor_spl_token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { createAccount } from "@solana/spl-token";


describe("create-tokens", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorSplToken as Program<AnchorSplToken>;

  const mintToken = anchor.web3.Keypair.generate()

  const associateTokenProgram = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")

  const tokenAccount =anchor.utils.token.associatedAddress({mint:mintToken.publicKey,owner:provider.publicKey})

  const ta = anchor.web3.PublicKey.findProgramAddressSync(
    [provider.publicKey.toBuffer(),TOKEN_PROGRAM_ID.toBuffer(),mintToken.publicKey.toBuffer()],
    associateTokenProgram
  )[0]

  let tokenAccountKeyPair = anchor.web3.Keypair.generate()



  it("Create token!", async () => {

    console.log(mintToken.publicKey.toBase58())
    console.log(tokenAccount.toBase58())

    try {
      const tx = await program.methods.createToken(9,new anchor.BN(10**9*100))
        .accounts({
          mintToken:mintToken.publicKey,
          tokenAccount:tokenAccount,
          associateTokenProgram,
        })
        .signers([mintToken])
        .rpc();
        console.log("Your transaction signature", tx);
      } catch (error) {
        console.log(error)
      }
  });


  it("Token transfer", async () =>{

    let reciever = anchor.web3.Keypair.generate()

    const signature = await provider.connection.requestAirdrop(reciever.publicKey,anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction(signature)

    let recieverTokenAccountKeypair = anchor.web3.Keypair.generate()
    await createAccount(provider.connection,reciever,mintToken.publicKey,reciever.publicKey,recieverTokenAccountKeypair);

    try {
      const tx = await program.methods.transerToken(new anchor.BN(10**9*90))
      .accounts({
        mintToken:mintToken.publicKey,
        fromAccount:tokenAccount,
        toAccount:recieverTokenAccountKeypair.publicKey,
        associateTokenProgram
      })
      .signers([])
      .rpc()

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error)
    }


  })

  

  it("Set Authority token!", async () => {

    let new_signer = anchor.web3.Keypair.generate()
    try {
      const tx = await program.methods.setAuthorityToken(0)
      .accounts({
        mintToken:mintToken.publicKey,
        tokenAccount:tokenAccount,
        newSigner:new_signer.publicKey,
      })
      .signers([new_signer])
      .rpc();
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.log(e)
    }
  });

  it("Freeze token!", async () => {

    
    const tx = await program.methods.freezeToken()
    .accounts({
      mintToken:mintToken.publicKey,
      tokenAccount,
    })
    .signers([])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Unfreeze token!", async () => {

    
    const tx = await program.methods.unFreezeToken()
    .accounts({
      mintToken:mintToken.publicKey,
      tokenAccount,
    })
    .signers([])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Burn token!", async () => {

    try {
      const tx = await program.methods.burnToken(new anchor.BN(10**9*10))
        .accounts({
          mintToken:mintToken.publicKey,
          tokenAccount,
        })
        .signers([])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error)
    }
    
  });

  it("Close token!", async () => {

    
    const tx = await program.methods.closeToken()
    .accounts({
      mintToken:mintToken.publicKey,
      tokenAccount,
    })
    .signers([])
    .rpc();
    console.log("Your transaction signature", tx);
  });
  
});

