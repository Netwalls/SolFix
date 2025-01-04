import { useState } from "react";
import { FaPlus, FaEye, FaRegArrowAltCircleRight } from "react-icons/fa";
import StablecoinCard from "../context/StablecoinCard"; // Assuming you have a card component to display stablecoins
import { useStableCoin } from '../hooks/useStableCoin';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

const Home = () => {
  const { initializeStablecoin, mintTokens } = useStableCoin();
  const [stablecoins, setStablecoins] = useState<any[]>([]);
  const [mintingAmount, setMintingAmount] = useState(0);
  const [stablecoinName, setStablecoinName] = useState("");
  const [stablecoinSymbol, setStablecoinSymbol] = useState("");
  const [stablecoinIcon, setStablecoinIcon] = useState("");
  const [targetCurrency, setTargetCurrency] = useState("");
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [showViewCoins, setShowViewCoins] = useState(false);

  // Handle minting stablecoins
  const handleMintStablecoin = async () => {
    try {
      const tx = await initializeStablecoin({
        name: stablecoinName,
        symbol: stablecoinSymbol,
        targetCurrency,
        iconUri: stablecoinIcon,
      });
      
      console.log('Transaction signature:', tx);
      // Add success notification
    } catch (error) {
      console.error('Error:', error);
      // Add error notification
    }
  };

  // Handle minting more stablecoins for an existing stablecoin
  const handleMintMoreStablecoin = (index: number) => {
    if (!mintingAmount) return;
    const updatedStablecoins = [...stablecoins];
    updatedStablecoins[index].totalSupply += mintingAmount;
    setStablecoins(updatedStablecoins);
    setMintingAmount(0);
  };

  // Handle redeeming stablecoins
  const handleRedeemStablecoin = (index: number) => {
    if (!mintingAmount || mintingAmount > stablecoins[index].totalSupply) {
      alert("Invalid amount or insufficient stablecoin supply.");
      return;
    }

    const updatedStablecoins = [...stablecoins];
    updatedStablecoins[index].totalSupply -= mintingAmount;
    setStablecoins(updatedStablecoins);
    setMintingAmount(0);
  };

  const clearForm = () => {
    setMintingAmount(0);
    setStablecoinName("");
    setStablecoinSymbol("");
    setStablecoinIcon("");
    setTargetCurrency("");
  };

  return (
    <div className="min-h-screen flex flex-col justify-center items-center py-10 bg-gradient-to-r from-indigo-600 via-purple-600 to-pink-500">
      {/* Add wallet button */}
      <div className="mb-6">
        <WalletMultiButton />
      </div>

      {/* Web3 Banner */}
      <div className="w-full text-center p-5 mb-10">
        <h1 className="text-5xl font-extrabold text-white mb-4">
          Create and Manage Your Own Stablecoins on Sol-Fix
        </h1>
        <p className="text-lg text-white max-w-lg mx-auto mb-4">
          Empower your crypto journey by creating personalized stablecoins
          pegged to real-world currencies.
        </p>
      </div>

      {/* Main Buttons */}
      <div className="mb-10 w-full grid sm:grid-cols-1 md:grid-cols-1 lg:flex lg:space-x-4 justify-center items-center">
        <button
          onClick={() => setShowCreateForm(true)}
          className="bg-teal-600 text-white py-3 px-6 rounded-lg shadow-lg hover:bg-teal-700 transition duration-300 flex items-center space-x-2 mb-5 lg:mb-0"
        >
          <FaPlus className="text-xl" />
          <span>Create Stablecoin</span>
        </button>
        <button
          onClick={() => setShowViewCoins(true)}
          className="bg-blue-600 text-white py-3 px-6 rounded-lg shadow-lg hover:bg-blue-700 transition duration-300 flex items-center space-x-2"
        >
          <FaEye className="text-xl" />
          <span>View Stablecoins</span>
        </button>
      </div>

      {/* Create Stablecoin Form */}
      {showCreateForm && (
        <div className="bg-gray-800 p-8 rounded-xl shadow-xl w-full max-w-lg">
          <h2 className="text-3xl font-semibold mb-6">Create Your Stablecoin</h2>
          <input
            type="text"
            value={stablecoinName}
            onChange={(e) => setStablecoinName(e.target.value)}
            className="w-full p-3 border border-gray-700 rounded-lg mb-4 text-white bg-gray-900 placeholder-gray-400"
            placeholder="Stablecoin Name"
          />
          <input
            type="text"
            value={stablecoinSymbol}
            onChange={(e) => setStablecoinSymbol(e.target.value)}
            className="w-full p-3 border border-gray-700 rounded-lg mb-4 text-white bg-gray-900 placeholder-gray-400"
            placeholder="Symbol (e.g., USD)"
          />
          <input
            type="text"
            value={stablecoinIcon}
            onChange={(e) => setStablecoinIcon(e.target.value)}
            className="w-full p-3 border border-gray-700 rounded-lg mb-4 text-white bg-gray-900 placeholder-gray-400"
            placeholder="Icon URL (Optional)"
          />
          <input
            type="text"
            value={targetCurrency}
            onChange={(e) => setTargetCurrency(e.target.value)}
            className="w-full p-3 border border-gray-700 rounded-lg mb-4 text-white bg-gray-900 placeholder-gray-400"
            placeholder="Target Currency (e.g., USD)"
          />
          <input
            type="number"
            value={mintingAmount}
            onChange={(e) => setMintingAmount(Number(e.target.value))}
            className="w-full p-3 border border-gray-700 rounded-lg mb-4 text-white bg-gray-900 placeholder-gray-400"
            placeholder="Minting Amount"
          />
          <button
            onClick={handleMintStablecoin}
            className="w-full bg-teal-600 text-white py-3 rounded-lg mt-4 hover:bg-teal-700 transition duration-300"
          >
            Mint Stablecoin{" "}
            <FaRegArrowAltCircleRight className="inline ml-2 text-xl" />
          </button>
          <button
            onClick={() => setShowCreateForm(false)}
            className="w-full bg-red-600 text-white py-3 rounded-lg mt-4 hover:bg-red-700 transition duration-300"
          >
            Close Form
          </button>
        </div>
      )}

      {/* View Stablecoins */}
      {showViewCoins && stablecoins.length > 0 && (
        <div className="space-y-6 mt-10">
          <h2 className="text-2xl font-semibold">Your Minted Stablecoins</h2>
          {stablecoins.map((coin, idx) => (
            <div key={idx} className="bg-gray-800 p-6 rounded-xl shadow-xl">
              <StablecoinCard coin={coin} />
              <div className="flex justify-between mt-4">
                <button
                  onClick={() => handleMintMoreStablecoin(idx)}
                  className="bg-green-600 text-white py-2 px-4 rounded-lg hover:bg-green-700"
                >
                  Mint More
                </button>
                <button
                  onClick={() => handleRedeemStablecoin(idx)}
                  className="bg-red-600 text-white py-2 px-4 rounded-lg hover:bg-red-700"
                >
                  Redeem
                </button>
              </div>
            </div>
          ))}
          <button
            onClick={() => setShowViewCoins(false)}
            className="w-full bg-gray-600 text-white py-3 rounded-lg mt-4 hover:bg-gray-700 transition duration-300"
          >
            Close View
          </button>
        </div>
      )}

      {/* Display Message if No Stablecoins */}
      {showViewCoins && stablecoins.length === 0 && (
        <div className="text-center text-white mt-10">
          <p className="text-lg">
            You haven't created any stablecoins yet. Click on "Create
            Stablecoin" to start!
          </p>
        </div>
      )}
    </div>
  );
};

export default Home;
