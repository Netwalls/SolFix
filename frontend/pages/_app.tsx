import "@/styles/globals.css";
import type { AppProps } from "next/app";
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import { clusterApiUrl } from '@solana/web3.js';
import { useMemo } from 'react';
import { WalletProvider as CustomWalletProvider } from '../context/WalletContext';

require('@solana/wallet-adapter-react-ui/styles.css');

export default function App({ Component, pageProps }: AppProps) {
  const network = WalletAdapterNetwork.Devnet;
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);
  const wallets = useMemo(() => [new PhantomWalletAdapter()], []);

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <CustomWalletProvider>
            <Component {...pageProps} />
          </CustomWalletProvider>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}


// import '../styles/globals.css';
// import WalletContext from '../context/wallet';

// function MyApp({ Component, pageProps }: { Component: React.ComponentType; pageProps: any }) {
//   return (
//     <WalletContext>
//       <Component {...pageProps} />
//     </WalletContext>
//   );
// }

// export default MyApp;
