import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {  Swap } from "../target/types/swap";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { assert } from "chai";

describe("cancel_offer", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.YourProgram as Program<Swap>;

  let tokenMintA: anchor.web3.PublicKey;
  let tokenMintB: anchor.web3.PublicKey;
  let makerTokenAccountA: any;

  const maker = provider.wallet;
  const offerId = new anchor.BN(1);

  before(async () => {
    tokenMintA = await createMint(
      provider.connection,
      maker.payer,
      maker.publicKey,
      null,
      6
    );

    tokenMintB = await createMint(
      provider.connection,
      maker.payer,
      maker.publicKey,
      null,
      6
    );

    makerTokenAccountA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      maker.payer,
      tokenMintA,
      maker.publicKey
    );

    await mintTo(
      provider.connection,
      maker.payer,
      tokenMintA,
      makerTokenAccountA.address,
      maker.publicKey,
      1000000
    );
  });

  it("makes an offer", async () => {
    const [offer] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        maker.publicKey.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const vault = anchor.utils.token.associatedAddress({
      mint: tokenMintA,
      owner: offer,
    });

    await program.methods
      .makeOffer(offerId, new anchor.BN(500000), new anchor.BN(300000))
      .accounts({
        maker: maker.publicKey,
        tokenMintA: tokenMintA,
        tokenMintB: tokenMintB,
        makerTokenAccountA: makerTokenAccountA.address,
        offer: offer,
        vault: vault,
      })
      .rpc();

    const vaultAccount = await provider.connection.getTokenAccountBalance(
      vault
    );
    assert.equal(vaultAccount.value.amount, "500000");
  });

  it("cancels the offer and refunds tokens", async () => {
    const [offer] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        maker.publicKey.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const vault = anchor.utils.token.associatedAddress({
      mint: tokenMintA,
      owner: offer,
    });

    const makerBalanceBefore = await provider.connection.getTokenAccountBalance(
      makerTokenAccountA.address
    );

    await program.methods
      .cancelOffer()
      .accounts({
        maker: maker.publicKey,
        tokenMintA: tokenMintA,
        makerTokenAccountA: makerTokenAccountA.address,
        offer: offer,
        vault: vault,
      })
      .rpc();

    const makerBalanceAfter = await provider.connection.getTokenAccountBalance(
      makerTokenAccountA.address
    );

    assert.equal(makerBalanceAfter.value.amount, "1000000");

    try {
      await provider.connection.getAccountInfo(vault);
      assert.fail("Vault should be closed");
    } catch (error) {
      assert.ok("Vault is closed");
    }

    try {
      await provider.connection.getAccountInfo(offer);
      assert.fail("Offer should be closed");
    } catch (error) {
      assert.ok("Offer is closed");
    }
  });
});
