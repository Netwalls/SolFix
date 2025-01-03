// import { useMemo } from 'react';
// import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
// import { PhantomWalletAdapter, SolletWalletAdapter } from '@solana/wallet-adapter-wallets';
// import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';

// const WalletContext: React.FC = ({ children }) => {
//   const network = 'devnet'; // Or 'mainnet-beta' for mainnet

//   const wallets = useMemo(() => [
//     new PhantomWalletAdapter(),
//     new SolletWalletAdapter({ network })
//   ], [network]);

//   return (
//     <ConnectionProvider endpoint={`https://api.${network}.solana.com`}>
//       <WalletProvider wallets={wallets} autoConnect>
//         <WalletModalProvider>
//           {children}
//         </WalletModalProvider>
//       </WalletProvider>
//     </ConnectionProvider>
//   );
// };

// export default WalletContext;
