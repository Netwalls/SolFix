import { createContext, useContext, useEffect, useState } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';
import { Program, AnchorProvider, web3 } from '@coral-xyz/anchor';
import { useAnchorWallet, useConnection } from '@solana/wallet-adapter-react';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { IDL } from '../idl/stable_fun'; // We'll generate this from your anchor build

type WalletContextType = {
  program: Program | null;
  wallet: AnchorWallet | null;
}

export const WalletContext = createContext<WalletContextType>({
  program: null,
  wallet: null
});

export function WalletProvider({ children }) {
  const wallet = useAnchorWallet();
  const { connection } = useConnection();
  const [program, setProgram] = useState(null);

  useEffect(() => {
    if (wallet) {
      const provider = new AnchorProvider(
        connection,
        wallet,
        AnchorProvider.defaultOptions()
      );
      
      const program = new Program(
        IDL,
        new PublicKey("your_program_id_here"), // Replace with your deployed program ID
        provider
      );
      
      setProgram(program);
    }
  }, [wallet, connection]);

  return (
    <WalletContext.Provider value={{ program, wallet, connection }}>
      {children}
    </WalletContext.Provider>
  );
}
