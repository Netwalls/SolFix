type StablecoinProps = {
    coin: {
      name: string;
      symbol: string;
      icon: string;
      targetCurrency: string;
      amount: number;
    };
  };
  
  const StablecoinCard = ({ coin }: StablecoinProps) => {
    return (
      <div className="bg-gray-800 p-6 rounded-xl shadow-md text-white">
        <div className="flex justify-between items-center">
          <div className="flex items-center space-x-3">
            {coin.icon && <img src={coin.icon} alt={coin.name} className="w-10 h-10 rounded-full" />}
            <span className="text-xl font-semibold">{coin.name}</span>
          </div>
          <span className="text-lg">{coin.symbol}</span>
        </div>
        <div className="mt-4 text-sm text-gray-400">
          <p>Target Currency: {coin.targetCurrency}</p>
          <p>Amount Minted: {coin.amount}</p>
        </div>
      </div>
    );
  };
  
  export default StablecoinCard;
  