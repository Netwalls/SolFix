import "@/styles/globals.css";
import type { AppProps } from "next/app";

export default function App({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
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
