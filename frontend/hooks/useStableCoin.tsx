import { useContext } from 'react';
import { WalletContext } from '../context/WalletContext';
import { PublicKey } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';

export const useStableCoin = () => {
  const { program, wallet } = useContext(WalletContext);

  const initializeStablecoin = async (params: {
    name: string;
    symbol: string;
    targetCurrency: string;
    iconUri: string;
  }) => {
    if (!program || !wallet) return;

    try {
      const mint = anchor.web3.Keypair.generate();
      
      // Derive PDAs
      const [stablecoinConfig] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("stablecoin"),
          wallet.publicKey.toBuffer(),
          Buffer.from(params.symbol)
        ],
        program.programId
      );

      const tx = await program.methods
        .initializeStablecoin({
          name: params.name,
          symbol: params.symbol,
          targetCurrency: params.targetCurrency,
          iconUri: params.iconUri,
        })
        .accounts({
          authority: wallet.publicKey,
          stablecoinConfig,
          mint: mint.publicKey,
          // Add other required accounts
        })
        .signers([mint])
        .rpc();

      return tx;
    } catch (error) {
      console.error('Error initializing stablecoin:', error);
      throw error;
    }
  };

  const mintTokens = async (amount: number, stablecoinConfig: PublicKey) => {
    if (!program || !wallet) return;

    try {
      const tx = await program.methods
        .mintTokens(new anchor.BN(amount))
        .accounts({
          user: wallet.publicKey,
          stablecoinConfig,
          // Add other required accounts
        })
        .rpc();

      return tx;
    } catch (error) {
      console.error('Error minting tokens:', error);
      throw error;
    }
  };

  return {
    initializeStablecoin,
    mintTokens,
  };
};
