import * as anchor from "@coral-xyz/anchor";
import {
  getClusterAccAddress,
  getArciumEnv,
  getMXEAccAddress,
  getMempoolAccAddress,
  getExecutingPoolAccAddress,
  getCompDefAccAddress,
  getCompDefAccOffset,
  getComputationAccAddress,
  awaitComputationFinalization,
  x25519,
  getMXEPublicKey,
  RescueCipher,
  deserializeLE,
} from "@arcium-hq/client";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { useCallback } from "react";
import { randomBytes } from "crypto";

export function useArcium(program: anchor.Program<any>) {
  const { connection } = useConnection();
  const { publicKey } = useWallet();

  const submitPrivateOrder = useCallback(async (params: {
    size: number;
    price: number;
    direction: number; // 0 = long, 1 = short
    leverage: number;
  }) => {
    if (!publicKey) throw new Error("Wallet not connected");

    const provider = program.provider as anchor.AnchorProvider;
    const arciumEnv = getArciumEnv();
    const clusterAccount = getClusterAccAddress(arciumEnv.arciumClusterOffset);

    // Step 1 — Key exchange with MXE
    const clientPrivateKey = x25519.utils.randomSecretKey();
    const clientPublicKey = x25519.getPublicKey(clientPrivateKey);
    const mxePublicKey = await getMXEPublicKey(provider, program.programId);
    if (!mxePublicKey) throw new Error("MXE not ready");
    const sharedSecret = x25519.getSharedSecret(clientPrivateKey, mxePublicKey);
    const cipher = new RescueCipher(sharedSecret);

    // Step 2 — Encrypt sensitive inputs
    const inputs = [
      BigInt(params.size),
      BigInt(params.price),
      BigInt(params.leverage),
    ];
    const nonce = randomBytes(16);
    const ciphertext = cipher.encrypt(inputs, nonce);
    const computationOffset = new anchor.BN(randomBytes(8), "hex");
    const nonceBN = new anchor.BN(deserializeLE(nonce).toString());

    // Step 3 — Listen for result event before submitting
    type Event = anchor.IdlEvents<typeof program["idl"]>;
    const awaitEvent = async <E extends keyof Event>(eventName: E): Promise<Event[E]> => {
      let listenerId: number;
      const event = await new Promise<Event[E]>((res) => {
        listenerId = program.addEventListener(eventName, (event: Event[E]) => res(event));
      });
      await program.removeEventListener(listenerId!);
      return event;
    };
    const resultEventPromise = awaitEvent("orderMatchResult");

    // Step 4 — Submit encrypted computation to Arcium
    const compDefIndex = Buffer.from(
      getCompDefAccOffset("order_match")
    ).readUInt32LE();

    await program.methods
      .openPosition(
        computationOffset,
        Array.from(ciphertext[0]), // encrypted size
        Array.from(ciphertext[1]), // encrypted price
        Array.from(ciphertext[2]), // encrypted leverage
        Array.from(clientPublicKey),
        nonceBN,
        params.direction
      )
      .accountsPartial({
        trader: publicKey,
        clusterAccount,
        mxeAccount: getMXEAccAddress(program.programId),
        mempoolAccount: getMempoolAccAddress(arciumEnv.arciumClusterOffset),
        executingPool: getExecutingPoolAccAddress(arciumEnv.arciumClusterOffset),
        compDefAccount: getCompDefAccAddress(program.programId, compDefIndex),
        computationAccount: getComputationAccAddress(
          arciumEnv.arciumClusterOffset,
          computationOffset
        ),
      })
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    // Step 5 — Wait for finalization
    await awaitComputationFinalization(
      provider,
      computationOffset,
      program.programId,
      "confirmed"
    );

    // Step 6 — Decrypt result
    const resultEvent = await resultEventPromise;
    const resultNonce = Uint8Array.from(resultEvent.nonce);
    const decryptedPnl = cipher.decrypt(
      [resultEvent.encryptedResult],
      resultNonce
    )[0];

    return { computationOffset, decryptedPnl };
  }, [publicKey, program]);

  return { submitPrivateOrder };
}
